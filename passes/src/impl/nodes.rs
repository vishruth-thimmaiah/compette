use parser::nodes::{
    ASTNodes, ArrayIndex, AssignStmt, Attr, Block, Conditional, Expression, Extern, ForLoop,
    Function, FunctionCall, ImportCall, ImportDef, LetStmt, Literal, Loop, Method, Return,
    StructDef, Types, Variable,
};

use crate::r#impl::PassData;

pub trait PassTraversal<'a> {
    fn visit(&'a mut self, data: &PassData<'a>);
}

impl<'a> PassTraversal<'a> for ASTNodes {
    fn visit(&'a mut self, data: &PassData<'a>) {
        match self {
            ASTNodes::Function(function) => function.visit(data),
            ASTNodes::Block(block) => block.visit(data),
            ASTNodes::ForLoop(for_loop) => for_loop.visit(data),
            ASTNodes::AssignStmt(assign_stmt) => assign_stmt.visit(data),
            ASTNodes::ArrayIndex(array_index) => array_index.visit(data),
            ASTNodes::Attr(attr) => attr.visit(data),
            ASTNodes::Conditional(conditional) => conditional.visit(data),
            ASTNodes::Expression(expression) => expression.visit(data),
            ASTNodes::FunctionCall(function_call) => function_call.visit(data),
            ASTNodes::ImportDef(import_def) => import_def.visit(data),
            ASTNodes::ImportCall(import_call) => import_call.visit(data),
            ASTNodes::LetStmt(let_stmt) => let_stmt.visit(data),
            ASTNodes::Literal(literal) => literal.visit(data),
            ASTNodes::Loop(r#loop) => r#loop.visit(data),
            ASTNodes::Method(method) => method.visit(data),
            ASTNodes::Return(r#return) => r#return.visit(data),
            ASTNodes::StructDef(struct_def) => struct_def.visit(data),
            ASTNodes::Token(types) => types.visit(data),
            ASTNodes::Variable(variable) => variable.visit(data),
            ASTNodes::Break => return,
            ASTNodes::Extern(ext) => ext.visit(data),
        }
    }
}

impl<'a> PassTraversal<'a> for Function {
    fn visit(&'a mut self, data: &PassData<'a>) {
        self.body.visit(data);
    }
}

impl<'a> PassTraversal<'a> for Block {
    fn visit(&'a mut self, data: &PassData<'a>) {
        for node in self.body.iter_mut() {
            node.visit(data);
        }
    }
}

impl<'a> PassTraversal<'a> for AssignStmt {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for ArrayIndex {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Attr {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Conditional {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Expression {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for FunctionCall {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for ImportDef {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}
impl<'a> PassTraversal<'a> for ImportCall {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for LetStmt {
    fn visit(&'a mut self, data: &PassData<'a>) {
        data.vars.borrow_mut().push(self);
    }
}

impl<'a> PassTraversal<'a> for Literal {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Loop {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for ForLoop {
    fn visit(&'a mut self, data: &PassData<'a>) {
        self.body.visit(data);
    }
}

impl<'a> PassTraversal<'a> for Method {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Return {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for StructDef {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Types {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Variable {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}

impl<'a> PassTraversal<'a> for Extern {
    fn visit(&'a mut self, _data: &PassData<'a>) {}
}
