use inkwell::values::PointerValue;

use crate::lexer::types::DATATYPE;

use super::codegen::{CodeGen, VariableStore};

impl<'ctx> CodeGen<'ctx> {
    pub fn store_var(
        &self,
        fn_name: &str,
        var_name: &str,
        dt: &DATATYPE,
        is_mut: bool,
    ) -> PointerValue<'ctx> {
        let mut vars = self.variables.borrow_mut();

        let var = vars.iter_mut().find(|x| x.name == fn_name).unwrap();

        let ptr = self
            .builder
            .build_alloca(self.def_expr(dt), var_name)
            .unwrap();

        var.vars.insert(
            var_name.to_string(),
            VariableStore {
                ptr,
                is_mutable: is_mut,
                datatype: dt.clone(),
            },
        );

        ptr
    }
}
