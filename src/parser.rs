use crate::lexer::{Token};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(i64),
    Float(f64),
    Variable(String),
    BinaryOp {
        left: Box<Expression>,
        operator: BinOp,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    }
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Debug, Clone)]
pub enum PrintItem {
    Expr(Expression),
    String(String),
}

/// Represents an executable line or block in the language.
#[derive(Debug, Clone)]
pub struct Statement {
    pub label: Option<i64>,
    pub node: StatementNode,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
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
    /// FOR <var> = <start> TO <end> [STEP <step>] ... NEXT
    For {
        var: String,
        start: Expression,
        end: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
    /// IF <left> <op> <right> THEN <line_or_stmt>
    If {
        left: Expression,
        op: Token, // Using Token for simplicity in comparison ops
        right: Expression,
        then_part: Box<Statement>,
    },
    /// GOTO <line>
    Goto(i64),
    /// INPUT <var>
    Input(String),
    /// REM <comment>
    Rem,
    /// END
    End,
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
            Token::Float(f) => {
                self.advance();
                Expression::Float(f)
            }
            Token::Identifier(name) => {
                self.advance();
                if self.current_token() == &Token::LeftParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current_token() != &Token::RightParen {
                        loop {
                            args.push(self.parse_expr());
                            if self.current_token() == &Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RightParen);
                    Expression::FunctionCall { name, args }
                } else {
                    Expression::Variable(name)
                }
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

    fn parse_power(&mut self) -> Expression {
        let mut left = self.parse_primary();

        while self.current_token() == &Token::OperatorPower {
            self.advance();
            let right = self.parse_power(); // Right associative
            left = Expression::BinaryOp {
                left: Box::new(left),
                operator: BinOp::Power,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_term(&mut self) -> Expression {
        let mut left = self.parse_power();

        while matches!(self.current_token(), Token::OperatorMultiply | Token::OperatorDivide) {
            let op = match self.current_token() {
                Token::OperatorMultiply => BinOp::Multiply,
                Token::OperatorDivide => BinOp::Divide,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_power();
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

        while matches!(self.current_token(), Token::OperatorAdd | Token::OperatorSubtract) {
            let op = match self.current_token() {
                Token::OperatorAdd => BinOp::Add,
                Token::OperatorSubtract => BinOp::Subtract,
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
    fn parse_let(&mut self) -> StatementNode {
        self.expect(Token::Let);

        let var = match self.current_token().clone() {
            Token::Identifier(name) => {
                self.advance();
                name
            }
            _ => panic!("Expected identifier after LET"),
        };

        self.expect(Token::Equal);
        let expr = self.parse_expr();

        StatementNode::Let { var, value: expr }
    }

    /// Parse a PRINT statement: PRINT X, "HELLO";
    fn parse_print(&mut self) -> StatementNode {
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
                    items.push(PrintItem::Expr(self.parse_expr()));
                }
            }

            match self.current_token() {
                Token::Comma | Token::Semicolon => {
                    self.advance();
                    if matches!(self.current_token(), Token::Newline | Token::Eof) {
                        newline = false;
                        break;
                    }
                }
                _ => break,
            }
        }

        StatementNode::Print { items, newline }
    }

    /// Parse a FOR loop: FOR I = 1 TO 5 STEP 2 ... NEXT
    fn parse_for(&mut self) -> StatementNode {
        self.expect(Token::For);

        let var = match self.current_token().clone() {
            Token::Identifier(name) => {
                self.advance();
                name
            }
            _ => panic!("Expected identifier after FOR"),
        };
        self.expect(Token::Equal);
        let start = self.parse_expr();
        self.expect(Token::To);
        let end = self.parse_expr();

        let mut step = None;
        if self.current_token() == &Token::Step {
            self.advance();
            step = Some(self.parse_expr());
        }

        self.skip_newlines();

        let mut body = Vec::new();
        while self.current_token() != &Token::Next && self.current_token() != &Token::Eof {
            body.push(self.parse_statement());
            self.skip_newlines();
        }

        self.expect(Token::Next);
        
        // Optional NEXT <var>
        if let Token::Identifier(_) = self.current_token() {
            self.advance();
        }

        StatementNode::For { var, start, end, step, body }
    }

    fn parse_if(&mut self) -> StatementNode {
        self.expect(Token::If);
        let left = self.parse_expr();

        // Using '[' as '<=' and ']' as '>='
        let op = match self.current_token().clone() {
            Token::Equal | Token::NotEqual |
            Token::LessThan | Token::LessOrEqual |
            Token::GreaterThan | Token::GreaterOrEqual => {
                let t = self.current_token().clone();
                self.advance();
                t
            }
            _ => panic!("Expected comparison operator in IF"),
        };
        
        let right = self.parse_expr();
        self.expect(Token::Then);

        let then_stmt = if let Token::Number(line) = self.current_token() {
            let l = *line;
            self.advance();
            Statement { label: None, node: StatementNode::Goto(l) }
        } else {
            self.parse_statement()
        };

        StatementNode::If { left, op, right, then_part: Box::new(then_stmt) }
    }

    fn parse_goto(&mut self) -> StatementNode {
        self.expect(Token::Goto);
        match self.current_token() {
            Token::Number(line) => {
                let l = *line;
                self.advance();
                StatementNode::Goto(l)
            }
            _ => panic!("Expected line number after GOTO"),
        }
    }

    fn parse_input(&mut self) -> StatementNode {
        self.expect(Token::Input);
        let var = match self.current_token().clone() {
            Token::Identifier(name) => {
                self.advance();
                name
            }
            _ => panic!("Expected identifier after INPUT"),
        };
        StatementNode::Input(var)
    }

    /// Entry point for parsing any statement.
    fn parse_statement(&mut self) -> Statement {
        self.skip_newlines();

        let label = if let Token::Number(n) = self.current_token() {
            let l = *n;
            self.advance();
            Some(l)
        } else {
            None
        };

        let node = match self.current_token() {
            Token::Let => self.parse_let(),
            Token::Print => self.parse_print(),
            Token::For => self.parse_for(),
            Token::If => self.parse_if(),
            Token::Goto => self.parse_goto(),
            Token::Input => self.parse_input(),
            Token::Rem => {
                self.advance();
                StatementNode::Rem
            }
            Token::End => {
                self.advance();
                StatementNode::End
            }
            Token::Newline | Token::Eof => {
                // Just a line number or empty line
                StatementNode::Rem
            }
            _ => panic!("Unexpected token at start of statement: {:?}", self.current_token()),
        };

        Statement { label, node }
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

