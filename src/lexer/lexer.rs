use crate::tokens::{ParsedToken, Token};

// TODO: maybe current_char should be grouped with position?
// TODO: support unicode and emojis?
// Transforms source code into tokens
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
        self.skip_whitespace();

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
            _ => {
                if self.is_current_char_letter() {
                    let identifier = self.read_identifier();
                    return ParsedToken::new(Token::new(identifier), identifier);
                } else if self.is_current_char_digit() {
                    let number = self.read_number();
                    return ParsedToken::new(Token::Integer, number);
                } else {
                    return ParsedToken::new(
                        Token::Illegal,
                        self.current_char.to_string().as_str(),
                    );
                }
            }
        };

        self.read_char();

        token
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == ' '
            || self.current_char == '\t'
            || self.current_char == '\n'
            || self.current_char == '\r'
        {
            self.read_char();
        }
    }

    // TODO: refactor read_identifier & read_number into single function?
    fn read_identifier(&mut self) -> &str {
        let position = self.position;

        while self.is_current_char_letter() {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;

        while self.is_current_char_digit() {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn is_current_char_letter(&self) -> bool {
        self.current_char.is_alphabetic() || self.current_char == '_'
    }

    fn is_current_char_digit(&self) -> bool {
        self.current_char.is_numeric()
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
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        ";

        let mut lexer = Lexer::new(input);

        let expected_tokens = vec![
            ParsedToken::new(Token::Let, "let"),
            ParsedToken::new(Token::Identifier, "five"),
            ParsedToken::new(Token::Assign, "="),
            ParsedToken::new(Token::Integer, "5"),
            ParsedToken::new(Token::Semicolon, ";"),
            ParsedToken::new(Token::Let, "let"),
            ParsedToken::new(Token::Identifier, "ten"),
            ParsedToken::new(Token::Assign, "="),
            ParsedToken::new(Token::Integer, "10"),
            ParsedToken::new(Token::Semicolon, ";"),
            ParsedToken::new(Token::Let, "let"),
            ParsedToken::new(Token::Identifier, "add"),
            ParsedToken::new(Token::Assign, "="),
            ParsedToken::new(Token::Function, "fn"),
            ParsedToken::new(Token::LeftParenthesis, "("),
            ParsedToken::new(Token::Identifier, "x"),
            ParsedToken::new(Token::Comma, ","),
            ParsedToken::new(Token::Identifier, "y"),
            ParsedToken::new(Token::RightParenthesis, ")"),
            ParsedToken::new(Token::LeftCurlyBrace, "{"),
            ParsedToken::new(Token::Identifier, "x"),
            ParsedToken::new(Token::Plus, "+"),
            ParsedToken::new(Token::Identifier, "y"),
            ParsedToken::new(Token::Semicolon, ";"),
            ParsedToken::new(Token::RightCurlyBrace, "}"),
            ParsedToken::new(Token::Semicolon, ";"),
            ParsedToken::new(Token::Let, "let"),
            ParsedToken::new(Token::Identifier, "result"),
            ParsedToken::new(Token::Assign, "="),
            ParsedToken::new(Token::Identifier, "add"),
            ParsedToken::new(Token::LeftParenthesis, "("),
            ParsedToken::new(Token::Identifier, "five"),
            ParsedToken::new(Token::Comma, ","),
            ParsedToken::new(Token::Identifier, "ten"),
            ParsedToken::new(Token::RightParenthesis, ")"),
            ParsedToken::new(Token::Semicolon, ";"),
            ParsedToken::new(Token::EOF, ""),
        ];

        for (idx, expected_token) in expected_tokens.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(*expected_token, token, "Error at position {idx}");
        }
    }
}
