use crate::tokens::Token;

#[derive(PartialEq)]
enum CharType {
    Letter,
    Digit,
}

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
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '!' => Token::Bang,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '0' => Token::EOF,
            v => {
                return if self.current_char_is_of_type(&CharType::Letter) {
                    Token::new(self.read_value(CharType::Letter))
                } else if self.current_char_is_of_type(&CharType::Digit) {
                    Token::Integer(self.read_value(CharType::Digit).to_string())
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

    fn read_value(&mut self, char_type: CharType) -> &str {
        let position = self.position;

        while self.current_char_is_of_type(&char_type) {
            self.read_char();
        }

        &self.input[position..self.position]
    }

    fn current_char_is_of_type(&self, char_type: &CharType) -> bool {
        match char_type {
            CharType::Letter => self.current_char.is_alphabetic() || self.current_char == '_',
            CharType::Digit => self.current_char.is_numeric(),
        }
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
        !-/*5;
        5 < 10 > 5;
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(String::from("5")),
            Token::Semicolon,
            Token::Integer(String::from("5")),
            Token::LessThan,
            Token::Integer(String::from("10")),
            Token::GreaterThan,
            Token::Integer(String::from("5")),
            Token::Semicolon,
            Token::EOF,
        ];

        for (idx, expected_token) in expected_tokens.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(*expected_token, token, "Error at position {idx}");
        }
    }
}
