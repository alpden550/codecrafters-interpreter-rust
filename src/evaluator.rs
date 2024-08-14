use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub enum Value {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::None => Ok(()),
        }
    }
}

pub fn evaluate(token: &Token) -> Value {
    let value = token.value.clone();

    match token.token_type {
        TokenType::Nil => Value::Nil,
        TokenType::False => Value::Bool(false),
        TokenType::True => Value::Bool(true),
        TokenType::String => Value::String(value.unwrap_or("".to_string())),
        TokenType::Number => Value::Number(value.unwrap().parse().unwrap()),
        _ => Value::None,
    }
}
