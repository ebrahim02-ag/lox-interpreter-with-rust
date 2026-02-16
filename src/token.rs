use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

#[derive(Debug, Clone)]
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

    pub fn default(line: usize) -> Self {
        Self {
            kind: TokenType::Nil,
            lexeme: "".to_string(),
            literal: Literal::Nil,
            line: line
        }
    }
}

impl fmt::Display for Token{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {} {}", self.kind, self.lexeme, self.literal, self.line)
    }
}
