use inkwell::values::PointerValue;

use crate::{lexer::types::Types, parser::nodes::AssignmentParserNode};

use super::func::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn new_ptr(&self, node: &AssignmentParserNode) -> PointerValue<'ctx> {
        match node.var_type {
            Types::U16 => self
                .builder
                .build_alloca(self.context.i16_type(), &node.var_name)
                .unwrap(),
            Types::U32 => self
                .builder
                .build_alloca(self.context.i32_type(), &node.var_name)
                .unwrap(),
            Types::U64 => self
                .builder
                .build_alloca(self.context.i64_type(), &node.var_name)
                .unwrap(),
            Types::I16 => self
                .builder
                .build_alloca(self.context.i16_type(), &node.var_name)
                .unwrap(),
            Types::I32 => self
                .builder
                .build_alloca(self.context.i32_type(), &node.var_name)
                .unwrap(),
            Types::I64 => self
                .builder
                .build_alloca(self.context.i64_type(), &node.var_name)
                .unwrap(),
            Types::F32 => self
                .builder
                .build_alloca(self.context.f32_type(), &node.var_name)
                .unwrap(),
            Types::F64 => self
                .builder
                .build_alloca(self.context.f64_type(), &node.var_name)
                .unwrap(),
            Types::BOOL => self
                .builder
                .build_alloca(self.context.bool_type(), &node.var_name)
                .unwrap(),
            Types::STRING => todo!(),
            _ => todo!(),
        }
    }
}
