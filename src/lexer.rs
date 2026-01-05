#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Set,
    Print,
    For,
    To,
    EndFor,
    Identifier(String),
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    LeftParen,
    RightParen,
    Newline,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        }
        else {
            None
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.advance();
            }
            else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> i64 {
        let mut num_string = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                num_string.push(ch);
                self.advance();
            }
            else {
                break;
            }
        }
        num_string.parse().unwrap()
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            }
            else {
                break;
            }
        }
        identifier
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.current_char() {
            None => Token::Eof,
            Some('\n') => {
                self.advance();
                Token::Newline
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Multiply
            }
            Some('/') => {
                self.advance();
                Token::Divide
            }
            Some('=') => {
                self.advance();
                Token::Equals
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some(ch) if ch.is_ascii_digit() => {
                let number = self.read_number();
                Token::Number(number)
            }
            Some(ch) if ch.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.to_uppercase().as_str() {
                    "SET" => Token::Set,
                    "PRINT" => Token::Print,
                    "FOR" => Token::For,
                    "TO" => Token::To,
                    "ENDFOR" => Token::EndFor,
                    _ => Token::Identifier(identifier),
                }
            }
            Some(ch) => {
                panic!("Unrecognized character: {}", ch);
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}
