use inkwell::types::BasicType;
use new_parser::nodes;

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_function_def(&self, func: &nodes::Function) -> Result<(), ()> {

        let args = vec![];

        let func_type = if let Some(rt) = &func.return_type {
            let llvm_rt = self.parser_to_llvm_dt(&rt);
            llvm_rt.fn_type(&args, false)
        } else {
            self.context.void_type().fn_type(&args, false)
        };
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

    #[test]
    fn test_codegen_function_def_no_ret() {
        let data = "func main() { }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

declare void @main()
"#
        )
    }
}
