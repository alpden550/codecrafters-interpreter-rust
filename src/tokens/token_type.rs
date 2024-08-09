use std::fmt::{Display, Formatter};

pub enum TokenType {
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,
    Eof,
}

impl TokenType {
    fn display_name(&self) -> &str {
        match &self {
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::Eof => "EOF",
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
