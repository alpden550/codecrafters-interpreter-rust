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
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '(' => println!("{} {c} null", TokenType::LeftParen),
                ')' => println!("{} {c} null", TokenType::RightParen),
                '{' => println!("{} {c} null", TokenType::LeftBrace),
                '}' => println!("{} {c} null", TokenType::RightBrace),
                ',' => println!("{} {c} null", TokenType::Comma),
                '.' => println!("{} {c} null", TokenType::Dot),
                '-' => println!("{} {c} null", TokenType::Minus),
                '+' => println!("{} {c} null", TokenType::Plus),
                ';' => println!("{} {c} null", TokenType::Semicolon),
                '*' => println!("{} {c} null", TokenType::Star),
                '/' => println!("{} {c} null", TokenType::Slash),
                '!' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        println!("{} {c}{next} null", TokenType::BangEqual);
                    }
                    _ => {
                        println!("{} {c} null", TokenType::Bang)
                    }
                },
                '=' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        println!("{} {c}{next} null", TokenType::EqualEqual);
                    }
                    _ => {
                        println!("{} {c} null", TokenType::Equal)
                    }
                },
                '>' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        println!("{} {c}{next} null", TokenType::GreaterEqual);
                    }
                    _ => {
                        println!("{} {c} null", TokenType::Greater)
                    }
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        println!("{} {c}{next} null", TokenType::LessEqual);
                    }
                    _ => {
                        println!("{} {c} null", TokenType::Less)
                    }
                },
                _ => {
                    print_error_line(line_number, c);
                    exit_code = 65;
                }
            }
        }
    }

    println!("{}  null", TokenType::Eof);
    exit_code
}

fn print_error_line(line_number: usize, token: char) {
    eprintln!(
        "[line {}] Error: Unexpected character: {token}",
        line_number + 1
    );
}
