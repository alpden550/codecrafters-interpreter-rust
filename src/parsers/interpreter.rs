use crate::parsers::expressions::{Expr, Value};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(l) => Ok(l),
            Expr::Grouping(expr) => self.evaluate(*expr),
            _ => Ok(Value::Nil),
        }
    }
}
