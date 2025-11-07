use std::cell::RefCell;
use std::collections::HashMap;

use parser::nodes::Datatype;

use crate::r#impl::nodes::PassTraversal;

mod nodes;

pub(crate) struct PassData<'a> {
    vars: RefCell<Vec<HashMap<&'a str, Variables<'a>>>>,
}

impl<'a> PassData<'a> {
    pub(crate) fn get_vars(&self, name: &str) -> Option<Variables<'a>> {
        let borrow = self.vars.borrow();

        for map in borrow.iter() {
            if let Some(var) = map.get(name) {
                return Some(var.clone());
            }
        }
        return None;
    }

    pub(crate) fn add_var(&self, name: &'a str, var: Variables<'a>) {
        self.vars.borrow_mut().last_mut().unwrap().insert(name, var);
    }

    pub(crate) fn push_scope(&self) {
        self.vars.borrow_mut().push(HashMap::new());
    }

    pub(crate) fn pop_scope(&self) {
        self.vars.borrow_mut().pop();
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Variables<'a> {
    pub(crate) let_stmt_ptr: *mut parser::nodes::LetStmt,
    _marker: std::marker::PhantomData<&'a mut parser::nodes::LetStmt>,
}

impl<'a> Variables<'a> {
    pub(crate) fn new(let_stmt: &'a mut parser::nodes::LetStmt) -> Self {
        Self {
            let_stmt_ptr: let_stmt as *mut _,
            _marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn set_dt(&self, dt: Datatype) {
        // SAFETY: The pointer is valid for 'a and the reference doesn't outlive it
        unsafe {
            (*self.let_stmt_ptr).datatype = dt;
        }
    }

    pub(crate) fn get_dt(&self) -> Datatype {
        // SAFETY: The pointer is valid for 'a and the reference doesn't outlive it
        unsafe { (*self.let_stmt_ptr).datatype.clone() }
    }
    //
    // pub(crate) fn get_value(&self) -> &parser::nodes::Expression {
    //     // SAFETY: The pointer is valid for 'a and the reference doesn't outlive it
    //     unsafe { &(*self.let_stmt_ptr).value }
    // }
    //
    // pub(crate) fn set_value(&self, value: parser::nodes::Expression) {
    //     // SAFETY: The pointer is valid for 'a and the reference doesn't outlive it
    //     unsafe {
    //         (*self.let_stmt_ptr).value = value;
    //     }
    // }
    //
    // pub(crate) fn get_mut(&self) -> bool {
    //     // SAFETY: The pointer is valid for 'a and the reference doesn't outlive it
    //     unsafe { (*self.let_stmt_ptr).mutable }
    // }
    //
    // pub(crate) fn set_mut(&self, value: bool) {
    //     // SAFETY: The pointer is valid for 'a and the reference doesn't outlive it
    //     unsafe {
    //         (*self.let_stmt_ptr).mutable = value;
    //     }
    // }
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
                vars: RefCell::new(vec![]),
            },
        }
    }

    pub fn invoke(&'a mut self) {
        self.parser.iter_mut().for_each(|x| x.visit(&self.data));
    }
}
