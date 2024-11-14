use inkwell::values::BasicValueEnum;

use crate::{
    lexer::types::{DATATYPE, OPERATOR},
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
            let struct_name = &node
                .left
                .any()
                .downcast_ref::<ValueParserNode>()
                .unwrap()
                .value;
            let field_name = &node
                .right
                .as_ref()
                .unwrap()
                .any()
                .downcast_ref::<ValueParserNode>()
                .unwrap()
                .value;
            return self.index_struct(&struct_name, &field_name, func_name);
        }

        let left_expr = self.add_expr_hand(&node.left, func_name, req_type);

        if node.right.is_none() {
            return left_expr;
        };
        let right_expr = self.add_expr_hand(node.right.as_ref().unwrap(), func_name, req_type);

        match node.operator.as_ref().unwrap() {
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
        }
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
                let array = self.get_array_val(iter_node, func_name, req_type);
                self.builder
                    .build_load(self.def_expr(req_type).unwrap(), array, "")
                    .unwrap()
            }
            _ => todo!(),
        };
        expr
    }
}
