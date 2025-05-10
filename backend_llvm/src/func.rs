use inkwell::{
    module::Linkage,
    types::{BasicMetadataTypeEnum, BasicType},
    values::{BasicValueEnum, FunctionValue, InstructionValue},
};
use lexer::types::Datatype;
use parser::nodes::{self, ASTNodes, Return};

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

    pub(crate) fn impl_import_call(
        &self,
        built_func: FunctionValue<'ctx>,
        call: &nodes::ImportCall,
    ) -> Result<Option<BasicValueEnum<'ctx>>, CodeGenError> {
        let path = &call.path.join("__");

        let func_attrs = self
            .import_resolver
            .get_extern_function(&path, call.path.first().unwrap());
        if func_attrs.is_none() {
            return Err(CodeGenError::new("Import could not be resolved"));
        }
        let (func_attrs, path) = func_attrs.unwrap();

        let func = if let Some(func) = self.module.get_function(&path) {
            func
        } else {
            let func = self
                .module
                .add_function(&path, func_attrs.func, Some(Linkage::External));
            self.execution_engine
                .as_ref()
                .map(|exec| exec.add_global_mapping(&func, func_attrs.ptr));
            func
        };

        match &*call.ident {
            ASTNodes::FunctionCall(func_call) => {
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
            _ => todo!(),
        }
    }

    pub(crate) fn impl_method_call(
        &self,
        built_func: FunctionValue<'ctx>,
        method: &nodes::Method,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        let callee = self.resolve_var(built_func, &*method.parent)?;
        self.import_resolver
            .get_builtin_function(callee.type_, &method.func.name)
    }

    pub(crate) fn impl_extern_call(
        &self,
        ext: &nodes::Extern,
    ) -> Result<FunctionValue<'ctx>, CodeGenError> {
        let args = self.impl_function_args(&ext.args)?;

        let func_type = if let Some(rt) = &ext.return_type {
            let llvm_rt = self.parser_to_llvm_dt(&rt);
            llvm_rt.fn_type(&args, false)
        } else {
            self.context.void_type().fn_type(&args, false)
        };

        let built_func = self
            .module
            .add_function(&ext.name, func_type, Some(Linkage::External));

        for (index, arg) in built_func.get_param_iter().enumerate() {
            arg.set_name(&ext.args[index].0);
        }

        Ok(built_func)
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
        let data = r#"
func add(a u32, b u32) u32 { return a + b }
func main() u32 {
    let u32 a = add(5, 7) 
    return a
}"#;
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

    #[test]
    fn test_imported_func_call() {
        let data = r#"
func main() u32 {
    let string s = "Test"
    std::io::println(s)
    return 0
}"#;

        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %0 = alloca [4 x i8], align 1
  store [4 x i8] c"Test", ptr %0, align 1
  %1 = alloca { i64, ptr }, align 8
  %2 = insertvalue { i64, ptr } { i64 4, ptr undef }, ptr %0, 1
  store { i64, ptr } %2, ptr %1, align 8
  call void @__std__io__println(ptr %1)
  ret i32 0
}

declare void @__std__io__println(ptr)
"#
        )
    }

    #[test]
    fn test_extern_func_call() {
        let data = r#"

extern func add(a u32, b u32) u32

func main() u32 {
    return 0
}"#;

        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

declare i32 @add(i32, i32)

define i32 @main() {
entry:
  ret i32 0
}
"#
        )
    }
}
