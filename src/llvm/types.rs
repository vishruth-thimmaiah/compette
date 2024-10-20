use inkwell::{
    types::{BasicType, BasicTypeEnum, FunctionType},
    values::{BasicValueEnum, PointerValue},
};

use crate::{lexer::types::DATATYPE, parser::nodes::AssignmentParserNode};

use super::func::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn new_ptr(&self, node: &AssignmentParserNode) -> PointerValue<'ctx> {
        let ty = self.def_expr(&node.var_type);
        self.builder.build_alloca(ty, &node.var_name).unwrap()
    }

    pub fn def_func(&self, ret_type: &DATATYPE) -> FunctionType<'ctx> {
        self.def_expr(ret_type).fn_type(&[], false)
    }

    pub fn string_to_value(&self, value: &str, val_type: &DATATYPE) -> BasicValueEnum<'ctx> {
        let expr_type = self.def_expr(val_type);

        if expr_type.is_int_type() {
            expr_type
                .into_int_type()
                .const_int_from_string(value, inkwell::types::StringRadix::Decimal).unwrap().into()
        }
        else if expr_type.is_float_type() {
            expr_type.into_float_type().const_float(value.parse::<f64>().unwrap()).into()
        }
        else {
            todo!()
        }
    }

    pub fn def_expr(&self, req_val: &DATATYPE) -> BasicTypeEnum<'ctx> {
        match req_val {
            DATATYPE::U16 => self.context.i16_type().into(),
            DATATYPE::U32 => self.context.i32_type().into(),
            DATATYPE::U64 => self.context.i64_type().into(),
            DATATYPE::I16 => self.context.i16_type().into(),
            DATATYPE::I32 => self.context.i32_type().into(),
            DATATYPE::I64 => self.context.i64_type().into(),
            DATATYPE::BOOL => self.context.bool_type().into(),
            DATATYPE::F32 => self.context.f32_type().into(),
            DATATYPE::F64 => self.context.f64_type().into(),
            DATATYPE::STRING => todo!(),
        }
    }
}
