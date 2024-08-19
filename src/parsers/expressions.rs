use crate::tokens::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
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
    Var(Token),
    Assign(Token, Box<Expr>),
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
            Self::Var(s) => write!(f, "{}", s),
            Self::Binary(l, o, r) => write!(f, "({} {l} {r})", o.name),
            Self::Assign(t, e) => write!(f, "({} {e})", t.name),
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Variable(Token, Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Print(e) => write!(f, "PRINT {:?}", e),
            Self::Expression(s) => write!(f, "{s}"),
            Self::Variable(t, e) => write!(f, "{} {}", t.name, e),
        }
    }
}
