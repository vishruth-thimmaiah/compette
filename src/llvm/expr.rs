use inkwell::values::BasicValueEnum;

use crate::{
    lexer::types::{DATATYPE, OPERATOR},
    parser::{
        nodes::{ExpressionParserNode, FunctionCallParserNode, ParserType, ValueParserNode},
        types::ParserTypes,
    },
};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    /// Handles any operations, breaking down more complex structures into BasicValueEnums.
    // pub fn add_expression(
    //     &self,
    //     node: &ExpressionParserNode,
    //     func_name: &str,
    //     req_type: &DATATYPE,
    // ) -> BasicValueEnum<'ctx> {
    //     if let DATATYPE::CUSTOM(name) = req_type {
    //         let iter_node = node
    //             .left
    //             .any()
    //             .downcast_ref::<ValueIterParserNode>()
    //             .unwrap();
    //         return self.create_struct(name, iter_node).into();
    //     }
    //     let left_val = match node.left.get_type() {
    //         ParserTypes::VALUE_ITER_CALL => {
    //             let iter_node = node
    //                 .left
    //                 .any()
    //                 .downcast_ref::<ValueIterCallParserNode>()
    //                 .unwrap();
    //             let array = self.get_array_val(iter_node, func_name, req_type);
    //             self.builder
    //                 .build_load(self.def_expr(req_type).unwrap(), array, "")
    //                 .unwrap()
    //         }
    //         ParserTypes::VALUE_ITER => {
    //             let iter_node = node
    //                 .left
    //                 .any()
    //                 .downcast_ref::<ValueIterParserNode>()
    //                 .unwrap();
    //             self.add_array(iter_node, func_name, req_type)
    //         }
    //         ParserTypes::VALUE => {
    //             let val_node = node.left.any().downcast_ref::<ValueParserNode>().unwrap();
    //             self.add_value(val_node, func_name, req_type)
    //         }
    //         ParserTypes::FUNCTION_CALL => {
    //             let func_node = node
    //                 .left
    //                 .any()
    //                 .downcast_ref::<FunctionCallParserNode>()
    //                 .unwrap();
    //
    //             self.add_func_call(func_node, func_name)
    //         }
    //         _ => errors::compiler_error(&format!("Invalid type: {:?}", node.left.get_type())),
    //     };
    //
    //     let right_val = {
    //         if let Some(right) = &node.right {
    //             let right_expr = right.any().downcast_ref::<ExpressionParserNode>().unwrap();
    //             if node.operator.as_ref().unwrap() == &OPERATOR::DOT {
    //                 let field_name = right_expr
    //                     .left
    //                     .any()
    //                     .downcast_ref::<ValueParserNode>()
    //                     .unwrap();
    //                 return self.index_struct("t", &field_name.value, func_name).into();
    //             }
    //             self.add_expression(right_expr, func_name, req_type)
    //         } else {
    //             return left_val;
    //         }
    //     };
    //
    //     match node.operator.as_ref().unwrap() {
    //         OPERATOR::PLUS => self.add_binary_operation(&left_val, &right_val),
    //         OPERATOR::MINUS => self.sub_binary_operation(&left_val, &right_val),
    //         OPERATOR::MULTIPLY => self.mul_binary_operation(&left_val, &right_val),
    //         OPERATOR::DIVIDE => self.div_binary_operation(&left_val, &right_val),
    //         OPERATOR::EQUAL => self.comp_binary_operation(OPERATOR::EQUAL, &left_val, &right_val),
    //         OPERATOR::NOT_EQUAL => {
    //             self.comp_binary_operation(OPERATOR::NOT_EQUAL, &left_val, &right_val)
    //         }
    //         OPERATOR::GREATER => {
    //             self.comp_binary_operation(OPERATOR::GREATER, &left_val, &right_val)
    //         }
    //         OPERATOR::LESSER => self.comp_binary_operation(OPERATOR::LESSER, &left_val, &right_val),
    //         OPERATOR::GREATER_EQUAL => {
    //             self.comp_binary_operation(OPERATOR::GREATER_EQUAL, &left_val, &right_val)
    //         }
    //         OPERATOR::LESSER_EQUAL => {
    //             self.comp_binary_operation(OPERATOR::LESSER_EQUAL, &left_val, &right_val)
    //         }
    //         _ => unreachable!(),
    //     }
    // }

    pub fn add_expression(
        &self,
        node: &ExpressionParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
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
            _ => todo!(),
        }
    }

    fn add_expr_hand(
        &self,
        node: &Box<dyn ParserType>,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let right_expr = match node.get_type() {
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
            _ => todo!(),
        };
        right_expr
    }
}
