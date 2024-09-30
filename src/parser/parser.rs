use crate::lexer::{lexer::Token, types::Types};

use super::nodes::{
    AssignmentParserNode, ExpressionParserNode, FunctionCallParserNode, FunctionParserNode,
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

    pub fn parse(&mut self) -> Vec<Box<dyn ParserType>> {
        let mut tokens: Vec<Box<dyn ParserType>> = vec![];

        loop {
            let token_type = &self.tree[self.position].r#type;
            match token_type {
                Types::NL => (),
                Types::EOF => break,
                Types::LET => tokens.push(Self::parse_assignment(self)),
                Types::FUNCTION => tokens.push(Self::parse_function(self)),
                Types::IDENTIFIER => tokens.push(Self::parse_function_call(self)),
                _ => panic!("Invalid token: {:?}\n Current: {:#?}", token_type, tokens),
            }

            self.position += 1;
        }

        tokens
    }

    fn parse_assignment(&mut self) -> Box<AssignmentParserNode> {
        if Self::get_prev_token(&self).r#type != Types::NL {
            panic!("Invalid token: {:?}", Self::get_prev_token(&self));
        }

        let var_name = Self::get_next_token(&self).value.unwrap();
        Self::set_next_position(self);

        if Self::get_next_token(&self).r#type != Types::ASSIGN {
            panic!("Invalid token");
        }
        Self::set_next_position(self);

        let value = Self::parse_expression(self);
        Self::set_next_position(self);

        return Box::new(AssignmentParserNode { var_name, value });
    }

    // TODO: Add support for parenthesis
    fn parse_expression(&mut self) -> Box<ExpressionParserNode> {
        let left = Self::get_next_token(&self);
        Self::set_next_position(self);

        let operator = Self::get_next_token(&self).r#type;
        match operator {
            Types::PLUS | Types::MINUS | Types::MULTIPLY | Types::DIVIDE => {
                Self::set_next_position(self);
                let right = Self::parse_expression(self);
                return Box::new(ExpressionParserNode {
                    left,
                    right: Some(right),
                    operator: Some(operator),
                });
            }
            Types::NL => {
                return Box::new(ExpressionParserNode {
                    left,
                    right: None,
                    operator: None,
                });
            }
            _ => panic!("Invalid token"),
        }
    }

    fn parse_function(&mut self) -> Box<FunctionParserNode> {
        if Self::get_prev_token(&self).r#type != Types::NL {
            panic!("Invalid token");
        }

        let func_name = Self::get_next_token(&self).value.unwrap();
        Self::set_next_position(self);

        if Self::get_next_token(&self).r#type != Types::LPAREN {
            panic!("Invalid token");
        }
        Self::set_next_position(self);

        let mut args: Vec<String> = vec![];
        loop {
            let token = Self::get_next_token(&self);
            if token.r#type == Types::RPAREN {
                break;
            }
            if token.r#type == Types::IDENTIFIER {
                args.push(token.value.unwrap());
            }
            Self::set_next_position(self);
        }

        Self::set_next_position(self);

        if Self::get_next_token(&self).r#type != Types::LBRACE {
            panic!("Invalid token");
        }
        Self::set_next_position(self);

        let body: Vec<Box<dyn ParserType>> = vec![];

        //TODO: Add support for nested functions
        loop {
            let token = Self::get_next_token(&self);
            if token.r#type == Types::RBRACE {
                break;
            }
            Self::set_next_position(self);
        }
        Self::set_next_position(self);

        return Box::new(FunctionParserNode {
            func_name,
            args,
            body,
        });
    }

    fn parse_function_call(&mut self) -> Box<FunctionCallParserNode> {
        if Self::get_prev_token(&self).r#type != Types::NL {
            panic!("Invalid token");
        }

        let func_name = Self::get_current_token(&self).value.unwrap();

        let mut args: Vec<String> = vec![];
        loop {
            let token = Self::get_next_token(&self);
            if token.r#type == Types::RPAREN {
                break;
            }
            if token.r#type == Types::IDENTIFIER {
                args.push(token.value.unwrap());
            }
            Self::set_next_position(self);
        }
        Self::set_next_position(self);

        return Box::new(FunctionCallParserNode { func_name, args });
    }
}
