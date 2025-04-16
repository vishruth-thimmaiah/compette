use inkwell::values::InstructionValue;
use new_parser::nodes;

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_let_stmt(
        &self,
        stmt: &nodes::LetStmt,
    ) -> Result<InstructionValue<'ctx>, ()> {
        let dt = self.parser_to_llvm_dt(&stmt.datatype);
        let expr = self.impl_expr(&stmt.value, dt)?;

        let ptr = self.builder.build_alloca(dt, &stmt.name).map_err(|_| ())?;

        self.builder.build_store(ptr, expr).map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_codegen_let_stmt() {
        let data = "func main() { let u32 a = 5 }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define void @main() {
entry:
  %a = alloca i32, align 4
  store i32 5, ptr %a, align 4
}
"#
        )
    }
}
