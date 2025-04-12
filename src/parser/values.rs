use crate::{
    errors,
    lexer::types::{ArrayDetails, Types, Datatype, Delimiter, Operator},
};

use super::{
    nodes::{
        AssignmentParserNode, ParserType, StructDefParserNode, StructParserNode,
        ValueIterCallParserNode, ValueIterParserNode, ValueParserNode, VariableCallParserNode,
    },
    Parser,
};

impl Parser {
    pub fn parse_assignment(&mut self) -> Box<AssignmentParserNode> {
        if self.get_prev_token().r#type != Types::NL {
            errors::parser_error(self, "invalid token");
        }

        let mut var_type = match self.get_next_token().r#type {
            Types::DATATYPE(dt) => dt,
            Types::IDENTIFIER => Datatype::CUSTOM(self.get_next_token().value.unwrap()),
            _ => errors::parser_error(self, "invalid token"),
        };
        self.set_next_position();

        let is_array = if self.get_next_token().r#type == Types::DELIMITER(Delimiter::LBRACKET) {
            self.set_next_position();
            if self.get_next_token().r#type != Types::DELIMITER(Delimiter::RBRACKET) {
                errors::parser_error(self, "Invalid array declaration");
            }
            self.set_next_position();
            true
        } else {
            false
        };

        let is_mutable = match self.get_next_token().r#type {
            Types::OPERATOR(Operator::NOT) => {
                self.set_next_position();
                true
            }
            _ => false,
        };

        let var_name = self.get_next_token().value.unwrap();
        self.set_next_position();

        if self.get_next_token().r#type != Types::OPERATOR(Operator::ASSIGN) {
            errors::parser_error(self, "invalid token")
        }
        self.set_next_position();

        let value = self.parse_expression();

        if let Datatype::STRING(_) = var_type {
            let down_cast = value.left.any().downcast_ref::<ValueParserNode>().unwrap();
            if let Types::DATATYPE(Datatype::STRING(len)) = down_cast.r#type {
                var_type = Datatype::STRING(len);
            }
        }

        if is_array {
            let try_downcast = value.left.any().downcast_ref::<ValueIterParserNode>();

            if try_downcast.is_none() {
                errors::parser_error(self, "Invalid array assignment");
            }

            let length = try_downcast.unwrap().value.len() as u32;

            var_type = Datatype::ARRAY(Box::new(ArrayDetails {
                datatype: var_type,
                length,
            }));
        }

        self.set_next_position();

        return Box::new(AssignmentParserNode {
            var_name,
            var_type,
            is_mutable,
            value,
        });
    }

    pub fn parse_array(&mut self) -> Box<ValueIterParserNode> {
        let mut array_contents = vec![];
        while self.get_next_token().r#type != Types::DELIMITER(Delimiter::RBRACKET) {
            array_contents.push(*self.parse_expression());
            if self.get_next_token().r#type == Types::DELIMITER(Delimiter::RBRACKET) {
                break;
            } else if self.get_next_token().r#type != Types::DELIMITER(Delimiter::COMMA) {
                errors::parser_error(self, "Expected comma after array element");
            }
            self.set_next_position();
        }

        self.set_next_position();

        return Box::new(ValueIterParserNode {
            value: array_contents,
        });
    }

    pub fn parse_identifier_call(&mut self) -> Box<VariableCallParserNode> {
        let var_name: Box<dyn ParserType> =
            if self.get_next_token().r#type == Types::DELIMITER(Delimiter::LBRACKET) {
                let name = self.get_current_token().value.unwrap();
                self.set_next_position();
                let val = Box::new(ValueIterCallParserNode {
                    value: name,
                    index: self.parse_expression(),
                });
                self.set_next_position();
                val
            } else {
                Box::new(ValueParserNode {
                    value: self.get_current_token().value.unwrap(),
                    r#type: Types::IDENTIFIER,
                })
            };

        if self.get_next_token().r#type != Types::OPERATOR(Operator::ASSIGN) {
            errors::parser_error(self, "invalid token");
        }
        self.set_next_position();
        return Box::new(VariableCallParserNode {
            var_name,
            rhs: self.parse_expression(),
        });
    }

    pub fn parse_def_struct(&mut self) -> Box<StructDefParserNode> {
        let struct_name = self.get_next_token().value.unwrap();

        self.set_next_position();

        if self.get_next_token().r#type != Types::DELIMITER(Delimiter::LBRACE) {
            errors::parser_error(self, "invalid token")
        }

        self.set_next_position();

        let mut fields: Vec<(String, Datatype)> = vec![];

        while self.get_next_token().r#type != Types::DELIMITER(Delimiter::RBRACE) {
            if self.get_next_token().r#type == Types::NL
                || self.get_next_token().r#type == Types::DELIMITER(Delimiter::COMMA)
            {
                self.set_next_position();
                continue;
            }

            self.set_next_position();

            if self.get_current_token().r#type != Types::IDENTIFIER {
                errors::parser_error(self, "invalid token");
            }

            let field_type = if let Types::DATATYPE(field_type) = self.get_next_token().r#type {
                field_type
            } else {
                errors::parser_error(self, "invalid token");
            };

            fields.push((self.get_current_token().value.unwrap(), field_type));

            self.set_next_position();
        }

        self.set_next_position();

        Box::new(StructDefParserNode {
            struct_name,
            fields,
        })
    }

    pub fn parse_struct(&mut self) -> Box<StructParserNode> {
        let mut fields = vec![];

        while self.get_next_token().r#type != Types::DELIMITER(Delimiter::RBRACE) {
            if self.get_next_token().r#type == Types::NL
                || self.get_next_token().r#type == Types::DELIMITER(Delimiter::COMMA)
            {
                self.set_next_position();
                continue;
            }

            self.set_next_position();
            if self.get_current_token().r#type != Types::IDENTIFIER {
                errors::parser_error(self, "invalid token");
            }

            let field_name = self.get_current_token().value.unwrap();

            let field_value = self.parse_expression();

            fields.push((field_name, *field_value));

            self.set_next_position();
        }

        self.set_next_position();

        Box::new(StructParserNode { fields })
    }
}
