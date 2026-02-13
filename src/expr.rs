use crate::token::{Token, Literal};

pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
    Variable(Variable),
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Unary {
    pub op: Token,
    pub right: Box<Expr>,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub op: Token,
    pub right: Box<Expr>,
}

pub struct Variable {
    pub name: Token,
}

pub trait Visitor<T> {
    fn visit_binaryexp(&self, e: &Binary) -> T;
    fn visit_groupingexp(&self, e: &Grouping) -> T;
    fn visit_literalexp(&self, e: &Literal) -> T;
    fn visit_unaryexp(&self, e: &Unary) -> T;
    fn visit_variableexp(&self, e: &Variable) -> T;
}

pub fn walk_expr<T>(visitor: &dyn Visitor<T>, e: &Expr) -> T {
    match e {
        Expr::Literal(lit) => visitor.visit_literalexp(lit),
        Expr::Binary(binary) => visitor.visit_binaryexp(binary),
        Expr::Unary(unary) => visitor.visit_unaryexp(unary),
        Expr::Grouping(grouping) => visitor.visit_groupingexp(grouping),
        Expr::Variable(variable) => visitor.visit_variableexp(variable),

    }
}
