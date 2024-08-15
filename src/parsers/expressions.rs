use crate::tokens::token::Token;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Literal {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "nil"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(Literal::Nil) => write!(f, "nil"),
            Self::Literal(Literal::Bool(b)) => write!(f, "{b}"),
            Self::Literal(Literal::String(s)) => write!(f, "{s}"),
            Self::Literal(Literal::Number(n)) => write!(f, "{:?}", n),
            Self::Grouping(e) => write!(f, "(group {e})"),
            _ => Ok(()),
        }
    }
}
