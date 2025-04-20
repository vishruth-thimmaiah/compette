use std::{cell::RefCell, collections::HashMap};

use inkwell::{
    context,
    types::{BasicTypeEnum, FunctionType},
    values::BasicValueEnum,
};
use new_parser::nodes::ImportDef;

use crate::CodeGenError;

#[derive(Debug)]
pub struct Resolver<'ctx> {
    context: &'ctx context::Context,
    resolved_paths: RefCell<HashMap<String, String>>,
}
#[derive(Debug)]
pub struct StdLibFunc<'ctx> {
    pub ptr: usize,
    pub func: FunctionType<'ctx>,
}

impl<'ctx> Resolver<'ctx> {
    pub fn new(context: &'ctx context::Context) -> Self {
        Self {
            context,
            resolved_paths: RefCell::new(HashMap::new()),
        }
    }

    fn get_stdlib_function(&self, name: &str) -> Option<StdLibFunc<'ctx>> {
        let func = match name {
            "__std__io__print" => StdLibFunc {
                ptr: stdlib::io::__std__io__print as usize,
                func: self
                    .context
                    .void_type()
                    .fn_type(&[self.context.i8_type().into()], false),
            },
            "__std__io__println" => StdLibFunc {
                ptr: stdlib::io::__std__io__println as usize,
                func: self
                    .context
                    .void_type()
                    .fn_type(&[self.context.i8_type().into()], false),
            },
            // Temporary funtion until format print is implemented
            "__std__io__printint" => StdLibFunc {
                ptr: stdlib::io::__std__io__printint as usize,
                func: self
                    .context
                    .void_type()
                    .fn_type(&[self.context.i64_type().into()], false),
            },
            "__std__io__printflt" => StdLibFunc {
                ptr: stdlib::io::__std__io__printflt as usize,
                func: self
                    .context
                    .void_type()
                    .fn_type(&[self.context.f64_type().into()], false),
            },
            _ => return None,
        };
        Some(func)
    }

    pub(crate) fn get_builtin_function(
        &self,
        callee_type: BasicTypeEnum<'ctx>,
        name: &str,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        return match callee_type {
            BasicTypeEnum::ArrayType(arr) => match name {
                "len" => Ok(self
                    .context
                    .i64_type()
                    .const_int(arr.len() as u64, false)
                    .into()),
                _ => todo!(),
            },

            _ => todo!(),
        };
    }

    pub(crate) fn resolve_import_def(&self, path: &ImportDef) -> Result<(), CodeGenError> {
        let k = path.path.last().unwrap().to_string();
        let v = path
            .path
            .iter()
            .take(path.path.len() - 1)
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("__");
        self.resolved_paths.borrow_mut().insert(k, v);
        Ok(())
    }

    pub(crate) fn get_extern_function(
        &self,
        path: &str,
        last: &str,
    ) -> Option<(StdLibFunc<'ctx>, String)> {
        let path = if let Some(ext) = self.resolved_paths.borrow().get(last) {
            "__".to_string() + ext + "__" + path
        } else {
            "__".to_string() + path
        };
        if path.starts_with("__std__") {
            self.get_stdlib_function(&path).map(|v| (v, path))
        } else {
            todo!()
        }
    }
}
