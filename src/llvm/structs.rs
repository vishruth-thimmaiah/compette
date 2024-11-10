use inkwell::values::{BasicValueEnum, PointerValue};

use crate::{
    lexer::types::DATATYPE,
    parser::nodes::{StructParserNode, ValueIterParserNode},
};

use super::codegen::{CodeGen, StructStore};

impl<'ctx> CodeGen<'ctx> {
    pub fn def_struct(&self, node: &StructParserNode) {
        let a = self.context.opaque_struct_type(&node.struct_name);
        let fields = node
            .fields
            .values()
            .map(|dt| self.def_expr(dt).unwrap())
            .collect::<Vec<_>>();
        a.set_body(&fields, false);

        self.structs.borrow_mut().push(StructStore {
            name: node.struct_name.clone(),
            fields: node.fields.keys().map(|x| x.clone()).collect(),
        });
    }

    pub fn create_struct(&self, name: &str, fields: &ValueIterParserNode) -> PointerValue<'ctx> {
        let struct_type = self.module.get_struct_type(name).unwrap();

        let mut struct_val = vec![];

        for (i, value) in fields.value.iter().enumerate() {
            let req_type =
                self.get_datatype(struct_type.get_field_type_at_index(i as u32).unwrap());
            let value = self.add_expression(&value, name, &req_type);
            struct_val.push(value);
        }

        let struct_value = struct_type.const_named_struct(&struct_val);

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

        let nm = if let DATATYPE::CUSTOM(nm) = &var.datatype {
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
