use crate::environments::Environment;
use crate::models::expressions::Expr;
use crate::models::statements::Stmt;
use crate::models::token_types::TokenType;
use crate::models::tokens::Token;
use crate::models::values::Value;

pub struct Interpreter<'a> {
    environment: Environment,
    pub stmts: &'a [Stmt],
    pub errors: Vec<String>,
}

impl<'a> Interpreter<'a> {
    pub fn new(stmts: &'a [Stmt]) -> Self {
        Interpreter {
            environment: Environment::new(None),
            stmts,
            errors: Vec::new(),
        }
    }

    pub fn interpret(&mut self) {
        for stmt in self.stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(e) => self.errors.push(e),
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
            Stmt::If(c, eb, tb) => self.visit_if_stmt(c, eb, tb),
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
                self.environment.define(t.clone().name, value);
                Ok(())
            }
            Stmt::While(e, s) => self.visit_while_stmt(e, s),
            Stmt::Block(s) => Ok(self.execute_block(s)),
        }
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Box<Stmt>,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), String> {
        if self.evaluate(condition)?.is_truthy() {
            return self.execute(then_branch);
        }

        match else_branch {
            None => {}
            Some(s) => {
                return self.execute(s);
            }
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), String> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }

        Ok(())
    }

    fn execute_block(&mut self, stmts: &[Stmt]) {
        let previous = self.environment.clone();
        self.environment = Environment::new(Some(Box::new(previous)));

        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(e) => self.errors.push(e),
            }
        }
        self.environment = *self.environment.enclosing.take().unwrap();
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Logical(l, t, r) => self.visit_logical_expr(l, t, r),
            Expr::Grouping(e) => self.evaluate(e),
            Expr::Unary(t, e) => self.visit_unary_expr(t, e),
            Expr::Binary(l, t, r) => self.visit_binary_expr(l, t, r),
            Expr::Variable(t) => self.visit_variable_expr(t),
            Expr::Assign(t, e) => self.visit_assign_expr(t, e),
        }
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        token: &Token,
        right: &Expr,
    ) -> Result<Value, String> {
        let left_value = self.evaluate(left)?;

        if token.token_type == TokenType::Or {
            if left_value.is_truthy() {
                return Ok(left_value);
            }
        } else {
            if !left_value.is_truthy() {
                return Ok(left_value);
            }
        }

        self.evaluate(right)
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
        self.environment.get(token)
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<Value, String> {
        let value = self.evaluate(expr)?;
        self.environment.assign(token, value.clone())
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
