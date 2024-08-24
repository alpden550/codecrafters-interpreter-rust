use crate::models::expressions::Expr;
use crate::models::tokens::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Var(Token, Option<Expr>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Print(e) => write!(f, "Print {e}"),
            Self::Expression(e) => write!(f, "Expression {e}"),
            Self::Var(t, e) => write!(f, "Variable {t} for {:?}", e),
        }
    }
}
