use std::cmp::{min};

use crate::token::{Literal, Token};
use crate::token_type::{self, TokenType};
use crate::expr::{Assign, Binary, Expr, Grouping, Unary, Variable as VariableExpr, Logical};
use crate::lox_error;
use crate::stmt::{Expression, Print, Stmt, Variable, Block, If, While};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

pub struct ParserError;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self{
        Self{
            tokens: tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self._at_end(){
            statements.push(self.declaration()?);
        }

        return Some(statements);
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let token_type = [TokenType::Var];
        let statement = match self._match(&token_type) {
            true => self.var_statement(),
            false => self.statement()
        };

        match statement {
            Ok(stm) => Some(stm),
            Err(stm) => {
                self._synchronize();
                None
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParserError>  {
        let token_type = [TokenType::If];
        if self._match(&token_type) {
            return self.if_statement()
        }

        let token_type = [TokenType::Print];
        if self._match(&token_type) {
            return self.print_statement()
        }

        let token_type = [TokenType::LeftBrace];
        if self._match(&token_type) {
            return Ok(Stmt::Block(Block{statements: self.block()?}))
        }

        let token_type = [TokenType::While];
        if self._match(&token_type) {
            return self.while_statement()
        }

        let token_type = [TokenType::For];
        if self._match(&token_type) {
            return self.for_statement()
        }

        return self.expression_statement()
    }


    fn var_statement(&mut self) -> Result<Stmt, ParserError> {
        // this john is trying to emulate -> var a = 12;

        let name = self._consume(&TokenType::Identifier, "Expected variable name :<(")?.clone();
        let token_type = [TokenType::Equal];
        let expr = match self._match(&token_type) {
            true => self.expression(),
            false => Ok(Expr::Literal(Literal::Nil)) // var a -> means var a = None;
        };
        self._consume(&TokenType::Semicolon, "Expected ';' at the end of statement")?;
        return Ok(Stmt::Variable(Variable {name: name, initializer: expr?}));
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self._consume(&TokenType::LeftParen, "Expected a '(' after 'if'")?;
        let condition = self.expression()?;
        self._consume(&TokenType::RightParen, "Expected a ')' end of 'if' expression")?;

        let then_stmt = Box::new(self.statement()?);
        let mut else_stmt = None;
        let token_type = [TokenType::Else];
        if self._match(&token_type) {
            else_stmt = Some(Box::new(self.statement()?));
        }

        return Ok(Stmt::If(If{
            condition: condition,
            then_branch: then_stmt,
            else_branch: else_stmt
        }))

    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self._consume(&TokenType::LeftParen, "Expected a '(' after 'while'")?;
        let condition = self.expression()?;
        self._consume(&TokenType::RightParen, "Expected a ')' end of 'while' expression")?;

        let body = Box::new(self.statement()?);

        return Ok(Stmt::While(While{
            condition: condition,
            body: body,
        }))
    }

    fn for_statement(&mut self) -> Result<Stmt, ParserError> {
        // there is no for statement trait, we just desugar it into a while loop

        self._consume(&TokenType::LeftParen, "Expected a '(' after 'for'")?;
        let initializer: Option<Stmt>;
        let semicolon = [TokenType::Semicolon];
        let var_type = TokenType::Var;
        if self._match(&semicolon){
            // the variable was declared outside -> for (; i < 10; i = i + 1)
            initializer = None;
        }
        else if self._check(&var_type){
            // for (var i = 0...
            initializer = self.declaration();
        } else {
            // for (i = 0) -> assignment
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expr> = None;
        if !self._check(&semicolon[0]){
            condition = Some(self.expression()?);
        }

        self._consume(&semicolon[0], "Expected a ';' after conditon in 'for'")?;

        let mut increment: Option<Expr> = None;
        if !self._check(&semicolon[0]){
            increment = Some(self.expression()?);
        }

        self._consume(&&TokenType::RightParen, "Expected a ')' end of 'for'")?;
        let mut body = self.statement()?;

        if let Some(e) = increment {
            // if an increment exists, then it should be executed after the body every loop
            // so we wrap it around a block with the body and the increment so they
            // can always be executed together
            body = Stmt::Block(Block {
                statements: vec![
                    body,
                    Stmt::Expression(Expression { expression: e })
                ]
            });
        }

        // if no condition, then explicity set it to true
        let condition = condition.unwrap_or_else(|| Expr::Literal(Literal::Bool(true)));

        body = Stmt::While(While { condition: condition, body: Box::new(body) });

        // finally, jam the initializer, if it exists, to the top so it runs once before the while loop
        if let Some(e) = initializer {
            body = Stmt::Block(Block {
                statements: vec![
                     e,
                    body
                ]
            });
        }

        Ok(body)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression();
        self._consume(&TokenType::Semicolon, "Expected ';' at the end of statement")?;
        return Ok(Stmt::Print(Print {expression: expr?}));
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression();
        self._consume(&TokenType::Semicolon, "Expected ';' at the end of statement")?;
        return Ok(Stmt::Expression(Expression {expression: expr?}));
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParserError> {
        let mut statements = Vec::new();

        let token_type = TokenType::RightBrace;
        while !self._check(&token_type) & !self._at_end(){
            let expr = self.declaration();
            statements.extend(expr);
        }
        self._consume(&token_type, "expected a '}' after the bloke")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, ParserError>{
        self.assignment()
    }

    fn assignment(&mut self) ->  Result<Expr, ParserError> {
        let expr = self.or()?;
        let token_types = [TokenType::Equal];

        if !self._match(&token_types) {
            return Ok(expr)
        }

        match expr {
            Expr::Variable(var) => {
                // a = b = 2 for example
                let value = self.assignment()?;
                let name = var.name.clone();
                Ok(Expr::Assign(Assign {name: name, value: Box::new(value)}))
            },
            _ => {
                let prev = self._previous();
                Err(self._error(prev, "Invalid assignment target"))
            }
        }
    }

    fn or(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::Or];
        let mut expr = self.and()?;
        while self._match(&token_types){
            let operator = self._previous().clone();
            let right = self.and()?;
            expr = Expr::Logical(Logical{condition: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParserError> {
        let token_types = [TokenType::And];
        let mut expr = self.comma()?;
        while self._match(&token_types){
            let operator = self._previous().clone();
            let right = self.comma()?;
            expr = Expr::Logical(Logical{condition: operator, left: Box::new(expr), right: Box::new(right)});
        }
        Ok(expr)
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

        if self._match(&[TokenType::Identifier]){
            let name = self._previous().clone();
            return Ok(Expr::Variable(VariableExpr { name: name}))
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
