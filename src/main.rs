mod tokens;

use crate::tokens::token_type::TokenType;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                let exit_code = parse_file_content(&file_contents);
                if exit_code != 0 {
                    exit(exit_code);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn parse_file_content(file_contents: &String) -> i32 {
    let mut exit_code = 0;
    let lines = file_contents.lines();

    for (line_number, line) in lines.enumerate() {
        for char in line.chars() {
            match char {
                '(' => println!("{} {char} null", TokenType::LeftParen),
                ')' => println!("{} {char} null", TokenType::RightParen),
                '{' => println!("{} {char} null", TokenType::LeftBrace),
                '}' => println!("{} {char} null", TokenType::RightBrace),
                ',' => println!("{} {char} null", TokenType::Comma),
                '.' => println!("{} {char} null", TokenType::Dot),
                '-' => println!("{} {char} null", TokenType::Minus),
                '+' => println!("{} {char} null", TokenType::Plus),
                ';' => println!("{} {char} null", TokenType::Semicolon),
                '*' => println!("{} {char} null", TokenType::Star),
                '/' => println!("{} {char} null", TokenType::Slash),
                _ => {
                    eprintln!(
                        "[line {}] Error: Unexpected character: {char}",
                        line_number + 1
                    );
                    exit_code = 65;
                }
            }
        }
    }

    println!("{}  null", TokenType::Eof);
    exit_code
}
