use crate::models::tokens::Token;
use crate::models::values::Value;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{v}"),
            Self::Grouping(e) => write!(f, "(group {e})"),
            Self::Unary(t, e) => write!(f, "({} {e})", t.name),
            Self::Binary(l, o, r) => write!(f, "({} {l} {r})", o.name),
        }
    }
}

impl Expr {
    #[allow(dead_code)]
    pub fn accept(&self) {}
}
