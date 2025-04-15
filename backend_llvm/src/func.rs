use inkwell::types::BasicType;
use new_parser::nodes;

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_function_def(&self, func: &nodes::Function) -> Result<(), ()> {
        let return_type = self.parser_to_llvm_dt(&func.return_type);
        let func_type = return_type.fn_type(&[], false);
        self.module.add_function(&func.name, func_type, None);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_codegen_function_def() {
        let data = "func main() i32 { }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

declare i32 @main()
"#
        )
    }
}
