use crate::models::tokens::Token;
use crate::models::values::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Assign(Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{v}"),
            Self::Grouping(e) => write!(f, "(group {e})"),
            Self::Unary(t, e) => write!(f, "({} {e})", t.name),
            Self::Variable(t) => write!(f, "(variable {t})"),
            Self::Assign(t, e) => write!(f, "({} {e})", t.name),
            Self::Binary(l, o, r) => write!(f, "({} {l} {r})", o.name),
        }
    }
}
