use crate::parser::{BinOp, Expression, Statement};
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
            Expression::Number(n) => n.to_string(),
            Expression::Variable(name) => name.clone(),
            Expression::BinaryOp { left, operator, right } => {
                let left_string = self.generate_expr(left);
                let right_string = self.generate_expr(right);
                let operator_string = match operator {
                    BinOp::Add => "+",
                    BinOp::Subtract => "-",
                    BinOp::Multiply => "*",
                    BinOp::Divide => "/",
                };
                // Adding parentheses to respect operator precedence in the target C code
                format!("({} {} {})", left_string, operator_string, right_string)
            }
        }
    }

    fn collect_variables(&mut self, statements: &[Statement]) {
        for stmt in statements {
            match stmt {
                Statement::Set { var, .. } => {
                    self.variables.insert(var.clone());
                }
                Statement::For { var, body, .. } => {
                    self.variables.insert(var.clone());
                    self.collect_variables(body);
                }
                Statement::Print { .. } => {}
            }
        }
    }

    fn generate_statements(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Set { var, value } => {
                let value_string = self.generate_expr(value);
                format!("{}{} = {};\n", self.indent(), var, value_string)
            }
            Statement::Print { expr } => {
                let expr_string = self.generate_expr(expr);
                format!("{}printf(\"%lld\\n\", (long long){});\n", self.indent(), expr_string)
            }
            Statement::For { var, start, end, body } => {
                let start_string = self.generate_expr(start);
                let end_string = self.generate_expr(end);
                let mut result = format!(
                    "{}for ({} = {}; {} <= {}; {}++) {{\n",
                    self.indent(),
                    var,
                    start_string,
                    var,
                    end_string,
                    var
                );

                self.indent_level += 1;
                for stmt in body {
                    result.push_str(&self.generate_statements(stmt));
                }
                self.indent_level -= 1;

                result.push_str(&format!("{}}}\n", self.indent()));
                result
            }
        }
    }

    pub fn generate(&mut self, statements: &[Statement]) -> String {
        self.collect_variables(statements);

        let mut result = String::new();
        result.push_str("#include <stdio.h>\n\n");
        result.push_str("int main() {\n");

        if !self.variables.is_empty() {
            result.push_str("    long long ");
            let vars: Vec<String> = self.variables.iter().cloned().collect();
            result.push_str(&vars.join(", "));
            result.push_str(";\n\n"); // Added double newline for spacing
        }

        for stmt in statements {
            result.push_str(&self.generate_statements(stmt));
        }

        result.push_str("\n    return 0;\n");
        result.push_str("}\n");

        result
    }
}
