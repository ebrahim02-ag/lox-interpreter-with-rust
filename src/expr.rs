use crate::token::{Token, Literal};

pub enum Expr {
    Literal(Literal),
    Grouping(Grouping),
    Unary(Unary),
    Binary(Binary),
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Unary<'a> {
    pub op: &'a Token,
    pub right: Box<Expr>,
}

pub struct Binary<'a> {
    pub left: Box<Expr>,
    pub op: &'a Token,
    pub right: Box<Expr>,
}

pub trait Visitor<T> {
    fn visit_binaryexp(&mut self, e: &Binary) -> T;
    fn visit_groupingexp(&mut self, e: &Grouping) -> T;
    fn visit_literalexp(&mut self, e: &Literal) -> T;
    fn visit_unaryexp(&mut self, e: &Unary) -> T;
}

pub fn walk_expr<T>(visitor: &mut dyn Visitor<T>, e: &Expr) -> T {
    match e {
        Expr::Literal(lit) => visitor.visit_literalexp(lit),
        Expr::Binary(binary) => visitor.visit_binaryexp(binary),
        Expr::Unary(unary) => visitor.visit_unaryexp(unary),
        Expr::Grouping(grouping) => visitor.visit_groupingexp(grouping),
    }
}
