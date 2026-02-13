use crate::token::{Token, Literal};
use crate::expr::Expr;

pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Variable(Variable)
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

pub trait Visitor<T> {
    fn visit_expression(&self, e: &Expression) -> T;
    fn visit_print(&self, e: &Print) -> T;
}



pub fn walk_stmt<T>(visitor: &dyn Visitor<T>, e: &Stmt) -> T {
    match e {
        Stmt::Expression(expr) => visitor.visit_expression(expr),
        Stmt::Print(pri) => visitor.visit_print(pri),
    }
}
