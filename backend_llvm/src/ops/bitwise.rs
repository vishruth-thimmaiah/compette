use inkwell::values::BasicValueEnum;

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn and_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if !(left.is_int_value() && right.is_int_value()) && !(left.is_vector_value() && right.is_vector_value()) {
            return Err(CodeGenError::new(
                "Bitwise operations can only be performed on integers",
            ));
        }
        self.builder
            .build_and(left.into_int_value(), right.into_int_value(), "")
            .map_err(CodeGenError::from_llvm_err)
            .map(|op| op.into())
    }

    pub(crate) fn or_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if !(left.is_int_value() && right.is_int_value()) && !(left.is_vector_value() && right.is_vector_value()) {
            return Err(CodeGenError::new(
                "Bitwise operations can only be performed on integers",
            ));
        }
        self.builder
            .build_or(left.into_int_value(), right.into_int_value(), "")
            .map_err(CodeGenError::from_llvm_err)
            .map(|op| op.into())
    }

    pub(crate) fn xor_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if !(left.is_int_value() && right.is_int_value()) && !(left.is_vector_value() && right.is_vector_value()) {
            return Err(CodeGenError::new(
                "Bitwise operations can only be performed on integers",
            ));
        }
        self.builder
            .build_xor(left.into_int_value(), right.into_int_value(), "")
            .map_err(CodeGenError::from_llvm_err)
            .map(|op| op.into())
    }

    pub(crate) fn shl_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if !(left.is_int_value() && right.is_int_value()) && !(left.is_vector_value() && right.is_vector_value()) {
            return Err(CodeGenError::new(
                "Bitwise operations can only be performed on integers",
            ));
        }

        self.builder
            .build_left_shift(left.into_int_value(), right.into_int_value(), "")
            .map_err(CodeGenError::from_llvm_err)
            .map(|op| op.into())
    }

    pub(crate) fn shr_binary_operation(
        &self,
        left: &BasicValueEnum<'ctx>,
        right: &BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if !(left.is_int_value() && right.is_int_value()) && !(left.is_vector_value() && right.is_vector_value()) {
            return Err(CodeGenError::new(
                "Bitwise operations can only be performed on integers",
            ));
        }
        self.builder
            .build_right_shift(left.into_int_value(), right.into_int_value(), false, "")
            .map_err(CodeGenError::from_llvm_err)
            .map(|op| op.into())
    }
}
