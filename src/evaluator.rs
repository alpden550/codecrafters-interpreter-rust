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
            Self::None => write!(f, ""),
        }
    }
}

impl Value {
    fn str_to_bool(s: &str) -> Result<bool, &str> {
        match s.to_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err("Invalid input"),
        }
    }
}

pub fn evaluate(token: &Token) -> Value {
    let name = token.name.as_str();
    let value = token.value.clone();

    match token.token_type {
        TokenType::Nil => Value::Nil,
        TokenType::False | TokenType::True => {
            let boolean = Value::str_to_bool(name).unwrap_or(false);
            Value::Bool(boolean)
        }
        TokenType::String => Value::String(value.unwrap_or("".to_string())),
        TokenType::Number => Value::Number(value.unwrap().parse().unwrap()),
        _ => Value::None,
    }
}
