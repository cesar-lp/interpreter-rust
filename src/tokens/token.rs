#[derive(PartialEq, Debug)]
pub struct ParsedToken {
    pub token: Token,
    pub literal: String,
}

impl ParsedToken {
    pub fn new(token: Token, value: &str) -> Self {
        ParsedToken {
            token,
            literal: value.to_string(),
        }
    }
}

// TODO: allow data to be passed into the enum variant
// TODO: implement lookup table?
#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Illegal,
    EOF,
    Identifier,
    Integer,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    Function,
    Let,
}

impl Token {
    pub fn new(value: &str) -> Self {
        match value {
            "fn" => Token::Function,
            "let" => Token::Let,
            _ => Token::Identifier,
        }
    }
}

// Transforms tokens into AST
struct Parser {}
