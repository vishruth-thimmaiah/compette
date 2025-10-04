use std::{cell::RefCell, collections::HashMap};

use inkwell::{
    types::BasicTypeEnum,
    values::{BasicValueEnum, FunctionValue, InstructionValue, PointerValue},
};
use parser::nodes::{self, ASTNodes};

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

    pub(crate) fn _remove(&self, name: &str) {
        self.vars.borrow_mut().remove(name);
    }

    pub(crate) fn clear(&self) {
        self.vars.borrow_mut().clear();
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_let_stmt(
        &self,
        built_func: FunctionValue<'ctx>,
        stmt: &nodes::LetStmt,
    ) -> Result<PointerValue<'ctx>, CodeGenError> {
        let dt = self.parser_to_llvm_dt(&stmt.datatype);
        let expr = self.impl_expr(&stmt.value, built_func, dt)?;

        let ptr = if expr.is_pointer_value() {
            expr.into_pointer_value()
        } else {
            let ptr = self
                .builder
                .build_alloca(dt, &stmt.name)
                .map_err(CodeGenError::from_llvm_err)?;

            self.builder
                .build_store(ptr, expr)
                .map_err(CodeGenError::from_llvm_err)?;

            ptr
        };
        self.var_ptrs.insert(&stmt.name, ptr, dt, stmt.mutable);
        return Ok(ptr);
    }

    pub(crate) fn impl_assign_stmt(
        &self,
        built_func: FunctionValue<'ctx>,
        stmt: &nodes::AssignStmt,
    ) -> Result<InstructionValue, CodeGenError> {
        let var = self.resolve_var(built_func, &stmt.name).and_then(|op| {
            op.mutable
                .then_some(op)
                .ok_or(CodeGenError::new("Variable not mutable"))
        })?;
        let expr = self.impl_expr(&stmt.value, built_func, var.type_)?;

        self.builder
            .build_store(var.ptr, expr)
            .map_err(CodeGenError::from_llvm_err)
    }

    pub(crate) fn resolve_var(
        &self,
        built_func: FunctionValue<'ctx>,
        node: &nodes::ASTNodes,
    ) -> Result<Variable<'ctx>, CodeGenError> {
        match node {
            ASTNodes::Variable(var) => Ok(self
                .var_ptrs
                .get(&var.name)
                .ok_or(CodeGenError::new("Variable not found"))?),
            ASTNodes::Attr(attr) => self.impl_attr_access(built_func, attr),
            ASTNodes::ArrayIndex(ind) => self.impl_array_index(built_func, ind),
            _ => todo!("{:?}", node),
        }
    }

    pub(crate) fn impl_array_index(
        &self,
        built_func: FunctionValue<'ctx>,
        index: &nodes::ArrayIndex,
    ) -> Result<Variable<'ctx>, CodeGenError> {
        let mut array_var = self.resolve_var(built_func, &index.array_var)?;
        let index = self.impl_expr(&index.index, built_func, self.context.i32_type().into())?;
        let inner_dt = if let BasicTypeEnum::ArrayType(at) = array_var.type_ {
            at.get_element_type()
        } else if let BasicTypeEnum::VectorType(vt) = array_var.type_ {
            vt.get_element_type()
        } else {
            unreachable!()
        };
        let ptr = unsafe {
            self.builder
                .build_in_bounds_gep(
                    array_var.type_,
                    array_var.ptr,
                    &[self.context.i32_type().const_zero(), index.into_int_value()],
                    "",
                )
                .map_err(CodeGenError::from_llvm_err)
                .map(|v| v.into())
        }?;
        array_var.ptr = ptr;
        array_var.type_ = inner_dt;
        Ok(array_var)
    }

    pub(crate) fn impl_array_index_val(
        &self,
        built_func: FunctionValue<'ctx>,
        ind: &nodes::ArrayIndex,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        let array_var = self.impl_array_index(built_func, ind)?;
        self.builder
            .build_load(array_var.type_, array_var.ptr, "")
            .map_err(CodeGenError::from_llvm_err)
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

    #[test]
    fn test_codegen_access_array() {
        let data = "func main() u32 { 
let u32[] a = [1, 2, 3, 4, 5]
return a[2]
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca [5 x i32], align 4
  store [5 x i32] [i32 1, i32 2, i32 3, i32 4, i32 5], ptr %a, align 4
  %0 = getelementptr inbounds [5 x i32], ptr %a, i32 0, i32 2
  %1 = load i32, ptr %0, align 4
  ret i32 %1
}
"#
        )
    }

    #[test]
    fn test_codegen_access_array_nested() {
        let data = "func main() u32 { 
let u32[][][] a = [[[1, 2], [3, 4]], [[5, 6], [7, 8]], [[9, 10], [11, 12]]]
return a[2][1][0]
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca [3 x [2 x [2 x i32]]], align 4
  store [3 x [2 x [2 x i32]]] [[2 x [2 x i32]] [[2 x i32] [i32 1, i32 2], [2 x i32] [i32 3, i32 4]], [2 x [2 x i32]] [[2 x i32] [i32 5, i32 6], [2 x i32] [i32 7, i32 8]], [2 x [2 x i32]] [[2 x i32] [i32 9, i32 10], [2 x i32] [i32 11, i32 12]]], ptr %a, align 4
  %0 = getelementptr inbounds [3 x [2 x [2 x i32]]], ptr %a, i32 0, i32 2
  %1 = getelementptr inbounds [2 x [2 x i32]], ptr %0, i32 0, i32 1
  %2 = getelementptr inbounds [2 x i32], ptr %1, i32 0, i32 0
  %3 = load i32, ptr %2, align 4
  ret i32 %3
}
"#
        )
    }
}
