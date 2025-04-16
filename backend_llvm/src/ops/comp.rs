use inkwell::values::BasicValueEnum;
use lexer::types::Operator;

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn comp_binary_operation(
        &self,
        op: &Operator,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, ()> {
        let (ip, fp) = self.ops_to_llvm_predicate(op);
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            Ok(self
                .builder
                .build_int_compare(ip, left_int, right_int, "")
                .unwrap()
                .into())
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            Ok(self
                .builder
                .build_float_compare(fp, left_float, right_float, "")
                .unwrap()
                .into())
        }
    }

    pub(crate) fn ops_to_llvm_predicate(
        &self,
        op: &Operator,
    ) -> (inkwell::IntPredicate, inkwell::FloatPredicate) {
        match op {
            Operator::EQUAL => (inkwell::IntPredicate::EQ, inkwell::FloatPredicate::OEQ),
            Operator::NOT_EQUAL => (inkwell::IntPredicate::NE, inkwell::FloatPredicate::ONE),
            Operator::GREATER => (inkwell::IntPredicate::SGT, inkwell::FloatPredicate::OGT),
            Operator::LESSER => (inkwell::IntPredicate::SLT, inkwell::FloatPredicate::OLT),
            Operator::GREATER_EQUAL => (inkwell::IntPredicate::SGE, inkwell::FloatPredicate::OGE),
            Operator::LESSER_EQUAL => (inkwell::IntPredicate::SLE, inkwell::FloatPredicate::UEQ),
            _ => todo!("Binary operator {:?} not implemented", op),
        }
    }
}
