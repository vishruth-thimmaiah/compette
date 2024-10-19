use inkwell::{
    types::{BasicType, FunctionType, IntType},
    values::PointerValue,
};

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
    pub fn def_func(&self, ret_type: &Types) -> FunctionType<'ctx> {
        let llvm_ret_type: Box<dyn BasicType> = match ret_type {
            Types::U16 => Box::new(self.context.i16_type()),
            Types::U32 => Box::new(self.context.i32_type()),
            Types::U64 => Box::new(self.context.i64_type()),
            Types::I16 => Box::new(self.context.i16_type()),
            Types::I32 => Box::new(self.context.i32_type()),
            Types::I64 => Box::new(self.context.i64_type()),
            Types::F32 => Box::new(self.context.f32_type()),
            Types::F64 => Box::new(self.context.f64_type()),
            Types::BOOL => Box::new(self.context.bool_type()),
            Types::STRING => todo!(),
            _ => todo!(),
        };

        llvm_ret_type.fn_type(&[], false)
    }

    pub fn def_expr(&self, req_val: &Types) -> IntType {
        match req_val {
            Types::U16 => self.context.i16_type(),
            Types::U32 => self.context.i32_type(),
            Types::U64 => self.context.i64_type(),
            Types::I16 => self.context.i16_type(),
            Types::I32 => self.context.i32_type(),
            Types::I64 => self.context.i64_type(),
            Types::BOOL => self.context.bool_type(),
            Types::STRING => todo!(),
            _ => todo!(),
        }
    }
}
