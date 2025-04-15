use inkwell::types::BasicTypeEnum;
use lexer::types::Datatype;

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn parser_to_llvm_dt(&self, bt: &Datatype) -> BasicTypeEnum<'ctx> {
        match bt {
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
            Datatype::NONE => todo!(),
            Datatype::STRING(_) => todo!(),
            Datatype::NARRAY(_) => todo!(),
            Datatype::CUSTOM(_) => todo!(),
            // To be removed
            _ => unreachable!(),
        }
    }
}
