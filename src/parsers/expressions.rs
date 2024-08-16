use crate::tokens::token::Token;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(Value::Nil) => write!(f, "nil"),
            Self::Literal(Value::Bool(b)) => write!(f, "{b}"),
            Self::Literal(Value::String(s)) => write!(f, "{s}"),
            Self::Literal(Value::Number(n)) => write!(f, "{:?}", n),
            Self::Grouping(e) => write!(f, "(group {e})"),
            Self::Unary(t, e) => write!(f, "({} {e})", t.name),
            Self::Binary(l, o, r) => write!(f, "({} {l} {r})", o.name),
        }
    }
}
