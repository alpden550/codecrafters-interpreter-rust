use crate::models::tokens::Token;
use crate::models::values::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
    Binary(Box<Expr>, Token, Box<Expr>),
    Assign(Token, Box<Expr>),
    Call(Box<Expr>, Token, Box<Vec<Expr>>),
    Grouping(Box<Expr>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Literal(v) => write!(f, "{v}"),
            Self::Logical(l, o, r) => write!(f, "({l} {o} {r})"),
            Self::Unary(t, e) => write!(f, "({} {e})", t.name),
            Self::Variable(t) => write!(f, "(variable {t})"),
            Self::Binary(l, o, r) => write!(f, "({} {l} {r})", o.name),
            Self::Assign(t, e) => write!(f, "({} {e})", t.name),
            Self::Call(callee, _paren, arguments) => {
                write!(f, "func {callee} with args {:?}", arguments)
            }
            Self::Grouping(e) => write!(f, "(group {e})"),
        }
    }
}
