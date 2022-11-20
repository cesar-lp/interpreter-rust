// TODO: implement lookup table?
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal(String),
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
    If,
    Else,
    True,
    False,
    Return,
    Equals,
    NotEquals,
}

impl Token {
    pub fn new(value: &str) -> Self {
        match value {
            "fn" => Token::Function,
            "let" => Token::Let,
            "true" => Token::True,
            "false" => Token::False,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            identifier => Token::Identifier(identifier.to_string()),
        }
    }
}
