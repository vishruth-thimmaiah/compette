use std::collections::HashMap;

use parser::nodes::{Datatype, Expression};

use crate::r#impl::nodes::PassTraversal;

mod nodes;

struct PassData<'a> {
    vars: std::cell::RefCell<Vec<HashMap<&'a str, Variables<'a>>>>,
}

impl<'a> PassData<'a> {
    pub fn get_vars(&self, name: &str) -> Option<Variables<'a>> {
        let borrow = self.vars.borrow();

        for map in borrow.iter() {
            if let Some(var) = map.get(name) {
                return Some(var.clone());
            }
        }
        return None;
    }
}

#[derive(Debug, Clone)]
struct Variables<'a> {
    pub value: &'a Expression,
    pub datatype: Datatype,
    pub mutable: bool,
}

impl<'a> Variables<'a> {
    pub fn new(let_stmt: &'a parser::nodes::LetStmt) -> Self {
        Self {
            value: &let_stmt.value,
            datatype: let_stmt.datatype.clone(),
            mutable: let_stmt.mutable,
        }
    }
}

pub struct PassManager<'a> {
    parser: &'a mut Vec<parser::nodes::ASTNodes>,
    data: PassData<'a>,
}

impl<'a> PassManager<'a> {
    pub fn new(parser: &'a mut Vec<parser::nodes::ASTNodes>) -> Self {
        Self {
            parser,
            data: PassData {
                vars: std::cell::RefCell::new(vec![]),
            },
        }
    }

    pub fn invoke(&'a mut self) {
        self.parser.iter_mut().for_each(|x| x.visit(&self.data));
    }
}
