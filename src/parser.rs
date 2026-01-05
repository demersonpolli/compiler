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
pub enum PrintItem {
    Expr(Expression),
    String(String),
}

/// Represents an executable line or block in the language.
#[derive(Debug, Clone)]
pub enum Statement {
    /// LET <var> = <expr>
    Let {
        var: String,
        value: Expression,
    },
    /// PRINT <item1>, <item2>, ... [; or ,]
    Print {
        items: Vec<PrintItem>,
        newline: bool,
    },
    /// FOR <var> = <start> TO <end> ... NEXT
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

    /// Parse a LET statement: LET X = 10
    fn parse_let(&mut self) -> Statement {
        self.expect(Token::Let);

        let var = match self.current_token().clone() {
            Token::Identifier(name) => {
                self.advance();
                name
            }
            _ => panic!("Expected identifier after LET"),
        };

        self.expect(Token::Equals);
        let expr = self.parse_expr();

        Statement::Let { var, value: expr }
    }

    /// Parse a PRINT statement: PRINT X, "HELLO";
    fn parse_print(&mut self) -> Statement {
        self.expect(Token::Print);
        
        let mut items = Vec::new();
        let mut newline = true;

        loop {
            match self.current_token() {
                Token::Newline | Token::Eof => break,
                Token::String(s) => {
                    let s_clone = s.clone();
                    self.advance();
                    items.push(PrintItem::String(s_clone));
                }
                Token::Comma => {
                    // In some BASICs, a comma by itself might mean a tab/space
                    // but usually it's a separator. If it's at the end, it suppresses newline.
                    self.advance();
                    if matches!(self.current_token(), Token::Newline | Token::Eof) {
                        newline = false;
                        break;
                    }
                }
                Token::Semicolon => {
                    self.advance();
                    if matches!(self.current_token(), Token::Newline | Token::Eof) {
                        newline = false;
                        break;
                    }
                }
                _ => {
                    // Try to parse an expression
                    items.push(PrintItem::Expr(self.parse_expr()));
                }
            }

            // Check for separators or end of line
            match self.current_token() {
                Token::Comma | Token::Semicolon => {
                    let tok = self.current_token().clone();
                    self.advance();
                    if matches!(self.current_token(), Token::Newline | Token::Eof) {
                        newline = false;
                        break;
                    }
                }
                _ => break,
            }
        }

        Statement::Print { items, newline }
    }

    /// Parse a FOR loop: FOR I = 1 TO 5 ... NEXT
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
        // Continue parsing statements until we hit NEXT or EOF
        while self.current_token() != &Token::Next && self.current_token() != &Token::Eof {
            body.push(self.parse_statement());
            self.skip_newlines();
        }

        self.expect(Token::Next);
        
        // Note: BASIC often allows NEXT <var>, but we simplify to just NEXT for now.

        Statement::For { var, start, end, body }
    }

    /// Entry point for parsing any statement.
    /// To support IF, GOTO, etc., add new matches here.
    fn parse_statement(&mut self) -> Statement {
        self.skip_newlines();

        match self.current_token() {
            Token::Let => self.parse_let(),
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

