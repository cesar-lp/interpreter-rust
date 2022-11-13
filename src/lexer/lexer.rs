use crate::tokens::{ParsedToken, Token};

// TODO: maybe current_char should be grouped with position?
// TODO: support unicode and emojis?
pub struct Lexer {
    input: String,
    position: usize,      // points to current char
    read_position: usize, // after current char
    current_char: char,   // under examination
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            current_char: '0',
        };

        lexer.read_char();

        lexer
    }

    pub fn next_token(&mut self) -> ParsedToken {
        let token = match self.current_char {
            '=' => ParsedToken::new(Token::Assign, self.current_char.to_string().as_str()),
            ';' => ParsedToken::new(Token::Semicolon, self.current_char.to_string().as_str()),
            '(' => ParsedToken::new(
                Token::LeftParenthesis,
                self.current_char.to_string().as_str(),
            ),
            ')' => ParsedToken::new(
                Token::RightParenthesis,
                self.current_char.to_string().as_str(),
            ),
            '{' => ParsedToken::new(
                Token::LeftCurlyBrace,
                self.current_char.to_string().as_str(),
            ),
            '}' => ParsedToken::new(
                Token::RightCurlyBrace,
                self.current_char.to_string().as_str(),
            ),
            ',' => ParsedToken::new(Token::Comma, self.current_char.to_string().as_str()),
            '+' => ParsedToken::new(Token::Plus, self.current_char.to_string().as_str()),
            '0' => ParsedToken::new(Token::EOF, ""),
            unhandled_char => {
                panic!("Invalid character parsed {unhandled_char}")
            }
        };

        self.read_char();

        token
    }

    fn read_char(&mut self) {
        self.current_char = match self.input.chars().nth(self.read_position) {
            Some(ch) => ch,
            None => '0', // ASCII code for NUL, we haven't read anything yet or EOF
        };

        self.position = self.read_position;
        self.read_position += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_get_next_token() {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            ParsedToken::new(Token::Assign, "="),
            ParsedToken::new(Token::Plus, "+"),
            ParsedToken::new(Token::LeftParenthesis, "("),
            ParsedToken::new(Token::RightParenthesis, ")"),
            ParsedToken::new(Token::LeftCurlyBrace, "{"),
            ParsedToken::new(Token::RightCurlyBrace, "}"),
            ParsedToken::new(Token::Comma, ","),
            ParsedToken::new(Token::Semicolon, ";"),
            ParsedToken::new(Token::EOF, ""),
        ];

        for (idx, expected_token) in expected_tokens.iter().enumerate() {
            let token = lexer.next_token();

            if token != *expected_token {
                println!(
                    "tests[{}] - token type wrong, expected {:#?}, got {:#?}",
                    idx, expected_token, token
                )
            }
        }
    }
}
