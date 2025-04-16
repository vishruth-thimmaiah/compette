use inkwell::values::FunctionValue;
use new_parser::nodes::{ASTNodes, Block};

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn codegen_block(
        &self,
        block: &Block,
        built_func: FunctionValue<'ctx>,
    ) -> Result<(), ()> {
        let basic_block = self.context.append_basic_block(built_func, "entry");
        self.builder.position_at_end(basic_block);

        for node in &block.body {
            match node {
                ASTNodes::LetStmt(let_stmt) => self.impl_let_stmt(let_stmt)?,
                ASTNodes::Return(ret) => self.impl_function_return(built_func, ret)?,
                _ => todo!(),
            };
        }

        Ok(())
    }
}
