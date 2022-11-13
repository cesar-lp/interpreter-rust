#[derive(PartialEq, Debug)]
pub struct ParsedToken {
    token: Token,
    literal: String,
}

impl ParsedToken {
    pub fn new(token: Token, value: &str) -> Self {
        ParsedToken {
            token,
            literal: value.to_string(),
        }
    }
}

// const ILLEGAL : &str = "ILLEGAL";
// const EOF : &str = "EOF";

// // Identifiers + literals
// const IDENTIFIER: &str = "IDENTIFIER";
// const INTEGER: &str = "INTEGER";

// // Operators
// const ASSIGN: &str = "=";
// const PLUS: &str = "+";

// // Delimiters
// const COMMA: &str = ",";
// const SEMICOLON : &str = ";";

// const LEFT_PARTENTHESIS: &str = "(";
// const RIGHT_PARTENTHESIS: &str = ")";
// const LEFT_CURLY_BRACE: &str = "{";
// const RIGHT_CURLY_BRACE: &str = "}";

// // Keywords
// const FUNCTION: &str = "FUNCTION";
// const LET: &str = "LET";

#[derive(Debug, PartialEq)]
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

// Transforms source code into tokens
struct Lexer {}

// Transforms tokens into AST
struct Parser {}
