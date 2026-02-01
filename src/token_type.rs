

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType{
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof,
}

impl From<&str> for TokenType {
    fn from(item: &str) -> Self {
        match item{
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" =>     TokenType::If,
            "nil" =>    TokenType::Nil,
            "or" =>     TokenType::Or,
            "print" =>  TokenType::Print,
            "return" => TokenType::Return,
            "super" =>  TokenType::Super,
            "this" =>   TokenType::This,
            "true" =>   TokenType::True,
            "var" =>    TokenType::Var,
            "while" =>  TokenType::While,
            _ => TokenType::Identifier // like a variable for example
        }
    }
}
