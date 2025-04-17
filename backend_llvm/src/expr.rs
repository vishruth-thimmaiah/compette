use inkwell::{
    types::{BasicType, BasicTypeEnum},
    values::BasicValueEnum,
};
use lexer::types::{Operator, Types};
use new_parser::nodes::{ASTNodes, Expression, Literal, Variable};

use crate::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub(crate) fn impl_expr(
        &self,
        node: &Expression,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, ()> {
        match node {
            Expression::Simple {
                left,
                right,
                operator,
            } => {
                let left_val = self.impl_simple_expr_arm(left, dt)?;

                if let Some(right_val) = right {
                    let right_val = self.impl_simple_expr_arm(right_val, dt)?;
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
                    array_val.push(self.impl_expr(value, inner_dt)?);
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
                        self.impl_expr(val, dt.get_field_type_at_index(field as u32).unwrap())
                            .unwrap(),
                    );
                }
                let struct_vals = struct_vals
                    .into_iter()
                    .map(|v| v.unwrap())
                    .collect::<Vec<_>>();
                Ok(dt.const_named_struct(&struct_vals).into())
            }
            _ => todo!(),
        }
    }

    fn impl_simple_expr_arm(
        &self,
        arm: &ASTNodes,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, ()> {
        match arm {
            ASTNodes::Literal(lit) => self.impl_literal(lit, dt),
            ASTNodes::Variable(var) => self.impl_variable(var, dt),
            ASTNodes::Expression(expr) => self.impl_expr(expr, dt),
            _ => todo!("Simple expr arm {:?}", arm),
        }
    }

    fn impl_binary_operation(
        &self,
        left_val: BasicValueEnum<'ctx>,
        right_val: BasicValueEnum<'ctx>,
        operator: &Operator,
    ) -> Result<BasicValueEnum<'ctx>, ()> {
        match operator {
            Operator::PLUS => self.add_binary_operation(&left_val, &right_val),
            Operator::MINUS => self.sub_binary_operation(&left_val, &right_val),
            Operator::MULTIPLY => self.mul_binary_operation(&left_val, &right_val),
            Operator::DIVIDE => self.div_binary_operation(&left_val, &right_val),
            Operator::EQUAL
            | Operator::NOT_EQUAL
            | Operator::GREATER
            | Operator::GREATER_EQUAL
            | Operator::LESSER
            | Operator::LESSER_EQUAL => self.comp_binary_operation(operator, &left_val, &right_val),
            _ => todo!("Binary operator {:?} not implemented", operator),
        }
    }

    fn impl_literal(
        &self,
        lit: &Literal,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, ()> {
        match lit.r#type {
            Types::BOOL => Ok(self
                .context
                .bool_type()
                .const_int(lit.value.parse::<u64>().unwrap(), false)
                .into()),
            Types::NUMBER => {
                if dt.is_float_type() {
                    let f64_value = lit.value.parse::<f64>().unwrap();
                    return Ok(dt.into_float_type().const_float(f64_value).into());
                } else if dt.is_int_type() {
                    let i64_value = lit.value.parse::<u64>().unwrap();
                    return Ok(dt.into_int_type().const_int(i64_value, false).into());
                }
                unreachable!()
            }
            _ => todo!(),
        }
    }

    fn impl_variable(
        &self,
        var: &Variable,
        dt: BasicTypeEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, ()> {
        let ptr = self.var_ptrs.get(&var.name).ok_or(())?;
        self.builder.build_load(dt, ptr, &var.name).map_err(|_| ())
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
}
