use crate::models::expressions::Expr;
use crate::models::token_types::TokenType;
use crate::models::tokens::Token;
use crate::models::values::Value;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {}
    }

    pub fn interpret(&self, expr: Expr) -> Result<Value, String> {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(v) => Ok(v),
            Expr::Grouping(e) => self.evaluate(*e),
            Expr::Unary(t, e) => self.visit_unary_expr(t, *e),
            Expr::Binary(l, t, r) => self.visit_binary_expr(*l, t, *r),
        }
    }

    fn visit_unary_expr(&self, token: Token, expr: Expr) -> Result<Value, String> {
        let right = self.evaluate(expr)?;

        match token.token_type {
            TokenType::Minus => {
                if let Some(n) = right.get_number() {
                    Ok(Value::Number(-n))
                } else {
                    Err(format!(
                        "[line {}] Not a number for MINUS operation.",
                        token.line_number
                    ))
                }
            }
            TokenType::Bang => Ok(Value::Bool(!right.is_truthy())),
            _ => Err(format!(
                "[line {}] Invalid operation for unary expression.",
                token.line_number
            )),
        }
    }

    fn visit_binary_expr(
        &self,
        left_expr: Expr,
        token: Token,
        right_expr: Expr,
    ) -> Result<Value, String> {
        let left = self.evaluate(left_expr)?;
        let right = self.evaluate(right_expr)?;

        match token.token_type {
            TokenType::Minus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => Err(format!(
                    "[line {}] Not a number for minus operation",
                    token.line_number,
                )),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => Err(format!(
                    "[line {}] Not a number for division operation",
                    token.line_number,
                )),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => Err(format!(
                    "[line {}] Not a number for multiply operation",
                    token.line_number,
                )),
            },
            TokenType::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => Err(format!(
                    "[line {}] Not a number or string for plus operation",
                    token.line_number,
                )),
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l > r)),
                _ => Err(format!(
                    "[line {}] Not a number or string for greater operation",
                    token.line_number,
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l >= r)),
                _ => Err(format!(
                    "[line {}] Not a number or string for greater equal operation",
                    token.line_number,
                )),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l < r)),
                _ => Err(format!(
                    "[line {}] Not a number or string for less operation",
                    token.line_number,
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l <= r)),
                _ => Err(format!(
                    "[line {}] Not a number or string for less equal operation",
                    token.line_number,
                )),
            },
            TokenType::BangEqual => Ok(Value::Bool(!left.is_equal(right))),
            TokenType::EqualEqual => Ok(Value::Bool(left.is_equal(right))),
            _ => Err(format!(
                "[line {}] Invalid operation {} for binary expression.",
                token.line_number, token.token_type
            )),
        }
    }
}
