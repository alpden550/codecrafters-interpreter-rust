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
            Expr::Binary(left, token, right) => self.visit_binary_expr(*left, token, *right),
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
                Value::Number(n) => Ok(Value::Bool(n == 0.0)),
                Value::String(n) => Ok(Value::Bool(n.is_empty())),
            },
            _ => Ok(Value::Nil),
        }
    }

    fn visit_binary_expr(&self, left: Expr, token: Token, right: Expr) -> Result<Value, String> {
        let left_expr = self.evaluate(left)?;
        let right_expr = self.evaluate(right)?;

        match token.token_type {
            TokenType::Minus => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
                _ => Err(format!(
                    "[line {}] invalid values, it must be numbers",
                    token.line_number
                )),
            },
            TokenType::Slash => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x / y)),
                _ => Err(format!(
                    "[line {}] invalid values, it must be numbers",
                    token.line_number
                )),
            },
            TokenType::Star => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
                _ => Err(format!(
                    "[line {}] invalid values, it must be numbers",
                    token.line_number
                )),
            },
            _ => Ok(Value::Nil),
        }
    }
}
