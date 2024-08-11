use crate::tokens::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub struct Token {
    token_type: TokenType,
    name: String,
    value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, name: String, value: Option<String>) -> Self {
        Token {
            token_type,
            name,
            value,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.name,
            self.value.clone().unwrap_or(String::from("null"))
        )
    }
}
