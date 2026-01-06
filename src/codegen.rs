use crate::parser::{BinOp, Expression, Statement, StatementNode, PrintItem};
use crate::lexer::Token;
use std::collections::BTreeSet;

pub struct CodeGenerator {
    indent_level: usize,
    variables: BTreeSet<String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            indent_level: 1,
            variables: BTreeSet::new(),
        }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn generate_expr(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(n) => format!("{}.0", n),
            Expression::Float(f) => f.to_string(),
            Expression::Variable(name) => name.clone(),
            Expression::BinaryOp { left, operator, right } => {
                let left_string = self.generate_expr(left);
                let right_string = self.generate_expr(right);
                match operator {
                    BinOp::Add => format!("({} + {})", left_string, right_string),
                    BinOp::Subtract => format!("({} - {})", left_string, right_string),
                    BinOp::Multiply => format!("({} * {})", left_string, right_string),
                    BinOp::Divide => format!("({} / {})", left_string, right_string),
                    BinOp::Power => format!("pow({}, {})", left_string, right_string),
                }
            }
            Expression::FunctionCall { name, args } => {
                let args_string: Vec<String> = args.iter().map(|a| self.generate_expr(a)).collect();
                match name.to_uppercase().as_str() {
                    "INT" => format!("floor({})", args_string[0]),
                    "RND" => "((double)rand() / (double)RAND_MAX)".to_string(),
                    "SQR" => format!("sqrt({})", args_string[0]),
                    "EXP" => format!("exp({})", args_string[0]),
                    "ABS" => format!("fabs({})", args_string[0]),
                    _ => format!("{}({})", name, args_string.join(", ")),
                }
            }
        }
    }

    fn collect_variables_node(&mut self, node: &StatementNode) {
        match node {
            StatementNode::Let { var, .. } => {
                self.variables.insert(var.clone());
            }
            StatementNode::For { var, body, .. } => {
                self.variables.insert(var.clone());
                self.collect_variables(body);
            }
            StatementNode::Input(var) => {
                self.variables.insert(var.clone());
            }
            StatementNode::If { then_part, .. } => {
                self.collect_variables_node(&then_part.node);
            }
            _ => {}
        }
    }

    fn collect_variables(&mut self, statements: &[Statement]) {
        for stmt in statements {
            self.collect_variables_node(&stmt.node);
        }
    }

    fn generate_statement_node(&mut self, node: &StatementNode) -> String {
        match node {
            StatementNode::Let { var, value } => {
                let value_string = self.generate_expr(value);
                format!("{}{} = {};\n", self.indent(), var, value_string)
            }
            StatementNode::Print { items, newline } => {
                let mut result = String::new();
                for item in items {
                    match item {
                        PrintItem::String(s) => {
                            result.push_str(&format!("{}printf(\"%s\", \"{}\");\n", self.indent(), s));
                        }
                        PrintItem::Expr(expr) => {
                            let expr_string = self.generate_expr(expr);
                            // We use %g for general float printing
                            result.push_str(&format!("{}printf(\"%g \", {});\n", self.indent(), expr_string));
                        }
                    }
                }
                if *newline {
                    result.push_str(&format!("{}printf(\"\\n\");\n", self.indent()));
                }
                result
            }
            StatementNode::For { var, start, end, step, body } => {
                let start_string = self.generate_expr(start);
                let end_string = self.generate_expr(end);
                let step_string = step.as_ref().map(|s| self.generate_expr(s)).unwrap_or("1.0".to_string());
                
                let mut result = format!(
                    "{}for ({} = {}; {} <= {}; {} += {}) {{\n",
                    self.indent(),
                    var,
                    start_string,
                    var,
                    end_string,
                    var,
                    step_string
                );

                self.indent_level += 1;
                for stmt in body {
                    result.push_str(&self.generate_statements_internal(stmt));
                }
                self.indent_level -= 1;

                result.push_str(&format!("{}}}\n", self.indent()));
                result
            }
            StatementNode::If { left, op, right, then_part } => {
                let left_string = self.generate_expr(left);
                let right_string = self.generate_expr(right);
                let op_string = match op {
                    Token::Equal         => "=",
                    Token::NotEqual       => "!=",
                    Token::LessThan       => "<",
                    Token::LessOrEqual    => "<=",
                    Token::GreaterThan    => ">",
                    Token::GreaterOrEqual => ">=",
                    _ => "==",
                };
                let mut result = format!("{}if ({} {} {}) {{\n", self.indent(), left_string, op_string, right_string);
                self.indent_level += 1;
                result.push_str(&self.generate_statements_internal(then_part));
                self.indent_level -= 1;
                result.push_str(&format!("{}}}\n", self.indent()));
                result
            }
            StatementNode::Goto(line) => {
                format!("{}goto line{};\n", self.indent(), line)
            }
            StatementNode::Input(var) => {
                format!("{}printf(\"? \"); scanf(\"%lf\", &{});\n", self.indent(), var)
            }
            StatementNode::Rem => "".to_string(),
            StatementNode::End => format!("{}return 0;\n", self.indent()),
        }
    }

    fn generate_statements_internal(&mut self, stmt: &Statement) -> String {
        let mut result = String::new();
        if let Some(label) = stmt.label {
            result.push_str(&format!("line{}:\n", label));
        }
        result.push_str(&self.generate_statement_node(&stmt.node));
        result
    }

    pub fn generate(&mut self, statements: &[Statement]) -> String {
        self.collect_variables(statements);

        let mut result = String::new();
        result.push_str("#include <stdio.h>\n");
        result.push_str("#include <stdlib.h>\n");
        result.push_str("#include <math.h>\n");
        result.push_str("#include <time.h>\n\n");
        result.push_str("int main() {\n");
        result.push_str("    srand(time(NULL));\n");

        if !self.variables.is_empty() {
            result.push_str("    double ");
            let vars: Vec<String> = self.variables.iter().cloned().collect();
            result.push_str(&vars.join(", "));
            result.push_str(";\n\n");
        }

        for stmt in statements {
            result.push_str(&self.generate_statements_internal(stmt));
        }

        result.push_str("\n    return 0;\n");
        result.push_str("}\n");

        result
    }
}
