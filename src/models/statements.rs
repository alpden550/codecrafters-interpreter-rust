use crate::models::expressions::Expr;
use crate::models::tokens::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Function(Token, Vec<Token>, Box<Vec<Stmt>>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Print(Expr),
    Var(Token, Option<Expr>),
    While(Expr, Box<Stmt>),
    Block(Box<Vec<Stmt>>),
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "Expression {e}"),
            Self::Function(name, params, body) => {
                write!(f, "func {name} {:?} with body {:?}", params, body)
            }
            Self::If(e, tb, eb) => write!(f, "If {} for than {:?} else {:?}", e, tb, eb),
            Self::Print(e) => write!(f, "Print {e}"),
            Self::Var(t, e) => write!(f, "Variable {t} for {:?}", e),
            Self::While(e, s) => write!(f, "While {} for {}", e, s),
            Self::Block(s) => write!(f, "Block for {:?}", s),
        }
    }
}
