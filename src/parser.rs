use crate::token::{Token};
use crate::token_type::{self, TokenType};
use crate::expr::{Expr, Binary, Unary, Grouping};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self{
        Self{
            tokens: tokens,
            current: 0
        }
    }


    fn expression(&self) -> Expr{
        return self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        let token_types = [TokenType::BangEqual, TokenType::EqualEqual];
        while self._match(&token_types){
            let operator = self._previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary{op: operator, left: Box::new(expr), right: Box::new(right)});
        }

        expr
    }

    fn _match(&mut self, token_types: &[TokenType]) -> bool {
        if token_types.iter().any(|x| self._check(x)){
            self._advance();
            return true
        }
        false
    }

    fn _check(&self, token_type: &TokenType) -> bool{
        if self._at_end() {
            return false
        }

        return *token_type == self._peek().kind
    }

    fn _peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn _at_end(&self) -> bool{
        self._peek().kind == TokenType::Eof
    }

    fn _previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn _advance(&mut self) -> &Token {
        if !self._at_end(){
            self.current += 1;
        }
        self._previous()
    }


    fn comparison(&self) -> Expr {
        Expr::Unary(Unary{op: self._peek(), right: Box::new(self.expression())})
    }
}
