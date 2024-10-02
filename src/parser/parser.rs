use std::process::exit;

use crate::lexer::{lexer::Token, types::Types};

use super::nodes::{
    AssignmentParserNode, ConditionalElseIfParserNode, ConditionalElseParserNode,
    ConditionalIfParserNode, ExpressionParserNode, FunctionCallParserNode, FunctionParserNode,
    ParserType,
};

pub struct Parser {
    tree: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(lexer_tokens: Vec<Token>) -> Self {
        Self {
            tree: lexer_tokens,
            position: 0,
        }
    }

    fn get_prev_token(&self) -> Token {
        if self.position == 0 {
            return Token::default();
        }
        self.tree
            .get(self.position - 1)
            .unwrap_or(&Token::default())
            .clone()
    }

    fn get_next_token(&self) -> Token {
        self.tree
            .get(self.position + 1)
            .unwrap_or(&Token::default())
            .clone()
    }

    fn get_current_token(&self) -> Token {
        self.tree
            .get(self.position)
            .unwrap_or(&Token::default())
            .clone()
    }

    fn set_next_position(&mut self) {
        self.position += 1;
    }

    fn handle_error(&self, message: &str) {
        let curr = self.get_current_token();
        let prev = self.get_prev_token();
        let next = self.get_next_token();
        panic!(
            "\nError at line: {}, column: {}\nError type: {}\nstopped at: {:?}, {:?}\nprev: {:?}, {:?},\nnext: {:?}, {:?}\n",
            curr.line+1, curr.column+1, message, curr.r#type, curr.value, prev.r#type, prev.value, next.r#type, next.value
        );
    }

    pub fn parse(&mut self) -> Vec<Box<dyn ParserType>> {
        self.parse_scope()
    }

    fn parse_scope(&mut self) -> Vec<Box<dyn ParserType>> {
        let mut tokens: Vec<Box<dyn ParserType>> = vec![];

        let mut nested = false;

        loop {
            let token_type = self.get_current_token().r#type;
            match token_type {
                Types::NL => (),
                Types::EOF => break,
                Types::LET => tokens.push(self.parse_assignment()),
                Types::IF => tokens.push(self.parse_conditional_if()),
                Types::FUNCTION => tokens.push(self.parse_function()),
                Types::IDENTIFIER => tokens.push(self.parse_function_call()),
                Types::LBRACE => nested = true,
                Types::RBRACE => {
                    if !nested {
                        self.handle_error("Invalid close brace");
                    }
                    break;
                }
                _ => self.handle_error("invalid token"),
            }

            self.position += 1;
        }

        tokens
    }

    fn parse_assignment(&mut self) -> Box<AssignmentParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            self.handle_error("invalid token")
        }

        let var_name = self.get_next_token().value.unwrap();
        self.set_next_position();

        if self.get_next_token().r#type != Types::ASSIGN {
            self.handle_error("invalid token")
        }
        self.set_next_position();

        let value = self.parse_expression();
        self.set_next_position();

        return Box::new(AssignmentParserNode { var_name, value });
    }

    // TODO: Add support for parenthesis
    fn parse_expression(&mut self) -> Box<ExpressionParserNode> {
        let left = self.get_next_token();
        self.set_next_position();

        let operator = self.get_next_token().r#type;
        match operator {
            Types::PLUS
            | Types::MINUS
            | Types::MULTIPLY
            | Types::DIVIDE
            | Types::EQUAL
            | Types::NOT_EQUAL
            | Types::GREATER
            | Types::LESSER
            | Types::GREATER_EQUAL
            | Types::LESSER_EQUAL => {
                self.set_next_position();
                let right = self.parse_expression();
                return Box::new(ExpressionParserNode {
                    left,
                    right: Some(right),
                    operator: Some(operator),
                });
            }
            Types::NL | Types::LBRACE => {
                return Box::new(ExpressionParserNode {
                    left,
                    right: None,
                    operator: None,
                });
            }
            _ => {
                self.handle_error("invalid token");
                exit(1)
            }
        }
    }

    fn parse_function(&mut self) -> Box<FunctionParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            self.handle_error("invalid token")
        }

        let func_name = self.get_next_token().value.unwrap();
        self.set_next_position();

        if self.get_next_token().r#type != Types::LPAREN {
            self.handle_error("invalid token")
        }
        self.set_next_position();

        let mut args: Vec<String> = vec![];
        loop {
            let token = self.get_next_token();
            if token.r#type == Types::RPAREN {
                break;
            }
            if token.r#type == Types::IDENTIFIER {
                args.push(token.value.unwrap());
            }
            self.set_next_position();
        }

        self.set_next_position();

        if self.get_next_token().r#type != Types::LBRACE {
            self.handle_error("invalid token")
        }
        self.set_next_position();

        let body = self.parse_scope();
        self.set_next_position();

        return Box::new(FunctionParserNode {
            func_name,
            args,
            body,
        });
    }

    fn parse_function_call(&mut self) -> Box<FunctionCallParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            self.handle_error("invalid token")
        }

        let func_name = self.get_current_token().value.unwrap();

        let mut args: Vec<String> = vec![];
        loop {
            let token = self.get_next_token();
            if token.r#type == Types::RPAREN {
                break;
            }
            if token.r#type == Types::IDENTIFIER {
                args.push(token.value.unwrap());
            }
            self.set_next_position();
        }
        self.set_next_position();

        return Box::new(FunctionCallParserNode { func_name, args });
    }

    fn parse_conditional_if(&mut self) -> Box<ConditionalIfParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            self.handle_error("invalid token")
        }

        let condition = self.parse_expression();
        self.set_next_position();

        let body = self.parse_scope();

        return Box::new(ConditionalIfParserNode {
            condition,
            body,
            else_if_body: self.parse_conditional_else_if(),
            else_body: self.parse_conditional_else(),
        });
    }

    fn parse_conditional_else_if(&mut self) -> Vec<ConditionalElseIfParserNode> {
        let mut else_if_body = vec![];

        loop {
            let token = self.get_next_token();
            if token.r#type != Types::ELSE {
                break;
            }
            self.set_next_position();

            let token = self.get_next_token();
            if token.r#type != Types::IF {
                break;
            }
            self.set_next_position();

            let condition = self.parse_expression();
            self.set_next_position();

            let body = self.parse_scope();

            else_if_body.push(ConditionalElseIfParserNode { condition, body });
        }

        else_if_body
    }

    fn parse_conditional_else(&mut self) -> Option<ConditionalElseParserNode> {
        if self.get_current_token().r#type != Types::ELSE {
            return None;
        }

        if self.get_next_token().r#type != Types::LBRACE {
            self.handle_error("invalid token")
        }
        self.set_next_position();

        let body = self.parse_scope();

        return Some(ConditionalElseParserNode { body });
    }
}
