use inkwell::{
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum},
    values::{BasicValueEnum, PointerValue},
};

use crate::{lexer::types::DATATYPE, parser::nodes::AssignmentParserNode};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn new_ptr(&self, node: &AssignmentParserNode) -> PointerValue<'ctx> {
        let ty = self.def_expr(&node.var_type);
        self.builder.build_alloca(ty, &node.var_name).unwrap()
    }

    pub fn string_to_value(&self, value: &str, val_type: &DATATYPE) -> BasicValueEnum<'ctx> {
        let expr_type = self.def_expr(val_type);

        if expr_type.is_int_type() {
            expr_type
                .into_int_type()
                .const_int_from_string(value, inkwell::types::StringRadix::Decimal)
                .unwrap()
                .into()
        } else if expr_type.is_float_type() {
            expr_type
                .into_float_type()
                .const_float(value.parse::<f64>().unwrap())
                .into()
        } else {
            todo!()
        }
    }

    pub fn def_func_args(
        &self,
        args: &Vec<(String, DATATYPE)>,
    ) -> Vec<BasicMetadataTypeEnum<'ctx>> {
        let mut result_arr: Vec<BasicMetadataTypeEnum<'ctx>> = Vec::new();

        for arg in args {
            result_arr.push(self.def_expr(&arg.1).into());
        }

        return result_arr;
    }

    pub fn get_datatype(&self, bt: BasicValueEnum) -> &DATATYPE {
        match bt.get_type() {
            BasicTypeEnum::IntType(it) => match it.get_bit_width() {
                1 => &DATATYPE::BOOL,
                16 => &DATATYPE::U16,
                32 => &DATATYPE::U32,
                64 => &DATATYPE::U64,
                _ => todo!(),
            },
            BasicTypeEnum::FloatType(_) => todo!(),
            _ => todo!(),
        }
    }

    pub fn def_expr(&self, req_type: &DATATYPE) -> BasicTypeEnum<'ctx> {
        match req_type {
            DATATYPE::U8 => self.context.i8_type().into(),
            DATATYPE::U16 => self.context.i16_type().into(),
            DATATYPE::U32 => self.context.i32_type().into(),
            DATATYPE::U64 => self.context.i64_type().into(),
            DATATYPE::I8 => self.context.i8_type().into(),
            DATATYPE::I16 => self.context.i16_type().into(),
            DATATYPE::I32 => self.context.i32_type().into(),
            DATATYPE::I64 => self.context.i64_type().into(),
            DATATYPE::BOOL => self.context.bool_type().into(),
            DATATYPE::F32 => self.context.f32_type().into(),
            DATATYPE::F64 => self.context.f64_type().into(),

            DATATYPE::STRING => todo!(),
            DATATYPE::ARRAY(inner) => self
                .def_expr(&inner.datatype)
                .array_type(inner.length)
                .into(),
        }
    }
}
