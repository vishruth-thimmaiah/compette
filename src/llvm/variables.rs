use inkwell::{
    types::VectorType,
    values::{ArrayValue, BasicValueEnum, PointerValue},
};

use crate::{
    errors,
    lexer::types::{Types, DATATYPE, OPERATOR},
    parser::{
        nodes::{
            AssignmentParserNode, ExpressionParserNode, FunctionCallParserNode,
            ValueIterCallParserNode, ValueIterParserNode, ValueParserNode, VariableCallParserNode,
        },
        types::ParserTypes,
    },
};

use super::codegen::{CodeGen, VariableStore};

impl<'ctx> CodeGen<'ctx> {
    pub fn add_variable(&self, func_name: &str, node: &AssignmentParserNode) {
        let ptr = self.new_ptr(node);
        self.variables.borrow_mut().iter_mut().for_each(|x| {
            if x.name == func_name {
                x.args.insert(
                    node.var_name.clone(),
                    VariableStore {
                        ptr,
                        is_mutable: node.is_mutable,
                        datatype: node.var_type.clone(),
                    },
                );
            }
        });

        let value = node
            .value
            .any()
            .downcast_ref::<ExpressionParserNode>()
            .unwrap();

        let possible_iter_node = value.left.any().downcast_ref::<ValueIterParserNode>();
        let expr = if node.is_mutable && possible_iter_node.is_some() {
            self.add_vec(possible_iter_node.unwrap(), func_name, &node.var_type)
        } else {
            self.add_expression(value, func_name, &node.var_type)
        };

        self.builder.build_store(ptr, expr).unwrap();
    }

    pub fn mod_variable(&self, func_name: &str, node: &VariableCallParserNode) {
        let variables = self.variables.borrow();
        let func = variables.iter().find(|x| x.name == func_name).unwrap();

        let var_name = if let Some(name) = node
            .var_name
            .any()
            .downcast_ref::<ValueIterCallParserNode>()
        {
            &name.value
        } else {
            &node
                .var_name
                .any()
                .downcast_ref::<ValueParserNode>()
                .unwrap()
                .value
        };
        let variable = func.args.get(var_name).expect("Variable not found");

        if !variable.is_mutable {
            errors::compiler_error("Cannot modify immutable variable");
        }

        let (var_ptr, datatype) = if node.var_name.get_type() == ParserTypes::VALUE_ITER_CALL {
            let datatype = if let DATATYPE::ARRAY(array_type) = &variable.datatype {
                &array_type.datatype
            } else {
                unreachable!()
            };
            (
                self.get_array_val(
                    node.var_name
                        .any()
                        .downcast_ref::<ValueIterCallParserNode>()
                        .unwrap(),
                    func_name,
                    datatype,
                ),
                datatype,
            )
        } else {
            (variable.ptr, &variable.datatype)
        };

        let expr = self.add_expression(&node.rhs, func_name, datatype);

        self.builder.build_store(var_ptr, expr).unwrap();
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
            errors::compiler_error("Expected array type")
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

    pub fn add_vec(
        &self,
        node: &ValueIterParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let vec_type = if let DATATYPE::ARRAY(array_type) = req_type {
            array_type
        } else {
            errors::compiler_error("Expected vec type")
        };

        let mut vec_val = vec![];
        for value in &node.value {
            let value = self.add_expression(&value, func_name, &vec_type.datatype);
            vec_val.push(value);
        }

        VectorType::const_vector(&vec_val).into()
    }

    pub fn get_array_val(
        &self,
        node: &ValueIterCallParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> PointerValue<'ctx> {
        let vars = self.variables.borrow();
        let var_name = vars
            .iter()
            .find(|x| x.name == func_name)
            .unwrap()
            .args
            .get(&node.value)
            .unwrap()
            .ptr;

        let val = &[self
            .add_expression(&node.index, func_name, req_type)
            .into_int_value()];
        let array_type = self.def_expr(req_type);

        unsafe {
            self.builder
                .build_in_bounds_gep(array_type, var_name, val, "")
                .unwrap()
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
                    .find(|x| x.name == func_name)
                    .unwrap()
                    .args
                    .get(node.value.as_str());
                let res = {
                    if let Some(var_name) = var_name {
                        self.builder
                            .build_load(self.def_expr(req_type), var_name.ptr, &node.value)
                            .unwrap()
                    } else if let Some(func) = self.module.get_function(func_name) {
                        func.get_params()
                            .iter()
                            .find(|x| x.get_name().to_str().unwrap() == node.value)
                            .unwrap()
                            .to_owned()
                    } else {
                        errors::compiler_error("Invalid type");
                    }
                };
                res
            }
            _ => errors::compiler_error("Invalid type"),
        }
    }

    pub fn add_expression(
        &self,
        node: &ExpressionParserNode,
        func_name: &str,
        req_type: &DATATYPE,
    ) -> BasicValueEnum<'ctx> {
        let left_val = match node.left.get_type() {
            ParserTypes::VALUE_ITER_CALL => {
                let iter_node = node
                    .left
                    .any()
                    .downcast_ref::<ValueIterCallParserNode>()
                    .unwrap();
                let array = self.get_array_val(iter_node, func_name, req_type);
                self.builder
                    .build_load(self.def_expr(req_type), array, "")
                    .unwrap()
            }
            ParserTypes::VALUE_ITER => {
                let iter_node = node
                    .left
                    .any()
                    .downcast_ref::<ValueIterParserNode>()
                    .unwrap();
                self.add_array(iter_node, func_name, req_type)
            }
            ParserTypes::VALUE => {
                let val_node = node.left.any().downcast_ref::<ValueParserNode>().unwrap();
                self.add_value(val_node, func_name, req_type)
            }
            ParserTypes::FUNCTION_CALL => {
                let func_node = node
                    .left
                    .any()
                    .downcast_ref::<FunctionCallParserNode>()
                    .unwrap();

                self.add_func_call(func_node, func_name)
            }
            _ => errors::compiler_error(&format!("Invalid type: {:?}", node.left.get_type())),
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
