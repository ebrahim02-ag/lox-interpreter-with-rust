use std::io;

use crate::token::{Literal, Token};
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


    fn expression(&mut self) -> Expr{
        return self.equality()
    }

    fn equality(&mut self) -> Expr {
        let token_types = [TokenType::BangEqual, TokenType::EqualEqual];
        self._left_recurse_binary(&token_types, Parser::comparison)
    }

    fn comparison(&mut self) -> Expr {
        let token_types = [TokenType::Greater, TokenType::GreaterEqual, TokenType::LessEqual, TokenType::Less];
        self._left_recurse_binary(&token_types, Parser::comparison)
    }

    fn term(&mut self) -> Expr {
        let token_types = [TokenType::Minus, TokenType::Plus];
        self._left_recurse_binary(&token_types, Parser::comparison)
    }

    fn factor(&mut self) -> Expr {
        let token_types = [TokenType::Star, TokenType::Slash];
        self._left_recurse_binary(&token_types, Parser::comparison)
    }

    fn unary(&mut self) -> Expr {
        let token_types = [TokenType::Bang, TokenType::Minus];
        let mut expr;
        if self._match(&token_types){
            let operator = self._previous();
            let right = self.unary();
            expr = Expr::Unary(Unary { op:operator, right: Box::new(right) })
        } else{
            expr = self.primary()
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        if self._match(&[TokenType::True]){
            return Expr::Literal(Literal::Bool(true))
        }

        if self._match(&[TokenType::False]){
            return Expr::Literal(Literal::Bool(false))
        }

        if self._match(&[TokenType::Nil]){
            return Expr::Literal(Literal::Nil)
        }

        if self._match(&[TokenType::Number, TokenType::String]){
            let literal = self._previous().literal.clone();
            return Expr::Literal(literal)
        }

        if self._match(&[TokenType::LeftParen]){
            let expr = self.expression();
            match self._consume(&TokenType::RightParen, "expected right paranthesis") {
                Ok(true) => (),
                Ok(false) => (),
                Err(error) => panic!("it's cooked...")
            }

            return Expr::Grouping(Grouping { expression: Box::new(expr) })
        }

        panic!("something went wrong")
    }

    fn _left_recurse_binary<F>(&mut self, token_types: &[TokenType], method: F) -> Expr
    where F:Fn(&mut Self) -> Expr
    {
        // This method takes in a method as an argument (which here is the function of next precedence)
        // which it then passes the mutable self to it to continue it's recursing journey.
        // Finally it constructs the expression and returns it
        let mut expr = method(self);
        while self._match(token_types){
            let operator = self._previous();
            let right = method(self);
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

    fn _consume(&mut self, token_type: &TokenType, error: &'static str) -> Result<bool, String> {
        if !self._match(&[*token_type]){
            return Err(error.to_string())
        }
        Ok(true)
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

}
