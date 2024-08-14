use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub enum Value {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

pub fn evaluate(tokens: &[Token]) -> Vec<Value> {
    let mut values: Vec<Value> = vec![];

    for i in 0..tokens.len() {
        let value = tokens[i].value.clone();
        match &tokens[i].token_type {
            TokenType::Nil => values.push(Value::Nil),
            TokenType::False => values.push(Value::Bool(false)),
            TokenType::True => values.push(Value::Bool(true)),
            TokenType::String => values.push(Value::String(value.unwrap_or("".to_string()))),
            TokenType::Number => values.push(Value::Number(value.unwrap().parse().unwrap())),
            TokenType::LeftParen => {
                evaluate(&tokens[i + 1..]);
            }
            _ => {
                continue;
            }
        }
    }

    values
}
