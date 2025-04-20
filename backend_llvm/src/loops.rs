use inkwell::{basic_block::BasicBlock, values::FunctionValue};
use lexer::types::Operator;
use parser::nodes::{self, ASTNodes, Expression};

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_for_loop_stmt(
        &self,
        built_func: FunctionValue<'ctx>,
        stmt: &nodes::ForLoop,
    ) -> Result<(), CodeGenError> {
        let for_init_block = self.context.append_basic_block(built_func, "for_init");
        let for_body_block = self.context.append_basic_block(built_func, "for_body");
        let for_cond_block = self.context.append_basic_block(built_func, "for_cond");
        let cont = self.context.append_basic_block(built_func, "for_cont");

        let index_ptr = self
            .builder
            .build_alloca(self.context.i64_type(), &stmt.increment.name)
            .map_err(CodeGenError::from_llvm_err)?;
        self.builder
            .build_store(index_ptr, self.context.i64_type().const_zero())
            .map_err(CodeGenError::from_llvm_err)?;

        self.var_ptrs.insert(
            &stmt.increment.name,
            index_ptr,
            self.context.i64_type().into(),
            false,
        );

        self.builder
            .build_unconditional_branch(for_init_block)
            .unwrap();
        self.builder.position_at_end(for_init_block);

        // For now, we only support using a variable as the iterator
        let iter = if let Expression::Simple { left, .. } = &stmt.iterator {
            if let ASTNodes::Variable(var) = left.as_ref() {
                self.var_ptrs.get(&var.name).unwrap()
            } else {
                unimplemented!()
            }
        } else {
            unimplemented!()
        };

        let iter_type = iter.type_.into_array_type();
        let iter_inner_type = iter_type.get_element_type();

        let index = self
            .builder
            .build_load(self.context.i64_type(), index_ptr, "")
            .unwrap();

        let is_avail = self.comp_binary_operation(
            &Operator::LESSER,
            &index,
            &self
                .context
                .i64_type()
                .const_int(iter_type.len() as u64, false)
                .into(),
        )?;

        let value_instance = unsafe {
            self.builder
                .build_in_bounds_gep(
                    iter_type,
                    iter.ptr,
                    &[self.context.i32_type().const_zero(), index.into_int_value()],
                    "",
                )
                .map_err(CodeGenError::from_llvm_err)
        }?;

        self.var_ptrs.insert(
            &stmt.value.name,
            value_instance,
            iter_inner_type.into(),
            false,
        );

        self.builder
            .build_conditional_branch(is_avail, for_body_block, cont)
            .unwrap();

        self.codegen_block(&stmt.body, built_func, for_body_block, Some(cont))?;

        self.builder
            .build_unconditional_branch(for_cond_block)
            .map_err(CodeGenError::from_llvm_err)?;

        self.builder.position_at_end(for_cond_block);

        let index = self
            .builder
            .build_load(self.context.i64_type(), index_ptr, "")
            .map_err(CodeGenError::from_llvm_err)?;

        let new_index = self
            .builder
            .build_int_add(
                index.into_int_value(),
                self.context.i64_type().const_int(1, false),
                "",
            )
            .map_err(CodeGenError::from_llvm_err)?;

        self.builder
            .build_store(index_ptr, new_index)
            .map_err(CodeGenError::from_llvm_err)?;

        self.builder
            .build_unconditional_branch(for_init_block)
            .map_err(CodeGenError::from_llvm_err)?;

        self.builder.position_at_end(cont);
        Ok(())
    }

    pub(crate) fn impl_loop_stmt(
        &self,
        built_func: FunctionValue<'ctx>,
        stmt: &nodes::Loop,
    ) -> Result<(), CodeGenError> {
        let loop_block = self.context.append_basic_block(built_func, "loop");
        let cont = self.context.append_basic_block(built_func, "loop_cont");
        if stmt.condition.is_some() {
            let loop_init = self.context.prepend_basic_block(loop_block, "loop_init");
            self.builder
                .build_unconditional_branch(loop_init)
                .map_err(CodeGenError::from_llvm_err)?;
            self.builder.position_at_end(loop_init);
            let expr = self.impl_expr(
                stmt.condition.as_ref().unwrap(),
                built_func,
                self.context.bool_type().into(),
            )?;

            self.builder
                .build_conditional_branch(expr.into_int_value(), loop_block, cont)
                .map_err(CodeGenError::from_llvm_err)?;

            self.codegen_block(&stmt.body, built_func, loop_block, Some(cont))?;
            self.builder
                .build_unconditional_branch(loop_init)
                .map_err(CodeGenError::from_llvm_err)?;
        } else {
            self.builder.build_unconditional_branch(loop_block).unwrap();
            self.codegen_block(&stmt.body, built_func, loop_block, Some(cont))?;
            self.builder.build_unconditional_branch(loop_block).unwrap();
        }

        self.builder.position_at_end(cont);
        Ok(())
    }

    pub(crate) fn codegen_break_stmt(
        &self,
        _built_func: FunctionValue<'ctx>,
        next_block: Option<BasicBlock<'ctx>>,
    ) -> Result<(), CodeGenError> {
        self.builder
            .build_unconditional_branch(next_block.unwrap())
            .map_err(CodeGenError::from_llvm_err)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_impl_loop_stmt() {
        let data = "func main() u32 { 
let u32! a = 0
loop a < 10 {
    a = a + 1
}
return a
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 0, ptr %a, align 4
  br label %loop_init

loop_init:                                        ; preds = %loop, %entry
  %a1 = load i32, ptr %a, align 4
  %0 = zext i32 %a1 to i64
  %1 = icmp slt i64 %0, 10
  br i1 %1, label %loop, label %loop_cont

loop:                                             ; preds = %loop_init
  %a2 = load i32, ptr %a, align 4
  %2 = add i32 %a2, 1
  store i32 %2, ptr %a, align 4
  br label %loop_init

loop_cont:                                        ; preds = %loop_init
  %a3 = load i32, ptr %a, align 4
  ret i32 %a3
}
"#
        )
    }

    #[test]
    fn test_impl_for_loop_stmt() {
        let data = "func main() u32 { 
    let u64[] array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    let u64! a = 0
    loop range val, index = array {
        a = index * val
    }
    return a -> u32
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %array = alloca [10 x i64], align 8
  store [10 x i64] [i64 0, i64 1, i64 2, i64 3, i64 4, i64 5, i64 6, i64 7, i64 8, i64 9], ptr %array, align 4
  %a = alloca i64, align 8
  store i64 0, ptr %a, align 4
  %index = alloca i64, align 8
  store i64 0, ptr %index, align 4
  br label %for_init

for_init:                                         ; preds = %for_cond, %entry
  %0 = load i64, ptr %index, align 4
  %1 = icmp slt i64 %0, 10
  %2 = getelementptr inbounds [10 x i64], ptr %array, i32 0, i64 %0
  br i1 %1, label %for_body, label %for_cont

for_body:                                         ; preds = %for_init
  %index1 = load i64, ptr %index, align 4
  %val = load i64, ptr %2, align 4
  %3 = mul i64 %index1, %val
  store i64 %3, ptr %a, align 4
  br label %for_cond

for_cond:                                         ; preds = %for_body
  %4 = load i64, ptr %index, align 4
  %5 = add i64 %4, 1
  store i64 %5, ptr %index, align 4
  br label %for_init

for_cont:                                         ; preds = %for_init
  %a2 = load i64, ptr %a, align 4
  %6 = trunc i64 %a2 to i32
  ret i32 %6
}
"#
        )
    }

    #[test]
    fn test_impl_loop_stmt_with_break() {
        let data = "func main() u32 { 
    let u64! a = 0
    loop a < 10 {
        if a == 3 {
            break
        }
        a = a + 1
    }
    return a -> u32
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i64, align 8
  store i64 0, ptr %a, align 4
  br label %loop_init

loop_init:                                        ; preds = %else, %entry
  %a1 = load i64, ptr %a, align 4
  %0 = icmp slt i64 %a1, 10
  br i1 %0, label %loop, label %loop_cont

loop:                                             ; preds = %loop_init
  %a2 = load i64, ptr %a, align 4
  %1 = icmp eq i64 %a2, 3
  br i1 %1, label %then, label %else

loop_cont:                                        ; preds = %then, %loop_init
  %a4 = load i64, ptr %a, align 4
  %2 = trunc i64 %a4 to i32
  ret i32 %2

then:                                             ; preds = %loop
  br label %loop_cont

else:                                             ; preds = %loop
  %a3 = load i64, ptr %a, align 4
  %3 = add i64 %a3, 1
  store i64 %3, ptr %a, align 4
  br label %loop_init
}
"#
        )
    }
}
