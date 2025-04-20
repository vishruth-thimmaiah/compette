use inkwell::{basic_block::BasicBlock, values::FunctionValue};
use new_parser::nodes::{ASTNodes, Block};

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn codegen_block(
        &self,
        block: &Block,
        built_func: FunctionValue<'ctx>,
        basic_block: BasicBlock<'ctx>,
    ) -> Result<BasicBlock, CodeGenError> {
        self.builder.position_at_end(basic_block);

        for node in &block.body {
            match node {
                ASTNodes::LetStmt(let_stmt) => {
                    self.impl_let_stmt(built_func, let_stmt)?;
                }
                ASTNodes::Conditional(cond) => {
                    self.impl_if_stmt(built_func, cond)?;
                }
                ASTNodes::Loop(loop_stmt) => {
                    self.impl_loop_stmt(built_func, loop_stmt)?;
                }
                ASTNodes::ForLoop(loop_stmt) => {
                    self.impl_for_loop_stmt(built_func, loop_stmt)?;
                }
                ASTNodes::Return(ret) => {
                    self.impl_function_return(built_func, ret)?;
                }
                ASTNodes::AssignStmt(stmt) => {
                    self.impl_assign_stmt(built_func, stmt)?;
                }
                ASTNodes::FunctionCall(call) => {
                    self.impl_function_call(built_func, call)?;
                }
                ASTNodes::ImportCall(call) => {
                    self.impl_import_call(built_func, call)?;
                }
                _ => todo!(),
            };
        }
        Ok(basic_block)
    }

    pub(crate) fn codegen_function_block(
        &self,
        block: &Block,
        built_func: FunctionValue<'ctx>,
    ) -> Result<(), CodeGenError> {
        let basic_block = self.context.append_basic_block(built_func, "entry");
        self.codegen_block(block, built_func, basic_block)?;
        self.var_ptrs.clear();

        if basic_block.get_terminator().is_none() {
            if built_func.get_type().get_return_type().is_none() {
                self.builder.build_return(None).unwrap();
            } else {
                return Err(CodeGenError::new("Missing return statement"));
            }
        }
        Ok(())
    }
}
