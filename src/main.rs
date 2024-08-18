mod errors;
mod parsers;
mod tokens;

use crate::errors::ExitCode;
use crate::parsers::interpreter::Interpreter;
use crate::parsers::parser::Parser;
use crate::tokens::token::parse_tokens;
use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];
    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });

    let (tokens, exit_code) = parse_tokens(&file_contents);
    match command.as_str() {
        "tokenize" => {
            for token in tokens {
                println!("{}", token);
            }
        }
        "parse" => {
            let mut parser = Parser::new(&tokens);
            parser.parse();
            for expr in parser.exprs {
                println!("{}", expr);
            }
        }
        "evaluate" => {
            let mut parser = Parser::new(&tokens);
            parser.parse();
            let interpreter = Interpreter::new();
            for expr in parser.exprs {
                let value = interpreter.evaluate(expr);
                match value {
                    Ok(v) => println!("{v}"),
                    Err(e) => {
                        eprintln!("{e}");
                        exit(ExitCode::RuntimeError as i32);
                    }
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }

    if exit_code != 0 {
        exit(exit_code);
    }
}
