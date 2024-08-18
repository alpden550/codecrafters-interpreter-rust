#![allow(dead_code)]

use crate::tokens::token_type::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub name: String,
    pub value: Option<String>,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        name: String,
        value: Option<String>,
        line_number: usize,
    ) -> Self {
        Token {
            token_type,
            name,
            value,
            line_number,
        }
    }

    fn parse_number(raw_value: &String, line_number: usize) -> Self {
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

        Token::new(TokenType::Number, name, value.into(), line_number)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.name,
            self.value.clone().unwrap_or(String::from("null")),
        )
    }
}

pub fn parse_tokens(file_contents: &String) -> (Vec<Token>, i32) {
    let mut exit_code = 0;
    let mut tokens: Vec<Token> = vec![];

    let lines = file_contents.lines();
    for (line_number, line) in lines.enumerate() {
        let mut chars = line.chars().peekable();
        let line_number = line_number + 1;

        'line_loop: while let Some(c) = chars.next() {
            match c {
                '(' => tokens.push(Token::new(
                    TokenType::LeftParen,
                    c.to_string(),
                    None,
                    line_number,
                )),
                ')' => tokens.push(Token::new(
                    TokenType::RightParen,
                    c.to_string(),
                    None,
                    line_number,
                )),
                '{' => tokens.push(Token::new(
                    TokenType::LeftBrace,
                    c.to_string(),
                    None,
                    line_number,
                )),
                '}' => tokens.push(Token::new(
                    TokenType::RightBrace,
                    c.to_string(),
                    None,
                    line_number,
                )),
                ',' => tokens.push(Token::new(
                    TokenType::Comma,
                    c.to_string(),
                    None,
                    line_number,
                )),
                '.' => tokens.push(Token::new(TokenType::Dot, c.to_string(), None, line_number)),
                '-' => tokens.push(Token::new(
                    TokenType::Minus,
                    c.to_string(),
                    None,
                    line_number,
                )),
                '+' => tokens.push(Token::new(
                    TokenType::Plus,
                    c.to_string(),
                    None,
                    line_number,
                )),
                ';' => tokens.push(Token::new(
                    TokenType::Semicolon,
                    c.to_string(),
                    None,
                    line_number,
                )),
                '*' => tokens.push(Token::new(
                    TokenType::Star,
                    c.to_string(),
                    None,
                    line_number,
                )),
                '/' => match chars.peek() {
                    Some('/') => {
                        break;
                    }
                    _ => tokens.push(Token::new(
                        TokenType::Slash,
                        c.to_string(),
                        None,
                        line_number,
                    )),
                },
                '!' => match chars.peek() {
                    Some('=') => {
                        let next = chars.next().unwrap();
                        let formatted = format!("{}{}", c, next);
                        tokens.push(Token::new(
                            TokenType::BangEqual,
                            formatted.to_string(),
                            None,
                            line_number,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(
                            TokenType::Bang,
                            c.to_string(),
                            None,
                            line_number,
                        ));
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
                            line_number,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(
                            TokenType::Equal,
                            c.to_string(),
                            None,
                            line_number,
                        ));
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
                            line_number,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(
                            TokenType::Greater,
                            c.to_string(),
                            None,
                            line_number,
                        ));
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
                            line_number,
                        ));
                    }
                    _ => {
                        tokens.push(Token::new(
                            TokenType::Less,
                            c.to_string(),
                            None,
                            line_number,
                        ));
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
                                eprintln!("[line {}] Error: Unterminated string.", line_number);
                                exit_code = 65;
                                break 'line_loop;
                            }
                        }
                    }
                    tokens.push(Token::new(
                        TokenType::String,
                        format!("\"{}\"", str_value),
                        str_value.into(),
                        line_number,
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

                    tokens.push(Token::parse_number(&num_value, line_number));
                    if num_value.ends_with(".") {
                        tokens.push(Token::new(
                            TokenType::Dot,
                            '.'.to_string(),
                            None,
                            line_number,
                        ));
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
                    tokens.push(Token::new(token_type, identifier, None, line_number));
                }
                _ => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, c);
                    exit_code = 65;
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, "".to_string(), None, 0));
    (tokens, exit_code)
}
