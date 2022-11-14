// TODO: implement lookup table?
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal(String),
    EOF,
    Identifier(String),
    Integer(String),
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    Function,
    Let,
    LessThan,
    GreaterThan,
}

impl Token {
    pub fn new(value: &str) -> Self {
        match value {
            "fn" => Token::Function,
            "let" => Token::Let,
            identifier => Token::Identifier(identifier.to_string()),
        }
    }
}
