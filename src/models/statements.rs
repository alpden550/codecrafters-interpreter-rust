use crate::models::expressions::Expr;
use crate::models::tokens::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Box<Vec<Stmt>>),
    Print(Expr),
    Expression(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Var(Token, Option<Expr>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Block(s) => write!(f, "Block for {:?}", s),
            Self::Print(e) => write!(f, "Print {e}"),
            Self::Expression(e) => write!(f, "Expression {e}"),
            Self::If(e, tb, eb) => write!(f, "If {} for than {:?} else {:?}", e, tb, eb),
            Self::Var(t, e) => write!(f, "Variable {t} for {:?}", e),
        }
    }
}
