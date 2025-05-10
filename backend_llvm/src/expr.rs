use inkwell::{
    AddressSpace,
    types::{BasicType, BasicTypeEnum},
    values::{BasicValueEnum, FunctionValue},
};
use lexer::types::{Operator, Types};
use parser::nodes::{ASTNodes, Expression, Literal, Variable};

use crate::{CodeGen, CodeGenError};

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_expr(
        &self,
        node: &Expression,
        built_func: FunctionValue<'ctx>,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match node {
            Expression::Simple {
                left,
                right,
                operator,
            } => {
                let left_val = self.impl_simple_expr_arm(left, built_func, dt)?;

                if let Some(right_val) = right {
                    if let ASTNodes::Token(Types::DATATYPE(dt)) = &**right_val {
                        operator.as_ref().unwrap().ne(&Operator::CAST).then_some(
                            CodeGenError::new("Invalid expression; expected a cast operation"),
                        );
                        let dt = self.parser_to_llvm_dt(dt);
                        return self.impl_cast_expr(left_val, dt);
                    }

                    let right_val = self.impl_simple_expr_arm(right_val, built_func, dt)?;
                    let (left_val, right_val) = self.impl_cast_simple_expr(left_val, right_val)?;
                    return self.impl_binary_operation(
                        left_val,
                        right_val,
                        operator.as_ref().unwrap(),
                    );
                }
                return Ok(left_val);
            }
            Expression::Array(arr) if dt.is_array_type() => {
                let dt = dt.into_array_type();
                let inner_dt = dt.get_element_type();
                inner_dt.as_basic_type_enum();
                let mut array_val = vec![];
                for value in arr {
                    array_val.push(self.impl_expr(value, built_func, inner_dt)?);
                }
                return Ok(self.dt_to_array(&inner_dt, array_val).into());
            }
            Expression::Struct(fields) if dt.is_struct_type() => {
                let mut struct_vals = vec![None; fields.len()];
                let dt = dt.into_struct_type();
                let name = dt.get_name().unwrap().to_str().unwrap();

                for (field, val) in fields {
                    let field = self.struct_defs.get_field_index(name, field).unwrap();
                    struct_vals[field] = Some(
                        self.impl_expr(
                            val,
                            built_func,
                            dt.get_field_type_at_index(field as u32).unwrap(),
                        )
                        .unwrap(),
                    );
                }
                let struct_vals = struct_vals
                    .into_iter()
                    .map(|v| v.unwrap())
                    .collect::<Vec<_>>();
                Ok(dt.const_named_struct(&struct_vals).into())
            }
            Expression::String(str) if dt.is_pointer_type() => {
                let string = self.context.const_string(str.as_bytes(), true);
                let string_ptr = self.builder.build_alloca(string.get_type(), "").unwrap();
                self.builder.build_store(string_ptr, string).unwrap();

                Ok(string_ptr.into())
            }
            Expression::String(str) => {
                println!("dt: {:?}", dt);
                let string = self.context.const_string(str.as_bytes(), false);
                let string_ptr = self.builder.build_alloca(string.get_type(), "").unwrap();
                self.builder.build_store(string_ptr, string).unwrap();

                let struct_ty = self.context.struct_type(
                    &[
                        self.context.i64_type().into(),
                        self.context.ptr_type(AddressSpace::default()).into(),
                    ],
                    false,
                );
                let struct_ptr = self.builder.build_alloca(struct_ty, "").unwrap();
                let str_struct = struct_ty.get_undef();

                let str_struct = self
                    .builder
                    .build_insert_value(
                        str_struct,
                        self.context.i64_type().const_int(str.len() as u64, false),
                        0,
                        "",
                    )
                    .unwrap();
                let str_struct = self
                    .builder
                    .build_insert_value(str_struct, string_ptr, 1, "")
                    .unwrap();

                self.builder.build_store(struct_ptr, str_struct).unwrap();

                Ok(struct_ptr.into())
            }
            _ => todo!(),
        }
    }

    fn impl_simple_expr_arm(
        &self,
        arm: &ASTNodes,
        built_func: FunctionValue<'ctx>,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match arm {
            ASTNodes::Literal(lit) => self.impl_literal(lit, dt),
            ASTNodes::Variable(var) => self.impl_variable(var, built_func),
            ASTNodes::Expression(expr) => self.impl_expr(expr, built_func, dt),
            ASTNodes::FunctionCall(call) => {
                self.impl_function_call(built_func, call).and_then(|v| {
                    v.ok_or(CodeGenError::new(
                        "Function does not have an associated return type; it cannot be used as an expression",
                    ))
                })
            }
            ASTNodes::ArrayIndex(ind) => self.impl_array_index_val(built_func, ind),
            ASTNodes::Attr(attr) => self.impl_attr_access_val(built_func, attr),
            ASTNodes::Method(method) => self.impl_method_call(built_func, method),
            _ => todo!("Simple expr arm {:?}", arm),
        }
    }

    fn impl_binary_operation(
        &self,
        left_val: BasicValueEnum<'ctx>,
        right_val: BasicValueEnum<'ctx>,
        operator: &Operator,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match operator {
            Operator::PLUS => self.add_binary_operation(&left_val, &right_val),
            Operator::MINUS => self.sub_binary_operation(&left_val, &right_val),
            Operator::MULTIPLY => self.mul_binary_operation(&left_val, &right_val),
            Operator::DIVIDE => self.div_binary_operation(&left_val, &right_val),
            Operator::MODULO => self.mod_binary_operation(&left_val, &right_val),
            Operator::EQUAL
            | Operator::NOT_EQUAL
            | Operator::GREATER
            | Operator::GREATER_EQUAL
            | Operator::LESSER
            | Operator::LESSER_EQUAL => self
                .comp_binary_operation(operator, &left_val, &right_val)
                .map(|v| v.into()),
            Operator::BITWISE_AND => self.and_binary_operation(&left_val, &right_val),
            Operator::BITWISE_OR => self.or_binary_operation(&left_val, &right_val),
            Operator::BITWISE_XOR => self.xor_binary_operation(&left_val, &right_val),
            Operator::LSHIFT => self.shl_binary_operation(&left_val, &right_val),
            Operator::RSHIFT => self.shr_binary_operation(&left_val, &right_val),
            _ => todo!("Binary operator {:?} not implemented", operator),
        }
    }

    fn impl_literal(
        &self,
        lit: &Literal,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        match lit.r#type {
            Types::BOOL => Ok(self
                .context
                .bool_type()
                .const_int(lit.value.parse::<u64>().unwrap(), false)
                .into()),
            Types::NUMBER => {
                if lit.value.contains('.') {
                    let f64_value = lit.value.parse::<f64>().unwrap();
                    return Ok(dt.into_float_type().const_float(f64_value).into());
                } else if dt.is_int_type() && dt.into_int_type().get_bit_width() != 1 {
                    let i64_value = lit.value.parse::<u64>().unwrap();
                    return Ok(dt.into_int_type().const_int(i64_value, false).into());
                } else {
                    let i64_value = lit.value.parse::<u64>().unwrap();
                    return Ok(self.context.i64_type().const_int(i64_value, false).into());
                }
            }
            _ => todo!(),
        }
    }

    fn impl_variable(
        &self,
        var: &Variable,
        built_func: FunctionValue<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        if let Some(var_data) = self.var_ptrs.get(&var.name) {
            if matches!(
                var_data.type_,
                BasicTypeEnum::ArrayType(_)
                    | BasicTypeEnum::StructType(_)
                    | BasicTypeEnum::PointerType(_)
            ) {
                return Ok(var_data.ptr.into());
            }
            println!("var_data: {:?}", var_data.type_);
            self.builder
                .build_load(var_data.type_, var_data.ptr, &var.name)
                .map_err(CodeGenError::from_llvm_err)
        } else {
            built_func
                .get_param_iter()
                .find(|param| param.get_name().to_str().unwrap() == var.name)
                .ok_or(CodeGenError::new(&format!(
                    "Variable {} not found",
                    var.name
                )))
        }
    }

    fn impl_cast_simple_expr(
        &self,
        left_expr: BasicValueEnum<'ctx>,
        right_expr: BasicValueEnum<'ctx>,
    ) -> Result<(BasicValueEnum<'ctx>, BasicValueEnum<'ctx>), CodeGenError> {
        let left_type = left_expr.get_type();
        let right_type = right_expr.get_type();

        let cast_fn = |op, value, r#type| {
            self.builder
                .build_cast(op, value, r#type, "")
                .map_err(CodeGenError::from_llvm_err)
        };

        match (left_type, right_type) {
            _ if left_type == right_type => Ok((left_expr, right_expr)),
            (BasicTypeEnum::IntType(a), BasicTypeEnum::IntType(b))
                if a.get_bit_width() < b.get_bit_width() =>
            {
                cast_fn(
                    inkwell::values::InstructionOpcode::ZExt,
                    left_expr,
                    right_type,
                )
                .map(|v| (v, right_expr))
            }
            (BasicTypeEnum::IntType(a), BasicTypeEnum::IntType(b))
                if a.get_bit_width() > b.get_bit_width() =>
            {
                cast_fn(
                    inkwell::values::InstructionOpcode::ZExt,
                    right_expr,
                    left_type,
                )
                .map(|v| (left_expr, v))
            }
            (BasicTypeEnum::FloatType(a), BasicTypeEnum::FloatType(b))
                if self.get_float_size(a) < self.get_float_size(b) =>
            {
                cast_fn(
                    inkwell::values::InstructionOpcode::FPExt,
                    left_expr,
                    right_type,
                )
                .map(|v| (v, right_expr))
            }
            (BasicTypeEnum::FloatType(a), BasicTypeEnum::FloatType(b))
                if self.get_float_size(a) > self.get_float_size(b) =>
            {
                cast_fn(
                    inkwell::values::InstructionOpcode::FPTrunc,
                    right_expr,
                    left_type,
                )
                .map(|v| (left_expr, v))
            }
            _ => todo!("impl_cast_simple_expr {:?} to {:?}", left_type, right_type),
        }
    }

    fn impl_cast_expr(
        &self,
        left_expr: BasicValueEnum<'ctx>,
        cast_to: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, CodeGenError> {
        let left_type = left_expr.get_type();

        let cast_fn = |op| {
            self.builder
                .build_cast(op, left_expr, cast_to, "")
                .map_err(CodeGenError::from_llvm_err)
        };

        match (left_type, cast_to) {
            _ if left_type == cast_to => Ok(left_expr),
            (BasicTypeEnum::IntType(_), BasicTypeEnum::FloatType(_)) => {
                cast_fn(inkwell::values::InstructionOpcode::SIToFP)
            }
            (BasicTypeEnum::FloatType(_), BasicTypeEnum::IntType(_)) => {
                cast_fn(inkwell::values::InstructionOpcode::FPToSI)
            }
            (BasicTypeEnum::IntType(a), BasicTypeEnum::IntType(b))
                if a.get_bit_width() < b.get_bit_width() =>
            {
                cast_fn(inkwell::values::InstructionOpcode::ZExt)
            }
            (BasicTypeEnum::IntType(a), BasicTypeEnum::IntType(b))
                if a.get_bit_width() > b.get_bit_width() =>
            {
                cast_fn(inkwell::values::InstructionOpcode::Trunc)
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_impl_expr() {
        let data = "func main() { let u32 a = 1 + 2 * 3 - 10 / 5 }";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define void @main() {
entry:
  %a = alloca i32, align 4
  store i32 5, ptr %a, align 4
  ret void
}
"#
        )
    }

    #[test]
    fn test_impl_expr_with_vars() {
        let data = "func main() u32 { 
    let u32 a = 1
    let u32 b = 2
    let u32 c = a + b
    return c
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 1, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 2, ptr %b, align 4
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %0 = add i32 %a1, %b2
  %c = alloca i32, align 4
  store i32 %0, ptr %c, align 4
  %c3 = load i32, ptr %c, align 4
  ret i32 %c3
}
"#
        )
    }

    #[test]
    fn test_impl_cast_expr() {
        let data = "func main() i32 { 
    let f32 a = 3.24
    let i32 b = a -> i32
    return b
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca float, align 4
  store float 0x4009EB8520000000, ptr %a, align 4
  %a1 = load float, ptr %a, align 4
  %0 = fptosi float %a1 to i32
  %b = alloca i32, align 4
  store i32 %0, ptr %b, align 4
  %b2 = load i32, ptr %b, align 4
  ret i32 %b2
}
"#
        )
    }

    #[test]
    fn test_impl_cast_expr_with_ints() {
        let data = "func main() i32 {
    let i32 a = 8
    let i64 b = 3
    let i32 c = a + b -> i32
    let i64 d = c -> i64 + b
    return d -> i32
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 8, ptr %a, align 4
  %b = alloca i64, align 8
  store i64 3, ptr %b, align 4
  %a1 = load i32, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %0 = trunc i64 %b2 to i32
  %1 = add i32 %a1, %0
  %c = alloca i32, align 4
  store i32 %1, ptr %c, align 4
  %c3 = load i32, ptr %c, align 4
  %2 = zext i32 %c3 to i64
  %b4 = load i64, ptr %b, align 4
  %3 = add i64 %2, %b4
  %d = alloca i64, align 8
  store i64 %3, ptr %d, align 4
  %d5 = load i64, ptr %d, align 4
  %4 = trunc i64 %d5 to i32
  ret i32 %4
}
"#
        )
    }

    #[test]
    fn test_impl_cast_expr_with_brackets() {
        let data = "func main() i32 {
    let i32 a = 3
    let i32 b = 7
    let i32 c = 10
    
    let i64 d = (a + b + c) -> i64
    return d -> i32
}";
        let result = crate::get_codegen_for_string(data).unwrap();

        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 3, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 7, ptr %b, align 4
  %c = alloca i32, align 4
  store i32 10, ptr %c, align 4
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %0 = add i32 %a1, %b2
  %c3 = load i32, ptr %c, align 4
  %1 = add i32 %0, %c3
  %2 = zext i32 %1 to i64
  %d = alloca i64, align 8
  store i64 %2, ptr %d, align 4
  %d4 = load i64, ptr %d, align 4
  %3 = trunc i64 %d4 to i32
  ret i32 %3
}
"#
        )
    }

    #[test]
    fn test_bitwise_operations() {
        let data = "func main() i32 {
    let i32 a = 3
    let i32 b = 7

    return a | b ^ b << 4 >> 2
}";
        let result = crate::get_codegen_for_string(data).unwrap();
        assert_eq!(
            result,
            r#"; ModuleID = 'main'
source_filename = "main"

define i32 @main() {
entry:
  %a = alloca i32, align 4
  store i32 3, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 7, ptr %b, align 4
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %b3 = load i32, ptr %b, align 4
  %0 = shl i32 %b3, 4
  %1 = lshr i32 %0, 2
  %2 = xor i32 %b2, %1
  %3 = or i32 %a1, %2
  ret i32 %3
}
"#
        )
    }
}
