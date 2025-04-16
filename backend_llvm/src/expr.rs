use inkwell::{types::BasicTypeEnum, values::BasicValueEnum};
use lexer::types::{Operator, Types};
use new_parser::nodes::{ASTNodes, Expression, Literal};

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
            _ => todo!(),
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
            | Operator::LESSER_EQUAL => self.comp_binary_operation(
                operator,
                &left_val,
                &right_val,
            ),
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
}
