mod lexer;
mod parser;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use std::env;
use std::fs;
use std::io::{self, Write};
use crate::codegen::CodeGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_file> [output_file]", args[0]);
        eprintln!("If output_file is not specified, outputs to stdout.");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let input = fs::read_to_string(input_file)
        .unwrap_or_else(|err| {
            eprintln!("Error reading file '{}': {}", input_file, err);
            std::process::exit(1);
        });

    let mut lexer = Lexer::new(&input);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut codegen = CodeGenerator::new();
    let c_code = codegen.generate(&ast);

    if args.len() >= 3 {
        let output_file = &args[2];
        fs::write(output_file, &c_code)
            .unwrap_or_else(|err| {
                eprintln!("Error writing file '{}': {}", output_file, err);
                std::process::exit(1);
            });
        println!("Compiled successfully! Output written to '{}'", output_file);
    }
    else {
        io::stdout().write_all(c_code.as_bytes()).unwrap();
    }
}
