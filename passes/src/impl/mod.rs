use crate::r#impl::nodes::PassTraversal;

mod nodes;

struct PassData<'a> {
    vars: std::cell::RefCell<Vec<&'a parser::nodes::LetStmt>>,
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
