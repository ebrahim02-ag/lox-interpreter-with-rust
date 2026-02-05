use std::cmp::{min};

use crate::token::{Literal, Token};
use crate::token_type::{self, TokenType};
use crate::expr::{Expr, Binary, Unary, Grouping};
use crate::lox_error;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

struct ParserError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self{
        Self{
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression(){
            Ok(expr) => return Some(expr),
            Err(error) => return None
        }
    }


    fn expression(&mut self) -> Result<Expr, ParserError>{
        return self.comma()
    }

    fn comma(&mut self) -> Result<Expr, ParserError> {
        // challenge question ch6. Comma has lowest precedence in C according to stackoverflow
        // https://stackoverflow.com/questions/54142/how-does-the-comma-operator-work-and-what-precedence-does-it-have
        let token_types = [TokenType::Comma];
        self._left_recurse_binary(&token_types, Parser::equality)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::BangEqual, TokenType::EqualEqual];
        self._left_recurse_binary(&token_types, Parser::comparison)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::Greater, TokenType::GreaterEqual, TokenType::LessEqual, TokenType::Less];
        self._left_recurse_binary(&token_types, Parser::term)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::Minus, TokenType::Plus];
        self._left_recurse_binary(&token_types, Parser::factor)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::Star, TokenType::Slash];
        self._left_recurse_binary(&token_types, Parser::unary)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::Bang, TokenType::Minus];
        let expr;
        if self._match(&token_types){
            let operator = self._previous().clone();
            let right = self.unary()?;
            expr = Expr::Unary(Unary { op:operator, right: Box::new(right) });
        } else {
            expr = self.primary()?;
        }
        Ok(expr)
    }

    // if i were to support postfix (e.g a++) i'd add it here as a method and
    // has its precedence right before primary

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self._match(&[TokenType::True]){
            return Ok(Expr::Literal(Literal::Bool(true)))
        }

        if self._match(&[TokenType::False]){
            return Ok(Expr::Literal(Literal::Bool(false)))
        }

        if self._match(&[TokenType::Nil]){
            return Ok(Expr::Literal(Literal::Nil))
        }

        if self._match(&[TokenType::Number, TokenType::String]){
            let literal = self._previous().literal.clone();
            return Ok(Expr::Literal(literal))
        }

        if self._match(&[TokenType::LeftParen]){
            let expr = self.expression()?;
            self._consume(&TokenType::RightParen, "expected right paranthesis")?;
            return Ok(Expr::Grouping(Grouping { expression: Box::new(expr) }))
        }

        /* binary error productiond. If we find a binary operator here it means that the
        binary expression method's left operand does not exist. e.g => (>= 2).
        Print out the error but return the method to ignore the token basically and continue recursion.
        Reason is cuz (+ 1 - 2) is valid and should evaluate to  (1 - 2)
        */
        if self._match(&[TokenType::BangEqual, TokenType::Equal]){
            self._error(self._peek(), "Missing Left Hand Operand");
            return self.equality();
        }
        if self._match(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]){
            self._error(self._peek(), "Missing Left Hand Operand");
            return self.comparison();
        }
        if self._match(&[TokenType::Plus]){
            self._error(self._peek(), "Missing Left Hand Operand");
            return self.term();
        }
        if self._match(&[TokenType::Star, TokenType::Slash]){
            self._error(self._peek(), "Missing Left Hand Operand");
            return self.factor();
        }

        Err(self._error(self._peek(), "expected expression"))
    }

    fn _left_recurse_binary<F>(&mut self, token_types: &[TokenType], method: F) -> Result<Expr, ParserError>
    where F: Fn(&mut Self) -> Result<Expr, ParserError>
    {
        // This method takes in a method as an argument (which here is the function of next precedence)
        // which it then passes the mutable self to it to continue it's recursing journey.
        // Finally it constructs the expression and returns it
        let mut expr = method(self)?;
        while self._match(token_types){
            let operator = self._previous().clone();
            let right = method(self)?;
            expr = Expr::Binary(Binary{op: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
    }

    fn _match(&mut self, token_types: &[TokenType]) -> bool {
        if token_types.iter().any(|x| self._check(x)){
            self._advance();
            return true
        }
        false
    }

    fn _match_mult(&mut self, token_type: &TokenType, lookahead: usize) -> bool{
        let mut advanced = 0;
        for _ in 0..lookahead {
            if self._check(token_type){
                self._advance();
                advanced = advanced + 1;
            } else {
                for _ in 0..advanced {
                    self._retreat();
                }
                return false
            }
        }

        return true
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

    fn _consume(&mut self, token_type: &TokenType, error: &'static str) -> Result<&Token, ParserError> {
        if self._check(token_type){
            return Ok(self._advance())
        }
        Err(self._error(self._peek(), error))
    }

    fn _error(&self, token: &Token, message: &str) -> ParserError {
        lox_error(token, message);
        ParserError
    }

    fn _synchronize(&mut self){
        self._advance();

        // this will be implemented once we have statements
        while !self._at_end() {
          // if (self._previous().type == StatmentType::SEMICOLON) {
          //     return
          // }

          // match self.peek().type {

          //   StatmentType::CLASS: ()
          //   StatmentType::FUN: ()
          //   StatmentType::VAR: ()
          //   StatmentType::FOR: ()
          //   StatmentType::IF: ()
          //   StatmentType::WHILE: ()
          //   StatmentType::PRINT: ()
          //   StatmentType::RETURN:( )
          //     return;
          // }

          self._advance();
        }
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

    fn _retreat(&mut self) -> &Token {
        self.current = min(self.current - 1, 0);
        self._peek()
    }

}
