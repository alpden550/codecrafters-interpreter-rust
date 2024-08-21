#![allow(dead_code)]

use crate::models::token_types::TokenType;
use crate::models::values::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub name: String,
    pub value: Value,
    pub line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, name: String, value: Value, line_number: usize) -> Self {
        Token {
            token_type,
            name,
            value,
            line_number,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.name, self.value)
    }
}
