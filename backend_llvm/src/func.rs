use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType},
    values::{BasicValueEnum, FunctionValue, InstructionValue},
};
use lexer::types::Datatype;
use new_parser::nodes::{self, Return};

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_function_def(
        &self,
        func: &nodes::Function,
    ) -> Result<FunctionValue<'ctx>, CodeGenError> {
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

        self.codegen_function_block(&func.body, built_func)?;

        Ok(built_func)
    }

    fn impl_function_args(
        &self,
        args: &Vec<(String, Datatype)>,
    ) -> Result<Vec<BasicMetadataTypeEnum<'ctx>>, CodeGenError> {
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
    ) -> Result<InstructionValue<'ctx>, CodeGenError> {
        if let Some(expr) = &ret.value {
            let ret_val = self.impl_expr(
                expr,
                built_func,
                built_func.get_type().get_return_type().unwrap(),
            )?;
            Ok(self.builder.build_return(Some(&ret_val)).unwrap())
        } else {
            Ok(self.builder.build_return(None).unwrap())
        }
    }

    pub(crate) fn impl_function_call(
        &self,
        built_func: FunctionValue<'ctx>,
        func_call: &nodes::FunctionCall,
    ) -> Result<Option<BasicValueEnum<'ctx>>, CodeGenError> {
        let func = self
            .module
            .get_function(&func_call.name)
            .ok_or(CodeGenError::new("Function not found"))?;
        let mut args = vec![];
        let params = func.get_type().get_param_types();
        for (i, arg) in func_call.args.iter().enumerate() {
            let param = params.get(i).ok_or(CodeGenError::new("Invalid arg"))?;
            let arg = self.impl_expr(arg, built_func, *param)?;
            args.push(arg.into());
        }
        let ret_val = self
            .builder
            .build_call(func, &args, "")
            .map_err(CodeGenError::from_llvm_err)?;
        Ok(ret_val.try_as_basic_value().left())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_codegen_function_def() {
        let data = "func main() i32 { return 0 }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  ret i32 0
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
  ret void
}
"#
        )
    }

    #[test]
    fn test_codegen_function_def_with_args() {
        let data = "func main(a u32, b u32) {}";
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

    #[test]
    fn test_codegen_function_call() {
        let data = "
func add(a u32, b u32) u32 { return a + b }
func main() u32 {
    add(5, 7) 
    return 0
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @add(i32 %a, i32 %b) {
entry:
  %0 = add i32 %a, %b
  ret i32 %0
}

define i32 @main() {
entry:
  %0 = call i32 @add(i32 5, i32 7)
  ret i32 0
}
"#
        )
    }

    #[test]
    fn test_codegen_function_call_as_expr() {
        let data = "
func add(a u32, b u32) u32 { return a + b }
func main() u32 {
    let u32 a = add(5, 7) 
    return a
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @add(i32 %a, i32 %b) {
entry:
  %0 = add i32 %a, %b
  ret i32 %0
}

define i32 @main() {
entry:
  %0 = call i32 @add(i32 5, i32 7)
  %a = alloca i32, align 4
  store i32 %0, ptr %a, align 4
  %a1 = load i32, ptr %a, align 4
  ret i32 %a1
}
"#
        )
    }
}
