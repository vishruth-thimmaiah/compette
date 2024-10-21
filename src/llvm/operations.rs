use inkwell::values::BasicValueEnum;

use super::func::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn add_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            self.builder
                .build_int_add(left_int, right_int, "")
                .unwrap()
                .into()
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            self.builder
                .build_float_add(left_float, right_float, "")
                .unwrap()
                .into()
        }
    }

    pub fn sub_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            self.builder
                .build_int_sub(left_int, right_int, "")
                .unwrap()
                .into()
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            self.builder
                .build_float_sub(left_float, right_float, "")
                .unwrap()
                .into()
        }
    }

    pub fn mul_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            self.builder
                .build_int_mul(left_int, right_int, "")
                .unwrap()
                .into()
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            self.builder
                .build_float_mul(left_float, right_float, "")
                .unwrap()
                .into()
        }
    }

    pub fn div_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            self.builder
                .build_int_signed_div(left_int, right_int, "")
                .unwrap()
                .into()
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            self.builder
                .build_float_div(left_float, right_float, "")
                .unwrap()
                .into()
        }
    }

    pub fn to_bool(&self, expr: &BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        if expr.is_int_value() {
            let val = self.context.i64_type().const_zero();
            let lhs = expr.into_int_value();
            if lhs.get_type().get_bit_width() == 1 {
                return *expr;
            }
            self.builder
                .build_int_compare(inkwell::IntPredicate::NE, lhs, val, "")
                .unwrap()
                .into()
        } else {
            let val = self.context.f64_type().const_zero();
            self.builder
                .build_float_compare(
                    inkwell::FloatPredicate::ONE,
                    expr.into_float_value(),
                    val,
                    "",
                )
                .unwrap()
                .into()
        }
    }
}
