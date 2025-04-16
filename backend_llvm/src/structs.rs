use std::{cell::RefCell, collections::HashMap};

use inkwell::types::StructType;
use new_parser::nodes::StructDef;

use crate::CodeGen;

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
    pub(crate) fn def_struct(&self, r#struct: &StructDef) {
        let struct_def = self.context.opaque_struct_type(&r#struct.name);
        let fields = r#struct
            .fields
            .iter()
            .map(|field| self.parser_to_llvm_dt(&field.1))
            .collect::<Vec<_>>();
        struct_def.set_body(&fields, false);

        self.struct_defs.add_struct(r#struct, struct_def);
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
}
