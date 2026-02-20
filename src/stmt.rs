use crate::token::{Token};
use crate::expr::Expr;

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Variable(Variable),
    Block(Block),
    If(If)
}

pub struct Expression {
    pub expression: Expr,
}

pub struct Print {
    pub expression: Expr,
}

pub struct Variable {
    pub name: Token,
    pub initializer: Expr,
}

pub struct Block {
    pub statements: Vec<Stmt>
}

pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>
}

pub trait Visitor<T> {
    fn visit_expression(&self, e: &Expression) -> T;
    fn visit_print(&self, e: &Print) -> T;
    fn visit_var_stmt(&self, e: &Variable) -> T;
    fn visit_block_stmt(&self, e: &Block) -> T;
    fn visit_if_stmt(&self, e: &If) -> T;
}


pub fn walk_stmt<T>(visitor: &dyn Visitor<T>, e: &Stmt) -> T {
    match e {
        Stmt::Expression(expr) => visitor.visit_expression(expr),
        Stmt::Print(pri) => visitor.visit_print(pri),
        Stmt::Variable(var) => visitor.visit_var_stmt(var),
        Stmt::Block(blo) => visitor.visit_block_stmt(blo),
        Stmt::If(i) => visitor.visit_if_stmt(i)
    }
}
