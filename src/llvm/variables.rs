use inkwell::values::{ArrayValue, BasicValueEnum};

use crate::{
    lexer::types::{Types, DATATYPE, OPERATOR},
    parser::{
        nodes::{
            AssignmentParserNode, ExpressionParserNode, FunctionCallParserNode,
            ValueIterCallParserNode, ValueIterParserNode, ValueParserNode, VariableCallParserNode,
        },
        types::ParserTypes,
    },
};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn add_variable(&self, func_name: &str, node: &AssignmentParserNode) {
        let alloc = self.new_ptr(node);
        self.variables.borrow_mut().iter_mut().for_each(|x| {
            if x.name == func_name {
                x.args
                    .insert(node.var_name.clone(), (alloc, node.is_mutable));
            }
        });

        let value = node
            .value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();

        self.builder
            .build_store(alloc, self.add_expression(value, func_name, &node.var_type))
            .unwrap();
    }

    pub fn mod_variable(&self, func_name: &str, node: &VariableCallParserNode) {
        let variables = self.variables.borrow();
        let func = variables.iter().find(|x| x.name == func_name).unwrap();

        let variable = func.args.get(&node.var_name).expect("Variable not found");

        if !variable.1 {
            panic!("Cannot modify immutable variable");
        }

        let variable = variable.0;

        let expr = self.add_expression(&node.rhs, func_name, &DATATYPE::U32);

        self.builder.build_store(variable, expr).unwrap();
    }

    pub fn add_array(
        &self,
        node: &ValueIterParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let array_type = if let DATATYPE::ARRAY(array_type) = req_type {
            array_type
        } else {
            panic!("Expected array type")
        };

        let mut array_val = vec![];

        for value in &node.value {
            let value = self.add_expression(&value, func_name, &array_type.datatype);
            array_val.push(value);
        }

        // Figure out how to do this without unsafe
        let array = unsafe {
            ArrayValue::new_const_array(&self.def_expr(&array_type.datatype), &array_val)
        };

        array.into()
    }

    pub fn get_array_val(
        &self,
        node: &ValueIterCallParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let vars = self.variables.borrow();
        let var_name = vars
            .iter()
            .filter(|x| x.name == func_name)
            .collect::<Vec<&_>>()[0]
            .args
            .get(&node.value)
            .unwrap()
            .0;

        unsafe {
            let val = &[self.add_expression(&node.index, func_name, req_type).into_int_value()];
            let array_type = self.def_expr(req_type);

            let ptr = self
                .builder
                .build_in_bounds_gep(array_type, var_name, val, "")
                .unwrap();
            self.builder.build_load(array_type, ptr, "").unwrap()
        }
    }

    pub fn add_value(
        &self,
        node: &ValueParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        match node.r#type {
            Types::NUMBER => self.string_to_value(&node.value, req_type),
            Types::BOOL => self.string_to_value(&node.value, req_type),

            Types::IDENTIFIER => {
                let vars = self.variables.borrow();
                let var_name = vars
                    .iter()
                    .filter(|x| x.name == func_name)
                    .collect::<Vec<&_>>()[0]
                    .args
                    .get(node.value.as_str());
                let res = {
                    if let Some(var_name) = var_name {
                        self.builder
                            .build_load(self.def_expr(req_type), var_name.0, &node.value)
                            .unwrap()
                    } else if let Some(func) = self.module.get_function(func_name) {
                        func.get_params()
                            .iter()
                            .find(|x| x.get_name().to_str().unwrap() == node.value)
                            .unwrap()
                            .to_owned()
                    } else {
                        panic!("Invalid type");
                    }
                };
                res
            }
            _ => panic!("Invalid type"),
        }
    }

    pub fn add_expression(
        &self,
        node: &ExpressionParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let left_val = match node.left.get_type() {
            ParserTypes::VALUE => {
                if let Some(iter) = node.left.any().downcast_ref::<ValueIterParserNode>() {
                    self.add_array(iter, func_name, req_type)
                } else if let Some(iter) = node.left.any().downcast_ref::<ValueIterCallParserNode>()
                {
                    self.get_array_val(iter, func_name, req_type)
                } else {
                    let val_node = node.left.any().downcast_ref::<ValueParserNode>().unwrap();
                    self.add_value(val_node, func_name, req_type)
                }
            }
            ParserTypes::FUNCTION_CALL => {
                let downcast_node = node
                    .left
                    .any()
                    .downcast_ref::<FunctionCallParserNode>()
                    .unwrap();

                let function = self.module.get_function(&downcast_node.func_name).unwrap();
                let mut args = Vec::new();
                let params = function.get_params();
                for (index, arg) in downcast_node.args.iter().enumerate() {
                    args.push(
                        self.add_expression(arg, func_name, self.get_datatype(params[index]))
                            .into(),
                    );
                }

                self.builder
                    .build_call(function, &args, &downcast_node.func_name)
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                    .unwrap()
            }
            _ => panic!("Invalid type"),
        };

        let right_val = {
            if let Some(right) = &node.right {
                let right_expr = right.any().downcast_ref::<ExpressionParserNode>().unwrap();
                self.add_expression(right_expr, func_name, req_type)
            } else {
                return left_val;
            }
        };

        match node.operator.as_ref().unwrap() {
            OPERATOR::PLUS => self.add_binary_operation(&left_val, &right_val),
            OPERATOR::MINUS => self.sub_binary_operation(&left_val, &right_val),
            OPERATOR::MULTIPLY => self.mul_binary_operation(&left_val, &right_val),
            OPERATOR::DIVIDE => self.div_binary_operation(&left_val, &right_val),
            OPERATOR::EQUAL => self.comp_binary_operation(OPERATOR::EQUAL, &left_val, &right_val),
            OPERATOR::NOT_EQUAL => {
                self.comp_binary_operation(OPERATOR::NOT_EQUAL, &left_val, &right_val)
            }
            OPERATOR::GREATER => {
                self.comp_binary_operation(OPERATOR::GREATER, &left_val, &right_val)
            }
            OPERATOR::LESSER => self.comp_binary_operation(OPERATOR::LESSER, &left_val, &right_val),
            OPERATOR::GREATER_EQUAL => {
                self.comp_binary_operation(OPERATOR::GREATER_EQUAL, &left_val, &right_val)
            }
            OPERATOR::LESSER_EQUAL => {
                self.comp_binary_operation(OPERATOR::LESSER_EQUAL, &left_val, &right_val)
            }
            _ => unreachable!(),
        }
    }
}
