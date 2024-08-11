mod tokens;

use crate::tokens::token::Token;
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
                let (tokens, exit_code) = parse_file_content(&file_contents);
                for token in tokens {
                    println!("{}", token);
                }
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

fn parse_file_content(file_contents: &String) -> (Vec<Token>, i32) {
    let mut exit_code = 0;
    let mut tokens: Vec<Token> = vec![];
    let doubled_tokens = ['!', '=', '<', '>'];

    let lines = file_contents.lines();
    for (line_number, line) in lines.enumerate() {
        let mut chars = line.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '(' => tokens.push(Token::new(TokenType::LeftParen, c.to_string(), None)),
                ')' => tokens.push(Token::new(TokenType::RightParen, c.to_string(), None)),
                '{' => tokens.push(Token::new(TokenType::LeftBrace, c.to_string(), None)),
                '}' => tokens.push(Token::new(TokenType::RightBrace, c.to_string(), None)),
                ',' => tokens.push(Token::new(TokenType::Comma, c.to_string(), None)),
                '.' => tokens.push(Token::new(TokenType::Dot, c.to_string(), None)),
                '-' => tokens.push(Token::new(TokenType::Minus, c.to_string(), None)),
                '+' => tokens.push(Token::new(TokenType::Plus, c.to_string(), None)),
                ';' => tokens.push(Token::new(TokenType::Semicolon, c.to_string(), None)),
                '*' => tokens.push(Token::new(TokenType::Star, c.to_string(), None)),
                '/' => tokens.push(Token::new(TokenType::Slash, c.to_string(), None)),
                '!' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        let formatted = format!("{}{}", c, next);
                        tokens.push(Token::new(
                            TokenType::BangEqual,
                            formatted.to_string(),
                            None,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(TokenType::Bang, c.to_string(), None));
                    }
                },
                '=' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        let formatted = format!("{}{}", c, next);
                        tokens.push(Token::new(
                            TokenType::EqualEqual,
                            formatted.to_string(),
                            None,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(TokenType::Equal, c.to_string(), None));
                    }
                },
                '>' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        let formatted = format!("{}{}", c, next);
                        tokens.push(Token::new(
                            TokenType::GreaterEqual,
                            formatted.to_string(),
                            None,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(TokenType::Greater, c.to_string(), None));
                    }
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        let formatted = format!("{}{}", c, next);
                        tokens.push(Token::new(
                            TokenType::LessEqual,
                            formatted.to_string(),
                            None,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(TokenType::Less, c.to_string(), None));
                    }
                },
                _ => {
                    print_error_line(line_number + 1, c);
                    exit_code = 65;
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, "  ".to_string(), None));
    (tokens, exit_code)
}

fn print_error_line(line_number: usize, token: char) {
    eprintln!(
        "[line {}] Error: Unexpected character: {token}",
        line_number
    );
}
