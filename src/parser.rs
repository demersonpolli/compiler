use crate::lexer::{Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        operator: BinOp,
        right: Box<Expression>,
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Set {
        var: String,
        value: Expression,
    },
    Print {
        expr: Expression,
    },
    For {
        var: String,
        start: Expression,
        end: Expression,
        body: Vec<Statement>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn current_token(&self) -> &Token {
        if self.position < self.tokens.len() {
            &self.tokens[self.position]
        }
        else {
            &Token::Eof
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, expected: Token) {
        if self.current_token() != &expected {
            panic!("expected {:?}, got {:?}", expected, self.current_token());
        }
        self.advance();
    }

    fn skip_newlines(&mut self) {
        while self.current_token() == &Token::Newline {
            self.advance();
        }
    }

    fn parse_primary(&mut self) -> Expression {
        match self.current_token().clone() {
            Token::Number(n) => {
                self.advance();
                Expression::Number(n)
            }
            Token::Identifier(name) => {
                self.advance();
                Expression::Variable(name)
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.parse_expr();
                self.expect(Token::RightParen);
                expr
            }
            _ => panic!("Unexpected token in expression: {:?}", self.current_token()),
        }
    }

    fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_primary();

        while matches!(self.current_token(), Token::Multiply | Token::Divide) {
            let op = match self.current_token() {
                Token::Multiply => BinOp::Multiply,
                Token::Divide => BinOp::Divide,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_primary();
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_expr(&mut self) -> Expression {
        let mut left = self.parse_term();

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = match self.current_token() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Subtract,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term();
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }

        left
    }

    fn parse_set(&mut self) -> Statement {
        self.expect(Token::Set);

        let var = match self.current_token().clone() {
            Token::Identifier(name) => {
                self.advance();
                name
            }
            _ => panic!("Expected identifier after SET"),
        };

        self.expect(Token::Equals);
        let expr = self.parse_expr();

        Statement::Set { var, value: expr }
    }

    fn parse_print(&mut self) -> Statement {
        self.expect(Token::Print);
        self.expect(Token::LeftParen);
        let expr = self.parse_expr();
        self.expect(Token::RightParen);

        Statement::Print { expr }
    }

    fn parse_for(&mut self) -> Statement {
        self.expect(Token::For);

        let var = match self.current_token().clone() {
            Token::Identifier(name) => {
                self.advance();
                name
            }
            _ => panic!("Expected identifier after FOR"),
        };
        self.expect(Token::Equals);
        let start = self.parse_expr();
        self.expect(Token::To);
        let end = self.parse_expr();

        self.skip_newlines();

        let mut body = Vec::new();
        while self.current_token() != &Token::EndFor && self.current_token() != &Token::Eof {
            body.push(self.parse_statement());
            self.skip_newlines();
        }

        self.expect(Token::EndFor);

        Statement::For { var, start, end, body }
    }

    fn parse_statement(&mut self) -> Statement {
        self.skip_newlines();

        match self.current_token() {
            Token::Set => self.parse_set(),
            Token::Print => self.parse_print(),
            Token::For => self.parse_for(),
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        self.skip_newlines();
        while self.current_token() != &Token::Eof {
            statements.push(self.parse_statement());
            self.skip_newlines();
        }

        statements
    }
}

