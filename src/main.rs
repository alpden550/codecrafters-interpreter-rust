mod errors;
mod evaluator;
mod tokens;

use crate::evaluator::evaluate;
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

    match command.as_str() {
        "tokenize" => {
            let (tokens, exit_code) = parse_tokens(&file_contents);
            for token in tokens {
                println!("{}", token);
            }
            if exit_code != 0 {
                exit(exit_code);
            }
        }
        "evaluate" => {
            let (tokens, exit_code) = parse_tokens(&file_contents);
            if exit_code != 0 {
                exit(exit_code);
            }
            let values = evaluate(&tokens);
            for value in values {
                println!("{}", value);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
