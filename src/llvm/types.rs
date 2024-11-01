use inkwell::{
    types::{BasicType, BasicTypeEnum},
    values::BasicValueEnum,
};

use crate::lexer::types::{ArrayDetails, DATATYPE};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
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
        } else if let &DATATYPE::STRING(_) = val_type {
            self.context.const_string(value.as_bytes(), true).into()
        } else {
            todo!()
        }
    }

    pub fn get_datatype(&self, bt: BasicTypeEnum<'ctx>) -> DATATYPE {
        match bt {
            BasicTypeEnum::IntType(it) => match it.get_bit_width() {
                1 => DATATYPE::BOOL,
                8 => DATATYPE::U8,
                16 => DATATYPE::U16,

                32 => DATATYPE::U32,
                64 => DATATYPE::U64,
                _ => todo!(),
            },
            BasicTypeEnum::FloatType(_) => todo!(),
            BasicTypeEnum::ArrayType(arr) => DATATYPE::ARRAY(Box::new(ArrayDetails {
                datatype: self.get_datatype(arr.get_element_type()).clone(),
                length: arr.len(),
            })),
            _ => todo!(),
        }
    }

    // TODO: Make U.. unsigned
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

            DATATYPE::STRING(len) => self
                .context
                .const_string(&vec![0; *len], true)
                .get_type()
                .into(),
            DATATYPE::ARRAY(inner) => self
                .def_expr(&inner.datatype)
                .array_type(inner.length)
                .into(),
        }
    }
}
