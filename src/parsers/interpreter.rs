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
                _ => Err(format!(
                    "Operand must be a number.\n[line {}]",
                    token.line_number
                )),
            },
            TokenType::Bang => match right {
                Value::Nil => Ok(Value::Bool(true)),
                Value::Bool(b) => Ok(Value::Bool(!b)),
                Value::Number(n) => Ok(Value::Bool(n == 0.0)),
                Value::String(n) => Ok(Value::Bool(n.is_empty())),
            },
            _ => Err(format!("[line {}] invalid value for -", token.line_number)),
        }
    }

    fn visit_binary_expr(&self, left: Expr, token: Token, right: Expr) -> Result<Value, String> {
        let left_expr = &self.evaluate(left)?;
        let right_expr = &self.evaluate(right)?;

        match token.token_type {
            TokenType::Slash => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x / y)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number
                )),
            },
            TokenType::Star => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x * y)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number
                )),
            },
            TokenType::Minus => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x - y)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number
                )),
            },
            TokenType::Plus => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Number(x + y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::String(
                    String::with_capacity(s1.len() + s2.len()) + &s1 + &s2,
                )),
                _ => Err(format!(
                    "Operands must be two numbers or two strings.\n[line {}]",
                    token.line_number,
                )),
            },
            TokenType::Greater => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x > y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::Bool(s1 > s2)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number,
                )),
            },
            TokenType::GreaterEqual => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x >= y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::Bool(s1 >= s2)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number,
                )),
            },
            TokenType::Less => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x < y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::Bool(s1 < s2)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number,
                )),
            },
            TokenType::LessEqual => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x <= y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::Bool(s1 <= s2)),
                _ => Err(format!(
                    "Operands must be numbers.\n[line {}]",
                    token.line_number,
                )),
            },
            TokenType::EqualEqual => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x == y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::Bool(s1.clone() == s2.clone())),
                (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 == b2)),
                (Value::Nil, Value::Nil) => Ok(Value::Bool(true)),
                _ => Ok(Value::Bool(false)),
            },
            TokenType::BangEqual => match (left_expr, right_expr) {
                (Value::Number(x), Value::Number(y)) => Ok(Value::Bool(x != y)),
                (Value::String(s1), Value::String(s2)) => Ok(Value::Bool(s1.clone() != s2.clone())),
                (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 != b2)),
                (Value::Nil, Value::Nil) => Ok(Value::Bool(false)),
                _ => Ok(Value::Bool(true)),
            },
            _ => Ok(Value::Nil),
        }
    }
}
