use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType},
    values::{FunctionValue, InstructionValue},
};
use lexer::types::Datatype;
use new_parser::nodes::{self, Return};

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_function_def(&self, func: &nodes::Function) -> Result<(), ()> {
        // Function parameters
        let args = self.impl_function_args(&func.args)?;

        // Function return type
        let func_type = if let Some(rt) = &func.return_type {
            let llvm_rt = self.parser_to_llvm_dt(&rt);
            llvm_rt.fn_type(&args, false)
        } else {
            self.context.void_type().fn_type(&args, false)
        };

        // Build the function definition
        let built_func = self.module.add_function(&func.name, func_type, None);

        // Set function parameters names
        for (index, arg) in built_func.get_param_iter().enumerate() {
            arg.set_name(&func.args[index].0);
        }

        self.codegen_block(&func.body, built_func)?;

        Ok(())
    }

    fn impl_function_args(
        &self,
        args: &Vec<(String, Datatype)>,
    ) -> Result<Vec<BasicMetadataTypeEnum<'ctx>>, ()> {
        let mut res_args = vec![];
        for (_, dt) in args {
            let llvm_dt = self.parser_to_llvm_dt(&dt);
            res_args.push(llvm_dt.into());
        }
        Ok(res_args)
    }

    pub(crate) fn impl_function_return(
        &self,
        built_func: FunctionValue<'ctx>,
        ret: &Return,
    ) -> Result<InstructionValue<'ctx>, ()> {
        if let Some(expr) = &ret.value {
            let ret_val = self.impl_expr(expr, built_func.get_type().get_return_type().unwrap())?;
            Ok(self.builder.build_return(Some(&ret_val)).unwrap())
        } else {
            Ok(self.builder.build_return(None).unwrap())
        }
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

define i32 @main() {
entry:
}
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

define void @main() {
entry:
}
"#
        )
    }

    #[test]
    fn test_codegen_function_def_with_args() {
        let data = "func main(a u32, b u32) u32 {}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %a, i32 %b) {
entry:
}
"#
        )
    }

    #[test]
    fn test_codegen_function_def_with_ret() {
        let data = "func main(a u32, b u32) { return }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define void @main(i32 %a, i32 %b) {
entry:
  ret void
}
"#
        )
    }

    #[test]
    fn test_codegen_function_def_with_ret_type() {
        let data = "func main(a u32, b u32) i32 { return 5 + 4 }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main(i32 %a, i32 %b) {
entry:
  ret i32 9
}
"#
        )
    }
}
