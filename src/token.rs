use crate::token_type::TokenType;
use std::fmt;

pub struct Token {
    kind: TokenType,
    pub lexeme: String,
    literal: Literal,
    line: usize,
}

pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl Token {
    pub fn new(kind: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self{
        Self{
            kind, 
            lexeme: lexeme.to_string(),
            literal,
            line
        }
    }
}

impl fmt::Display for Token{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.kind, self.lexeme, self.literal, self.line)
    }
}