use crate::models::expressions::Expr;
use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Print(e) => write!(f, "Print {}", e),
            Self::Expression(e) => write!(f, "Expression {}", e),
        }
    }
}
