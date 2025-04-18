use std::{cell::RefCell, collections::HashMap};

use inkwell::{
    types::BasicTypeEnum,
    values::{InstructionValue, PointerValue},
};
use new_parser::nodes::{self, ASTNodes};

use crate::{CodeGen, CodeGenError};

#[derive(Debug, Default)]
pub struct Variables<'ctx> {
    vars: RefCell<HashMap<String, Variable<'ctx>>>,
}

#[derive(Debug, Clone)]
pub struct Variable<'ctx> {
    pub ptr: PointerValue<'ctx>,
    pub type_: BasicTypeEnum<'ctx>,
    pub mutable: bool,
}

impl<'ctx> Variables<'ctx> {
    pub(crate) fn get(&self, name: &str) -> Option<Variable<'ctx>> {
        let borrow = self.vars.borrow();
        let var = borrow.get(name)?;
        Some(var.clone())
    }

    pub(crate) fn insert(
        &self,
        name: &str,
        ptr: PointerValue<'ctx>,
        type_: BasicTypeEnum<'ctx>,
        mutable: bool,
    ) {
        self.vars.borrow_mut().insert(
            name.to_string(),
            Variable {
                ptr,
                type_,
                mutable,
            },
        );
    }

    pub(crate) fn remove(&self, name: &str) {
        self.vars.borrow_mut().remove(name);
    }

    pub(crate) fn clear(&self) {
        self.vars.borrow_mut().clear();
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_let_stmt(
        &self,
        stmt: &nodes::LetStmt,
    ) -> Result<InstructionValue<'ctx>, CodeGenError> {
        let dt = self.parser_to_llvm_dt(&stmt.datatype);
        let expr = self.impl_expr(&stmt.value, dt)?;

        let ptr = self
            .builder
            .build_alloca(dt, &stmt.name)
            .map_err(CodeGenError::from_llvm_err)?;
        self.var_ptrs.insert(&stmt.name, ptr, dt, stmt.mutable);

        self.builder
            .build_store(ptr, expr)
            .map_err(CodeGenError::from_llvm_err)
    }

    pub(crate) fn impl_assign_stmt(
        &self,
        stmt: &nodes::AssignStmt,
    ) -> Result<InstructionValue, CodeGenError> {
        let var = self.resolve_var(&stmt.name).and_then(|op| {
            op.mutable
                .then_some(op)
                .ok_or(CodeGenError::new("Variable not mutable"))
        })?;
        let expr = self.impl_expr(&stmt.value, var.type_)?;

        self.builder
            .build_store(var.ptr, expr)
            .map_err(CodeGenError::from_llvm_err)
    }

    pub(crate) fn resolve_var(
        &self,
        node: &nodes::ASTNodes,
    ) -> Result<Variable<'ctx>, CodeGenError> {
        match node {
            ASTNodes::Variable(var) => Ok(self
                .var_ptrs
                .get(&var.name)
                .ok_or(CodeGenError::new("Variable not found"))?),
            _ => todo!(),
        }
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
  ret void
}
"#
        )
    }

    #[test]
    fn test_codegen_let_stmt_array() {
        let data = "func main() { let u32[] a = [1, 2, 3, 4, 5] }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define void @main() {
entry:
  %a = alloca [5 x i32], align 4
  store [5 x i32] [i32 1, i32 2, i32 3, i32 4, i32 5], ptr %a, align 4
  ret void
}
"#
        )
    }

    #[test]
    fn test_codegen_update_value() {
        let data = "func main() u32 { 
let u32! a = 5
a = 10 
return a
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 5, ptr %a, align 4
  store i32 10, ptr %a, align 4
  %a1 = load i32, ptr %a, align 4
  ret i32 %a1
}
"#
        )
    }
}
