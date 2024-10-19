use inkwell::{
    types::{BasicType, FunctionType, IntType},
    values::PointerValue,
};

use crate::{lexer::types::DATATYPE, parser::nodes::AssignmentParserNode};

use super::func::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn new_ptr(&self, node: &AssignmentParserNode) -> PointerValue<'ctx> {
        match node.var_type {
            DATATYPE::U16 => self
                .builder
                .build_alloca(self.context.i16_type(), &node.var_name)
                .unwrap(),
            DATATYPE::U32 => self
                .builder
                .build_alloca(self.context.i32_type(), &node.var_name)
                .unwrap(),
            DATATYPE::U64 => self
                .builder
                .build_alloca(self.context.i64_type(), &node.var_name)
                .unwrap(),
            DATATYPE::I16 => self
                .builder
                .build_alloca(self.context.i16_type(), &node.var_name)
                .unwrap(),
            DATATYPE::I32 => self
                .builder
                .build_alloca(self.context.i32_type(), &node.var_name)
                .unwrap(),
            DATATYPE::I64 => self
                .builder
                .build_alloca(self.context.i64_type(), &node.var_name)
                .unwrap(),
            DATATYPE::F32 => self
                .builder
                .build_alloca(self.context.f32_type(), &node.var_name)
                .unwrap(),
            DATATYPE::F64 => self
                .builder
                .build_alloca(self.context.f64_type(), &node.var_name)
                .unwrap(),
            DATATYPE::BOOL => self
                .builder
                .build_alloca(self.context.bool_type(), &node.var_name)
                .unwrap(),
            DATATYPE::STRING => todo!(),
        }
    }
    pub fn def_func(&self, ret_type: &DATATYPE) -> FunctionType<'ctx> {
        let llvm_ret_type: Box<dyn BasicType> = match ret_type {
            DATATYPE::U16 => Box::new(self.context.i16_type()),
            DATATYPE::U32 => Box::new(self.context.i32_type()),
            DATATYPE::U64 => Box::new(self.context.i64_type()),
            DATATYPE::I16 => Box::new(self.context.i16_type()),
            DATATYPE::I32 => Box::new(self.context.i32_type()),
            DATATYPE::I64 => Box::new(self.context.i64_type()),
            DATATYPE::F32 => Box::new(self.context.f32_type()),
            DATATYPE::F64 => Box::new(self.context.f64_type()),
            DATATYPE::BOOL => Box::new(self.context.bool_type()),
            DATATYPE::STRING => todo!(),
        };

        llvm_ret_type.fn_type(&[], false)
    }

    pub fn def_expr(&self, req_val: &DATATYPE) -> IntType {
        match req_val {
            DATATYPE::U16 => self.context.i16_type(),
            DATATYPE::U32 => self.context.i32_type(),
            DATATYPE::U64 => self.context.i64_type(),
            DATATYPE::I16 => self.context.i16_type(),
            DATATYPE::I32 => self.context.i32_type(),
            DATATYPE::I64 => self.context.i64_type(),
            DATATYPE::BOOL => self.context.bool_type(),
            DATATYPE::STRING => todo!(),
            _ => todo!(),
        }
    }
}
