use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Eof,
}

impl TokenType {
    fn display_name(&self) -> &str {
        match &self {
            TokenType::RightParen => "LEFT_PAREN",
            TokenType::LeftParen => "RIGHT_PAREN",
            TokenType::Eof => "EOF",
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
