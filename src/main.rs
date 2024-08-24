mod environments;
mod errors;
mod interpreter;
mod models;
mod parser;
mod scanner;

use crate::errors::ExitCode;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::parse_tokens;
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        exit(ExitCode::ExitError as i32);
    }

    let filename = &args[1];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        exit(ExitCode::ExitError as i32);
    });

    let (tokens, exit_code) = parse_tokens(&file_contents);
    if exit_code != 0 {
        exit(exit_code);
    }

    let mut parser = Parser::new(&tokens);
    parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&parser.stmts);

    if !parser.errors.is_empty() {
        for error in parser.errors {
            eprintln!("{error}");
        }
        exit(ExitCode::RuntimeError as i32);
    }
}
