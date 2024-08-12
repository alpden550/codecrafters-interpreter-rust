use std::fmt::{Display, Formatter};

pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,
    Eof,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    String,
}

impl TokenType {
    fn display_name(&self) -> &str {
        match &self {
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::Comma => "COMMA",
            TokenType::Dot => "DOT",
            TokenType::Minus => "MINUS",
            TokenType::Plus => "PLUS",
            TokenType::Semicolon => "SEMICOLON",
            TokenType::Star => "STAR",
            TokenType::Slash => "SLASH",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::String => "STRING",
            TokenType::Eof => "EOF",
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
