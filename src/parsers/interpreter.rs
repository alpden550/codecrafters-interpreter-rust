use crate::parsers::expressions::{Expr, Value};
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn evaluate(&self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(v) => Ok(v),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Unary(token, expr) => self.visit_unary_expr(token, *expr),
            _ => Ok(Value::Nil),
        }
    }

    fn visit_unary_expr(&self, token: Token, expr: Expr) -> Result<Value, String> {
        let right = self.evaluate(expr)?;
        match token.token_type {
            TokenType::Minus => match right {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(format!("[line {}] invalid value for -", token.line_number)),
            },
            TokenType::Bang => match right {
                Value::Nil => Ok(Value::Bool(true)),
                Value::Bool(b) => Ok(Value::Bool(!b)),
                Value::Number(_) => Ok(Value::Bool(false)),
                _ => Err(format!("[line {}] invalid value for !", token.line_number)),
            },
            _ => Ok(Value::Nil),
        }
    }
}
