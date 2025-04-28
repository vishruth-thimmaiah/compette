use inkwell::values::BasicValueEnum;

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn add_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            Ok(self
                .builder
                .build_int_add(left_int, right_int, "")
                .unwrap()
                .into())
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            Ok(self
                .builder
                .build_float_add(left_float, right_float, "")
                .unwrap()
                .into())
        }
    }

    pub(crate) fn sub_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            Ok(self
                .builder
                .build_int_sub(left_int, right_int, "")
                .unwrap()
                .into())
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            Ok(self
                .builder
                .build_float_sub(left_float, right_float, "")
                .unwrap()
                .into())
        }
    }

    pub(crate) fn mul_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            Ok(self
                .builder
                .build_int_mul(left_int, right_int, "")
                .unwrap()
                .into())
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            Ok(self
                .builder
                .build_float_mul(left_float, right_float, "")
                .unwrap()
                .into())
        }
    }

    pub(crate) fn div_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            Ok(self
                .builder
                .build_int_signed_div(left_int, right_int, "")
                .unwrap()
                .into())
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            Ok(self
                .builder
                .build_float_div(left_float, right_float, "")
                .unwrap()
                .into())
        }
    }

    pub(crate) fn mod_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if left.is_int_value() && right.is_int_value() {
            let left_int = left.into_int_value();
            let right_int = right.into_int_value();
            Ok(self
                .builder
                .build_int_signed_rem(left_int, right_int, "")
                .unwrap()
                .into())
        } else {
            let left_float = left.into_float_value();
            let right_float = right.into_float_value();
            Ok(self
                .builder
                .build_float_rem(left_float, right_float, "")
                .unwrap()
                .into())
        }
    }
}
