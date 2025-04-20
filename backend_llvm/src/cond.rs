use inkwell::{basic_block::BasicBlock, values::FunctionValue};
use new_parser::nodes::{self, Conditional};

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_if_stmt(
        &self,
        built_func: FunctionValue<'ctx>,
        mut stmt: &nodes::Conditional,
        next_block: Option<BasicBlock<'ctx>>,
    ) -> Result<(), CodeGenError> {
        while let Conditional::If {
            condition,
            body,
            else_body,
        } = stmt
        {
            let then_block = self.context.append_basic_block(built_func, "then");
            let then_cond =
                self.impl_expr(&condition, built_func, self.context.bool_type().into())?;
            let else_block = self.context.append_basic_block(built_func, "else");

            self.builder
                .build_conditional_branch(then_cond.into_int_value(), then_block, else_block)
                .map_err(CodeGenError::from_llvm_err)?;

            self.codegen_block(body, built_func, then_block, next_block)?;

            if let Some(else_body) = else_body {
                if let Conditional::Else { body } = &**else_body {
                    self.codegen_block(body, built_func, else_block, next_block)?;
                    break;
                }
                stmt = else_body;
                self.builder.position_at_end(else_block);
            } else {
                if then_block.get_terminator().is_none() {
                    self.builder.build_unconditional_branch(else_block).unwrap();
                }
                self.builder.position_at_end(else_block);
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_impl_if_stmt_with_else() {
        let data = "func main() { if true { return } else { return } }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define void @main() {
entry:
  br i1 true, label %then, label %else

then:                                             ; preds = %entry
  ret void

else:                                             ; preds = %entry
  ret void
}
"#
        )
    }

    #[test]
    fn test_impl_if_stmt_with_else_if_else() {
        let data = "func main() i32 { if 1 > 2 { return 0 } else if 1 < 2 { return 1 } else { return 2 } }";
        let result = crate::get_codegen_for_string(data).unwrap();
        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  br i1 false, label %then, label %else

then:                                             ; preds = %entry
  ret i32 0

else:                                             ; preds = %entry
  br i1 true, label %then1, label %else2

then1:                                            ; preds = %else
  ret i32 1

else2:                                            ; preds = %else
  ret i32 2
}
"#
        )
    }

    #[test]
    fn test_impl_if_stmt() {
        let data = "func main() u32 { if true { return 1 } return 2 }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  br i1 true, label %then, label %else

then:                                             ; preds = %entry
  ret i32 1

else:                                             ; preds = %entry
  ret i32 2
}
"#
        )
    }

    #[test]
    fn test_impl_if_stmt_with_else_if() {
        let data = r#"
func main() u32 {
    if true {
        return 1
    } else if false {
        return 2
    } else if true {
        return 3
    }
    return 4
}"#;
        let result = crate::get_codegen_for_string(&data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  br i1 true, label %then, label %else

then:                                             ; preds = %entry
  ret i32 1

else:                                             ; preds = %entry
  br i1 false, label %then1, label %else2

then1:                                            ; preds = %else
  ret i32 2

else2:                                            ; preds = %else
  br i1 true, label %then3, label %else4

then3:                                            ; preds = %else2
  ret i32 3

else4:                                            ; preds = %else2
  ret i32 4
}
"#
        )
    }
}
