use crate::parsers::expressions::{Expr, Literal};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Literal, String> {
        match expr {
            Expr::Literal(l) => Ok(l),
            Expr::Grouping(expr) => self.evaluate(*expr),
            _ => Ok(Literal::Nil),
        }
    }
}
