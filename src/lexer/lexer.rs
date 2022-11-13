use crate::tokens::Token;

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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.current_char {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            '{' => Token::LeftCurlyBrace,
            '}' => Token::RightCurlyBrace,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '0' => Token::EOF,
            v => {
                return if self.is_current_char_letter() {
                    Token::new(self.read_identifier())
                } else if self.is_current_char_digit() {
                    Token::Integer(self.read_number().to_string())
                } else {
                    Token::Illegal(v.to_string())
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
            Token::Let,
            Token::Identifier(String::from("five")),
            Token::Assign,
            Token::Integer(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("ten")),
            Token::Assign,
            Token::Integer(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LeftParenthesis,
            Token::Identifier(String::from("x")),
            Token::Comma,
            Token::Identifier(String::from("y")),
            Token::RightParenthesis,
            Token::LeftCurlyBrace,
            Token::Identifier(String::from("x")),
            Token::Plus,
            Token::Identifier(String::from("y")),
            Token::Semicolon,
            Token::RightCurlyBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("result")),
            Token::Assign,
            Token::Identifier(String::from("add")),
            Token::LeftParenthesis,
            Token::Identifier(String::from("five")),
            Token::Comma,
            Token::Identifier(String::from("ten")),
            Token::RightParenthesis,
            Token::Semicolon,
            Token::EOF,
        ];

        for (idx, expected_token) in expected_tokens.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(*expected_token, token, "Error at position {idx}");
        }
    }
}
