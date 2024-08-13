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
                let (tokens, exit_code) = parse_tokens(&file_contents);
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

fn parse_tokens(file_contents: &String) -> (Vec<Token>, i32) {
    let mut exit_code = 0;
    let mut tokens: Vec<Token> = vec![];

    let lines = file_contents.lines();
    for (line_number, line) in lines.enumerate() {
        let mut chars = line.chars().peekable();

        'line_loop: while let Some(c) = chars.next() {
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
                '/' => match chars.peek() {
                    Some('/') => {
                        tokens.push(Token::new(
                            TokenType::Comment,
                            TokenType::Comment.to_string().to_lowercase(),
                            line.replace("//", "").trim().to_string().into(),
                        ));
                        break 'line_loop;
                    }
                    _ => tokens.push(Token::new(TokenType::Slash, c.to_string(), None)),
                },
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
                ' ' | '\t' | '\r' => {}
                '"' => {
                    let mut str_value = String::new();
                    loop {
                        let value = chars.next();
                        match value {
                            Some('"') => break,
                            Some(_) => str_value.push(value.unwrap()),
                            None => {
                                eprintln!("[line {}] Error: Unterminated string.", line_number + 1);
                                exit_code = 65;
                                break 'line_loop;
                            }
                        }
                    }
                    tokens.push(Token::new(
                        TokenType::String,
                        format!("\"{}\"", str_value),
                        str_value.into(),
                    ));
                }
                token if token.is_digit(10) => {
                    let mut num_value = String::from(token);
                    let mut is_dot = false;

                    while let Some(t) = chars.peek() {
                        if t.is_digit(10) {
                            num_value.push(*t);
                            chars.next();
                        } else if *t == '.' && !is_dot {
                            num_value.push(*t);
                            chars.next();
                            is_dot = true;
                        } else {
                            break;
                        }
                    }

                    tokens.push(parse_number(&num_value));
                    if num_value.ends_with(".") {
                        tokens.push(Token::new(TokenType::Dot, '.'.to_string(), None));
                    }
                }
                token if token.is_alphanumeric() || token == '_' => {
                    let mut identifier = String::from(token);
                    while let Some(t) = chars.peek() {
                        if t.is_alphanumeric() || *t == '_' {
                            identifier.push(*t);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let token_type = TokenType::get_keyword_or_identifier(identifier.as_str());
                    tokens.push(Token::new(token_type, identifier, None));
                }
                _ => {
                    print_error_token_line(line_number + 1, c);
                    exit_code = 65;
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, "".to_string(), None));
    (tokens, exit_code)
}

fn print_error_token_line(line_number: usize, token: char) {
    eprintln!(
        "[line {}] Error: Unexpected character: {token}",
        line_number
    );
}

fn parse_number(raw_value: &String) -> Token {
    let mut name = String::from(raw_value);
    let mut value = String::from(raw_value);
    if raw_value.ends_with(".") {
        name = name.replace(".", "");
        value.push('0');
    } else if !raw_value.contains(".") {
        value.push('.');
        value.push('0');
    }

    if value.ends_with("00") {
        value.pop();
    }

    Token::new(TokenType::Number, name, value.into())
}
