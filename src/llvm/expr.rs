use std::cmp::Ordering;

use inkwell::values::BasicValueEnum;

use crate::{
    lexer::types::{Types, DATATYPE, OPERATOR},
    parser::{
        nodes::{
            ExpressionParserNode, FunctionCallParserNode, ParserType, ValueIterCallParserNode,
            ValueParserNode,
        },
        types::ParserTypes,
    },
};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn add_expression(
        &self,
        node: &ExpressionParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        if node.operator.is_some() && node.operator.as_ref().unwrap() == &OPERATOR::DOT {
            let obj_name = node
                .left
                .any()
                .downcast_ref::<ValueParserNode>()
                .unwrap();
            let right = node.right.as_ref().unwrap();
            if right.get_type() == ParserTypes::VALUE {
                let field_name = &right.any().downcast_ref::<ValueParserNode>().unwrap().value;
                return self.index_struct(&obj_name.value, &field_name, func_name);
            } else if right.get_type() == ParserTypes::FUNCTION_CALL {
                let func_call = right
                    .any()
                    .downcast_ref::<FunctionCallParserNode>()
                    .unwrap();
                return self.add_method_call(obj_name, func_call, func_name);
            }
        }

        let left_expr = self.add_expr_hand(&node.left, func_name, req_type);

        if node.right.is_none() {
            return left_expr;
        };

        if node.operator.as_ref().unwrap() == &OPERATOR::CAST {
            return self.cast_expr(
                &left_expr,
                &node
                    .right
                    .as_ref()
                    .unwrap()
                    .any()
                    .downcast_ref::<ValueParserNode>()
                    .unwrap(),
            );
        }

        let right_expr = self.add_expr_hand(node.right.as_ref().unwrap(), func_name, req_type);

        let (left_expr, right_expr) = self.impl_cast_expr(left_expr, right_expr);

        let expr = match node.operator.as_ref().unwrap() {
            OPERATOR::PLUS => self.add_binary_operation(&left_expr, &right_expr),
            OPERATOR::MINUS => self.sub_binary_operation(&left_expr, &right_expr),
            OPERATOR::MULTIPLY => self.mul_binary_operation(&left_expr, &right_expr),
            OPERATOR::DIVIDE => self.div_binary_operation(&left_expr, &right_expr),
            OPERATOR::EQUAL
            | OPERATOR::NOT_EQUAL
            | OPERATOR::GREATER
            | OPERATOR::GREATER_EQUAL
            | OPERATOR::LESSER
            | OPERATOR::LESSER_EQUAL => self.comp_binary_operation(
                node.operator.as_ref().unwrap().clone(),
                &left_expr,
                &right_expr,
            ),
            _ => todo!(),
        };
        expr
    }

    fn add_expr_hand(
        &self,
        node: &Box<dyn ParserType>,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let expr = match node.get_type() {
            ParserTypes::EXPRESSION => self.add_expression(
                node.any().downcast_ref::<ExpressionParserNode>().unwrap(),
                func_name,
                req_type,
            ),
            ParserTypes::VALUE => self.add_value(
                node.any().downcast_ref::<ValueParserNode>().unwrap(),
                func_name,
                req_type,
            ),
            ParserTypes::FUNCTION_CALL => self.add_func_call(
                node.any().downcast_ref::<FunctionCallParserNode>().unwrap(),
                func_name,
            ),
            ParserTypes::VALUE_ITER_CALL => {
                let iter_node = node
                    .any()
                    .downcast_ref::<ValueIterCallParserNode>()
                    .unwrap();
                let (array, dt) = self.get_array_val(iter_node, func_name);
                self.builder
                    .build_load(self.def_expr(&dt).unwrap(), array, "")
                    .unwrap()
            }
            _ => todo!(),
        };
        expr
    }

    fn impl_cast_expr(
        &self,
        left_expr: BasicValueEnum<'ctx>,
        right_expr: BasicValueEnum<'ctx>,
    ) -> (BasicValueEnum<'ctx>, BasicValueEnum<'ctx>) {
        let left_type = left_expr.get_type();
        let right_type = right_expr.get_type();
        if left_type == right_type {
            return (left_expr, right_expr);
        } else if left_type.is_int_type() && right_type.is_int_type() {
            let bigger = left_type
                .into_int_type()
                .get_bit_width()
                .cmp(&right_type.into_int_type().get_bit_width());
            if bigger == Ordering::Less {
                let new_expr = self
                    .builder
                    .build_cast(
                        inkwell::values::InstructionOpcode::ZExt,
                        left_expr,
                        right_type,
                        "",
                    )
                    .unwrap();
                return (new_expr, right_expr);
            } else {
                let new_expr = self
                    .builder
                    .build_cast(
                        inkwell::values::InstructionOpcode::ZExt,
                        right_expr,
                        left_type,
                        "",
                    )
                    .unwrap();
                return (left_expr, new_expr);
            }
        } else if left_type.is_float_type() && right_type.is_float_type() {
            todo!()
        } else {
            todo!()
        }
    }

    fn cast_expr(
        &self,
        left_expr: &BasicValueEnum<'ctx>,
        cast_to: &ValueParserNode,
    ) -> BasicValueEnum<'ctx> {
        let left_type = left_expr.get_type();

        let cast_to = if let Types::DATATYPE(cast_to) = cast_to.r#type.clone() {
            cast_to
        } else {
            unreachable!()
        };

        let cast_to_type = self.def_expr(&cast_to).unwrap();
        if left_type.is_int_type() && cast_to == DATATYPE::F64 || cast_to == DATATYPE::F32 {
            self.builder
                .build_cast(
                    inkwell::values::InstructionOpcode::SIToFP,
                    *left_expr,
                    cast_to_type,
                    "",
                )
                .unwrap()
        } else if left_type.is_float_type() && self.def_expr(&cast_to).unwrap().is_int_type() {
            self.builder
                .build_cast(
                    inkwell::values::InstructionOpcode::FPToSI,
                    *left_expr,
                    cast_to_type,
                    "",
                )
                .unwrap()
        } else {
            todo!()
        }
    }
}
