use inkwell::{
    types::{BasicType, BasicTypeEnum},
    values::{ArrayValue, BasicValueEnum},
};
use lexer::types::Datatype;

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn parser_to_llvm_dt(&self, dt: &Datatype) -> BasicTypeEnum<'ctx> {
        match dt {
            Datatype::U8 => self.context.i8_type().into(),
            Datatype::U16 => self.context.i16_type().into(),
            Datatype::U32 => self.context.i32_type().into(),
            Datatype::U64 => self.context.i64_type().into(),
            Datatype::I8 => self.context.i8_type().into(),
            Datatype::I16 => self.context.i16_type().into(),
            Datatype::I32 => self.context.i32_type().into(),
            Datatype::I64 => self.context.i64_type().into(),
            Datatype::BOOL => self.context.bool_type().into(),
            Datatype::F32 => self.context.f32_type().into(),
            Datatype::F64 => self.context.f64_type().into(),
            Datatype::STRING(_) => todo!(),
            Datatype::NARRAY(dt, size) => {
                self.parser_to_llvm_dt(dt).array_type(*size as u32).into()
            }
            Datatype::CUSTOM(name) => self.struct_defs.get_struct_ptr(name).unwrap().into(),
            Datatype::NONE => unreachable!(),
            // To be removed
            _ => unreachable!(),
        }
    }

    pub(crate) fn dt_to_array(
        &self,
        dt: &BasicTypeEnum<'ctx>,
        values: Vec<BasicValueEnum<'ctx>>,
    ) -> ArrayValue<'ctx> {
        match dt {
            BasicTypeEnum::ArrayType(at) => at.const_array(
                &values
                    .iter()
                    .map(|v| v.into_array_value())
                    .collect::<Vec<_>>(),
            ),
            BasicTypeEnum::IntType(it) => it.const_array(
                &values
                    .iter()
                    .map(|v| v.into_int_value())
                    .collect::<Vec<_>>(),
            ),
            BasicTypeEnum::FloatType(ft) => ft.const_array(
                &values
                    .iter()
                    .map(|v| v.into_float_value())
                    .collect::<Vec<_>>(),
            ),
            BasicTypeEnum::PointerType(pt) => pt.const_array(
                &values
                    .iter()
                    .map(|v| v.into_pointer_value())
                    .collect::<Vec<_>>(),
            ),
            BasicTypeEnum::StructType(st) => st.const_array(
                &values
                    .iter()
                    .map(|v| v.into_struct_value())
                    .collect::<Vec<_>>(),
            ),
            BasicTypeEnum::VectorType(vt) => vt.const_array(
                &values
                    .iter()
                    .map(|v| v.into_vector_value())
                    .collect::<Vec<_>>(),
            ),
        }
    }
}
