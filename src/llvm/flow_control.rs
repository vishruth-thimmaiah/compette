use crate::{
    lexer::types::DATATYPE,
    parser::{
        nodes::{ConditionalIfParserNode, LoopParserNode, ParserType},
        types::ParserTypes,
    },
};

use super::codegen::CodeGen;

impl<'ctx> CodeGen<'ctx> {
    pub fn add_conditional_if(&self, func_name: &str, node: &ConditionalIfParserNode) {
        let function = self.module.get_function(func_name).unwrap();
        let if_block = self.context.append_basic_block(function, "if");

        let cont = self.context.append_basic_block(function, "if_cont");

        let mut prev_block = (if_block, &node.condition);
        let mut else_if_blocks = Vec::new();

        for (index, else_if_cond) in node.else_if_body.iter().enumerate() {
            let c_name = &("cond_".to_string() + &index.to_string());
            let b_name = &("else_if_".to_string() + &index.to_string());
            let cond_eval_block = self.context.append_basic_block(function, c_name);

            let expr = self.add_expression(prev_block.1, func_name, &DATATYPE::U32);

            self.builder
                .build_conditional_branch(
                    self.to_bool(&expr).into_int_value(),
                    prev_block.0,
                    cond_eval_block,
                )
                .unwrap();

            let cond_block = self.context.append_basic_block(function, b_name);
            else_if_blocks.push(cond_block);
            self.builder.position_at_end(cond_block);
            self.nested_codegen(&else_if_cond.body, func_name, &DATATYPE::U32);

            self.add_unconditional(else_if_cond.body.last(), cont);

            self.builder.position_at_end(cond_eval_block);

            prev_block = (cond_block, &else_if_cond.condition);
        }

        let last_block = if let Some(else_body) = &node.else_body {
            let else_block = self.context.append_basic_block(function, "else");
            self.builder.position_at_end(else_block);
            self.nested_codegen(&else_body.body, func_name, &DATATYPE::U32);

            self.add_unconditional(else_body.body.last(), cont);

            else_block
        } else if prev_block.0 == if_block {
            cont
        } else {
            prev_block.0
        };

        self.builder
            .position_at_end(prev_block.0.get_previous_basic_block().unwrap());

        let expr = self.add_expression(prev_block.1, func_name, &DATATYPE::U32);

        self.builder
            .build_conditional_branch(
                self.to_bool(&expr).into_int_value(),
                prev_block.0,
                last_block,
            )
            .unwrap();

        self.builder.position_at_end(if_block);
        self.nested_codegen(&node.body, func_name, &DATATYPE::U32);

        self.add_unconditional(node.body.last(), cont);

        cont.move_after(last_block).unwrap();
        self.builder.position_at_end(cont);
    }

    pub fn add_unconditional(
        &self,
        last_item: Option<&Box<dyn ParserType>>,
        move_to: inkwell::basic_block::BasicBlock,
    ) {
        if let Some(last) = last_item {
            if last.get_type() == ParserTypes::RETURN {
                return;
            }
        }
        self.builder.build_unconditional_branch(move_to).unwrap();
    }

    pub fn add_loop(&self, func_name: &str, node: &LoopParserNode) {
        let function = self.module.get_function(func_name).unwrap();

        let loop_block = self.context.append_basic_block(function, "loop");
        let cont = self.context.append_basic_block(function, "loop_cont");

        let expr = self.add_expression(&node.condition, func_name, &DATATYPE::U32);

        self.builder
            .build_conditional_branch(self.to_bool(&expr).into_int_value(), loop_block, cont)
            .unwrap();

        self.builder.position_at_end(loop_block);
        self.nested_codegen(&node.body, func_name, &DATATYPE::U32);

        let expr = self.add_expression(&node.condition, func_name, &DATATYPE::U32);
        self.builder
            .build_conditional_branch(self.to_bool(&expr).into_int_value(), loop_block, cont)
            .unwrap();
        self.builder.position_at_end(cont);
    }
}
