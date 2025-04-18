use inkwell::values::FunctionValue;
use new_parser::nodes;

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_if_stmt(
        &self,
        built_func: FunctionValue<'ctx>,
        stmt: &nodes::Conditional,
    ) -> Result<(), CodeGenError> {
        let mut then_cond = self.impl_expr(&stmt.condition, self.context.bool_type().into())?;
        let mut then_block = self.codegen_block(&stmt.body, built_func, "then")?;

        let mut count = 0;

        while let Some((cond, body)) = stmt.get_else_if_for(count) {
            let else_block = self.context.append_basic_block(built_func, "else");
            self.builder.position_at_end(else_block);
            let else_cond = self.impl_expr(&cond, self.context.bool_type().into())?;

            self.builder
                .position_at_end(then_block.get_previous_basic_block().unwrap());
            self.builder
                .build_conditional_branch(then_cond.into_int_value(), then_block, else_block)
                .unwrap();
            then_block = self.codegen_block(body, built_func, "then")?;
            then_cond = else_cond;
            count += 1;
        }

        let else_block = if let Some(else_body) = &stmt.else_body {
            let else_block = self.codegen_block(else_body, built_func, "else")?;

            self.builder
                .position_at_end(then_block.get_previous_basic_block().unwrap());
            self.builder
                .build_conditional_branch(then_cond.into_int_value(), then_block, else_block)
                .unwrap();
            else_block
        } else {
            let destination_block = self.context.append_basic_block(built_func, "else");
            self.builder.position_at_end(destination_block);
            self.builder
                .position_at_end(then_block.get_previous_basic_block().unwrap());
            self.builder
                .build_conditional_branch(then_cond.into_int_value(), then_block, destination_block)
                .unwrap();
            destination_block
        };
        self.builder.position_at_end(else_block);

        return Ok(());
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
    fn test_impl_if_stmt_with_else_if() {
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
}
