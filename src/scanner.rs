use core::f32;

use crate::{lox_error, token_type};
use crate::token::{self, Token};
use crate::token_type::TokenType;
use crate::token::Literal;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize, 
    line: usize, 
    current: usize
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner{
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {        
        while !self.is_at_end(){
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "", Literal::Nil, self.line));

        &self.tokens
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal){
        let text = &self.source[self.start..self.current];
        let token: Token = Token::new(token_type, &text, literal, self.line);
        self.tokens.push(token);
    }

    fn scan_token(&mut self){
        let c = self.advance_char().unwrap();

        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil),
            '{' => self.add_token(TokenType::LeftBrace, Literal::Nil),
            '}' => self.add_token(TokenType::RightBrace, Literal::Nil),
            ',' => self.add_token(TokenType::Comma, Literal::Nil),
            '.' => self.add_token(TokenType::Dot, Literal::Nil),
            '-' => self.add_token(TokenType::Minus, Literal::Nil),
            '+' => self.add_token(TokenType::Plus, Literal::Nil),
            ';' => self.add_token(TokenType::Semicolon, Literal::Nil),
            '*' => self.add_token(TokenType::Star, Literal::Nil),
            '!' => {
                if self.match_char('='){
                    self.add_token(TokenType::BangEqual, Literal::Nil);
                } else {
                    self.add_token(TokenType::Bang, Literal::Nil);
                }
            }
            '=' => {
                if self.match_char('='){
                    self.add_token(TokenType::EqualEqual, Literal::Nil);
                } else {
                    self.add_token(TokenType::Equal, Literal::Nil);
                }
            }
            '<' => {
                if self.match_char('='){
                    self.add_token(TokenType::LessEqual, Literal::Nil);
                } else {
                    self.add_token(TokenType::Less, Literal::Nil);
                }
            }
            '>' => {
                if self.match_char('='){
                    self.add_token(TokenType::GreaterEqual, Literal::Nil);
                } else {
                    self.add_token(TokenType::Greater, Literal::Nil);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // ignore comments
                    while !self.is_at_end() && self.peek() != '\n'{
                        self.advance_char();
                    }
                } else if self.match_char('*'){
                    // ignore block comments => /* */
                    while !self.is_at_end() && !(self.match_char('*') && self.match_char('/')) {
                        self.advance_char();
                    }
                } else {
                    self.add_token(TokenType::Slash, Literal::Nil);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line+=1, 
            '"' => self.string(),
            _ => {
                if self.is_digit(&c) {
                    self.number();
                } else if self.is_alpha(&c){
                    self.identifier();
                }
                
                else {
                    lox_error(&self.line, "Unexpected character.")
                }
            },
        }
            
    }

    fn peek(&self) -> char {
        if self.is_at_end(){return '\0'}
        self.source[self.current..].chars().next().unwrap()
    }

    fn peek_next(&self) -> char{
        // Get second next character.
        if self.is_at_end(){return '\0'}
        let mut iter = self.source[self.current..].chars();
        iter.next(); // skip first next character
        let next = iter.next();
        match next{
            None =>  '\0',
            Some(i) => i
        }
    }

    fn advance_char(&mut self) -> Option<char> {
        let c = self.peek();
        self.current += c.len_utf8(); // move by correct byte width
        Some(c)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        // check if next character matches expected, if so, advance
        if self.is_at_end() { return false };

        let c = self.peek();
        if c != expected { return false };
        
        self.current += c.len_utf8();
        true
    }

    fn string(&mut self) {
        // find closing "
        while !self.is_at_end() && self.peek() != '"'{
            if self.peek() == '\n'{ self.line+=1 }
            self.advance_char();
        }

        if self.is_at_end(){ lox_error(&self.line, "Undetermined string");}

        // the closing " was found, advance to it
        self.advance_char();

        // remove the surrounding ", " is size 1 byte in UTF-8
        let value = Literal::String(self.source[self.start+1..self.current-1].to_string());
        self.add_token(TokenType::String, value);

    }

    fn is_digit(&self, c: &char) -> bool{
        return *c >= '0' && *c <= '9'
    }

    fn number(&mut self){
        // A number could be a floating 1.2344 -> only one '.' so this advances until met
        while self.is_digit(&self.peek()) {
            self.advance_char();
        }

        if self.peek() == '.' && self.is_digit(&self.peek_next()){ 
            self.advance_char();

            while self.is_digit(&self.peek()) {
                self.advance_char();
            }
        }

        self.add_token(TokenType::Number, Literal::Number(self.source[self.start..self.current].parse::<f64>().unwrap()));
    }

    fn identifier(&mut self){
        while self.is_alpha_numeric(&self.peek()) {
            self.advance_char();
        }
        
        let text = &self.source[self.start..self.current];
        let tt = TokenType::from(text);
        self.add_token(tt, Literal::Nil);
    }

    fn is_alpha(&self, c: &char) -> bool{
        return (*c >= 'a' && *c <= 'z') || (*c >= 'A' && *c <= 'Z') || (*c == '_')
    }

    fn is_alpha_numeric(&self, c: &char) -> bool{
        return self.is_alpha(c) || self.is_digit(c)
    }
}