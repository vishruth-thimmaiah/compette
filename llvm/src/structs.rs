use inkwell::values::{BasicValueEnum, PointerValue};

use lexer::types::Datatype;
use parser::nodes::{StructDefParserNode, StructParserNode};

use crate::compiler_error;

use super::codegen::{CodeGen, StructStore};

impl<'ctx> CodeGen<'ctx> {
    pub fn def_struct(&self, node: &StructDefParserNode) {
        let a = self.context.opaque_struct_type(&node.struct_name);
        let fields = node
            .fields
            .iter()
            .map(|field| self.def_expr(&field.1).unwrap())
            .collect::<Vec<_>>();
        a.set_body(&fields, false);

        self.structs.borrow_mut().push(StructStore {
            name: node.struct_name.clone(),
            fields: node.fields.iter().map(|x| x.0.clone()).collect(),
        });
    }

    pub fn create_struct(&self, name: &str, fields: &StructParserNode) -> PointerValue<'ctx> {
        let struct_type = self.module.get_struct_type(name).unwrap();
        let ref_cell = &self.structs.borrow();
        let struct_def = &ref_cell.iter().find(|x| &x.name == name).unwrap().fields;

        let len = struct_def.len();
        let _ = fields.fields.len() == len || compiler_error("invalid struct: fields do not match");

        let mut struct_fields = vec![self.context.i32_type().const_zero().into(); len];

        for (i, value) in fields.fields.iter() {
            let field = struct_def
                .iter()
                .position(|x| x == i)
                .unwrap_or_else(|| compiler_error("invalid struct: field not found"));

            let req_type =
                self.get_datatype(struct_type.get_field_type_at_index(field as u32).unwrap());
            let value = self.add_expression(&value, name, &req_type);
            struct_fields[field] = value;
        }

        let struct_value = struct_type.const_named_struct(&struct_fields);

        let ptr = self.builder.build_alloca(struct_type, "").unwrap();
        self.builder.build_store(ptr, struct_value).unwrap();

        ptr
    }

    pub fn index_struct(
        &self,
        struct_name: &str,
        index_node: &str,
        func_name: &str,
    ) -> BasicValueEnum<'ctx> {
        let binding = self.variables.borrow();

        let function = binding.iter().find(|x| x.name == func_name).unwrap();
        let var = function.vars.iter().find(|x| x.0 == struct_name).unwrap().1;

        let nm = if let Datatype::CUSTOM(nm) = &var.datatype {
            nm
        } else {
            unreachable!()
        };

        let structs = self.structs.borrow();
        let field_labels = &structs.iter().find(|x| &x.name == nm).unwrap().fields;
        let field_index = field_labels.iter().position(|x| x == index_node).unwrap();

        let ty = self.module.get_struct_type(&nm).unwrap();
        let ptr = self
            .builder
            .build_struct_gep(ty, var.ptr, field_index as u32, "")
            .unwrap();

        self.builder
            .build_load(ty.get_field_type_at_index(0).unwrap(), ptr, "")
            .unwrap()
    }
}
