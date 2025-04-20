use std::{cell::RefCell, collections::HashMap};

use inkwell::{
    types::StructType,
    values::{BasicValueEnum, FunctionValue},
};
use parser::nodes::{self, StructDef};

use crate::{CodeGen, CodeGenError, stmt::Variable};

#[derive(Debug, Default)]
pub struct StructDefs<'ctx> {
    items: RefCell<HashMap<String, StructData<'ctx>>>,
}

#[derive(Debug)]
pub struct StructData<'ctx> {
    ptr: StructType<'ctx>,
    fields: Vec<String>,
}

impl<'ctx> StructDefs<'ctx> {
    pub(crate) fn add_struct(&self, def: &StructDef, ptr: StructType<'ctx>) {
        let fields = def.fields.iter().map(|x| x.0.clone()).collect::<Vec<_>>();
        self.items
            .borrow_mut()
            .insert(def.name.clone(), StructData { ptr, fields });
    }

    pub(crate) fn get_field_index(&self, name: &str, field: &str) -> Option<usize> {
        let structs = self.items.borrow();
        structs
            .get(name)
            .map(|x| x.fields.iter().position(|x| x == field))?
    }

    pub(crate) fn get_struct_ptr(&self, name: &str) -> Option<StructType<'ctx>> {
        let structs = self.items.borrow();
        structs.get(name).map(|x| x.ptr)
    }
}

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn def_struct(
        &self,
        r#struct: &StructDef,
    ) -> Result<StructType<'ctx>, CodeGenError> {
        let struct_def = self.context.opaque_struct_type(&r#struct.name);
        let fields = r#struct
            .fields
            .iter()
            .map(|field| self.parser_to_llvm_dt(&field.1))
            .collect::<Vec<_>>();
        struct_def.set_body(&fields, false);

        self.struct_defs.add_struct(r#struct, struct_def);
        Ok(struct_def)
    }

    pub(crate) fn impl_attr_access(
        &self,
        built_func: FunctionValue<'ctx>,
        attr: &nodes::Attr,
    ) -> Result<Variable<'ctx>, CodeGenError> {
        let mut struct_var = self.resolve_var(built_func, &attr.parent)?;
        let struct_ty = struct_var.type_.into_struct_type();

        let field_index = self
            .struct_defs
            .get_field_index(
                &struct_ty.get_name().unwrap().to_str().unwrap(),
                &attr.name.name,
            )
            .ok_or(CodeGenError::new("Field not found"))?;

        let ptr = self
            .builder
            .build_struct_gep(struct_ty, struct_var.ptr, field_index as u32, "")
            .map_err(CodeGenError::from_llvm_err)?;

        struct_var.ptr = ptr;
        struct_var.type_ = struct_ty
            .get_field_type_at_index(field_index as u32)
            .unwrap();
        Ok(struct_var)
    }

    pub(crate) fn impl_attr_access_val(
        &self,
        built_func: FunctionValue<'ctx>,
        attr: &nodes::Attr,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        let struct_var = self.impl_attr_access(built_func, attr)?;

        self.builder
            .build_load(struct_var.type_, struct_var.ptr, "")
            .map_err(CodeGenError::from_llvm_err)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_codegen_struct_def() {
        let data = r#"
        struct Test { a u32, b u32 }
        func main() u32 { let Test t = { b 28, a 1 } return 0 }"#;
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

%Test = type { i32, i32 }

define i32 @main() {
entry:
  %t = alloca %Test, align 8
  store %Test { i32 1, i32 28 }, ptr %t, align 4
  ret i32 0
}
"#
        )
    }

    #[test]
    fn test_codegen_struct_field_access() {
        let data = r#"
        struct Test { a u32, b u32 }
        func main() u32 { 
            let Test t = { b 28, a 1 } 
            return t.b
        }"#;
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

%Test = type { i32, i32 }

define i32 @main() {
entry:
  %t = alloca %Test, align 8
  store %Test { i32 1, i32 28 }, ptr %t, align 4
  %0 = getelementptr inbounds %Test, ptr %t, i32 0, i32 1
  %1 = load i32, ptr %0, align 4
  ret i32 %1
}
"#
        )
    }

    #[test]
    fn test_codegen_struct_field_update() {
        let data = r#"
        struct Test { a u32, b u32 }
        func main() u32 { 
            let Test! t = { b 28, a 1 } 
            t.b = 10
            return t.b
        }"#;
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

%Test = type { i32, i32 }

define i32 @main() {
entry:
  %t = alloca %Test, align 8
  store %Test { i32 1, i32 28 }, ptr %t, align 4
  %0 = getelementptr inbounds %Test, ptr %t, i32 0, i32 1
  store i32 10, ptr %0, align 4
  %1 = getelementptr inbounds %Test, ptr %t, i32 0, i32 1
  %2 = load i32, ptr %1, align 4
  ret i32 %2
}
"#
        )
    }

    #[test]
    fn test_codegen_struct_field_update_nested() {
        let data = r#"
        struct A { a u32 }
        struct B { a A }
        struct C { b B }
        func main() u32 { 
            let C! c = { b { a { a 1 } } }
            c.b.a.a = 10
            return c.b.a.a
        }"#;
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

%C = type { %B }
%B = type { %A }
%A = type { i32 }

define i32 @main() {
entry:
  %c = alloca %C, align 8
  store %C { %B { %A { i32 1 } } }, ptr %c, align 4
  %0 = getelementptr inbounds %C, ptr %c, i32 0, i32 0
  %1 = getelementptr inbounds %B, ptr %0, i32 0, i32 0
  %2 = getelementptr inbounds %A, ptr %1, i32 0, i32 0
  store i32 10, ptr %2, align 4
  %3 = getelementptr inbounds %C, ptr %c, i32 0, i32 0
  %4 = getelementptr inbounds %B, ptr %3, i32 0, i32 0
  %5 = getelementptr inbounds %A, ptr %4, i32 0, i32 0
  %6 = load i32, ptr %5, align 4
  ret i32 %6
}
"#
        )
    }
}
