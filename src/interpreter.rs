use crate::environments::Environment;
use crate::models::expressions::Expr;
use crate::models::statements::Stmt;
use crate::models::token_types::TokenType;
use crate::models::tokens::Token;
use crate::models::values::Value;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        self.visit_stmt(stmt)
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(e) => {
                self.evaluate(e)?;
                Ok(())
            }
            Stmt::Print(e) => {
                let value = self.evaluate(e)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Var(t, e) => {
                let mut value = Value::Nil;
                match e {
                    Some(e) => {
                        value = self.evaluate(e)?;
                    }
                    None => {}
                }
                self.environment.insert(t.clone().name, value);
                Ok(())
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Grouping(e) => self.evaluate(e),
            Expr::Unary(t, e) => self.visit_unary_expr(t, e),
            Expr::Binary(l, t, r) => self.visit_binary_expr(l, t, r),
            Expr::Variable(t) => self.visit_variable_expr(t),
            Expr::Assign(t, e) => self.visit_assign_expr(t, e),
        }
    }

    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> Result<Value, String> {
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

    fn visit_variable_expr(&self, token: &Token) -> Result<Value, String> {
        let value = self.environment.get(token.name.as_str());
        match value {
            Some(v) => Ok(v.clone()),
            None => Err("Not founded".to_string()),
        }
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<Value, String> {
        let value = self.evaluate(expr)?;

        self.environment.insert(token.clone().name, value.clone());

        Ok(value)
    }

    fn visit_binary_expr(
        &mut self,
        left_expr: &Expr,
        token: &Token,
        right_expr: &Expr,
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
