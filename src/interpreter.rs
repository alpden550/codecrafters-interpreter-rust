use crate::environments::Environment;
use crate::errors::ValueError;
use crate::models::expressions::Expr;
use crate::models::lox_func::LoxFunction;
use crate::models::statements::Stmt;
use crate::models::token_types::TokenType;
use crate::models::tokens::Token;
use crate::models::values::Value;
use crate::native_funcs::clock_func::ClockFunction;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Interpreter<'a> {
    pub globals: Environment,
    pub environment: Environment,
    pub stmts: &'a [Stmt],
    pub errors: Vec<String>,
}

impl<'a> Interpreter<'a> {
    pub fn new(stmts: &'a [Stmt]) -> Self {
        let mut globals = Environment::new(None);
        globals.define(
            ClockFunction.to_string(),
            Value::Callable(Arc::new(ClockFunction)),
        );

        Interpreter {
            globals: globals.clone(),
            environment: globals,
            stmts,
            errors: Vec::new(),
        }
    }

    pub fn interpret(&mut self) {
        for stmt in self.stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(e) => match e {
                    ValueError::Error(m) => self.errors.push(m),
                    ValueError::Return(_) => {}
                },
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), ValueError> {
        self.visit_stmt(stmt)
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), ValueError> {
        match stmt {
            Stmt::Expression(e) => {
                self.evaluate(e)?;
                Ok(())
            }
            Stmt::Function(name, params, body) => self.visit_function_stmt(name, params, body),
            Stmt::If(c, eb, tb) => self.visit_if_stmt(c, eb, tb),
            Stmt::Print(e) => {
                let value = self.evaluate(e)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Return(_keyword, value) => self.visit_return_stmt(value),
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
            Stmt::Block(s) => {
                // let previous = self.environment.clone();
                // let env = Environment::new(Some(Box::new(previous)));
                self.execute_block(
                    s,
                    Environment::new(Some(Box::new(self.environment.clone()))),
                )
            }
        }
    }

    fn visit_function_stmt(
        &mut self,
        token: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Result<(), ValueError> {
        let func = Value::Callable(Arc::new(LoxFunction::new(
            token.clone(),
            Vec::from(params),
            Vec::from(body),
        )));
        self.environment.define(token.name.clone(), func); // use globals

        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Box<Stmt>,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<(), ValueError> {
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

    fn visit_return_stmt(&mut self, value: &Option<Expr>) -> Result<(), ValueError> {
        match value {
            None => Ok(()),
            Some(e) => {
                let ret = self.evaluate(e)?;
                Err(ValueError::Return(ret))
            }
        }
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), ValueError> {
        while self.evaluate(condition)?.is_truthy() {
            self.execute(body)?;
        }

        Ok(())
    }

    pub fn execute_block(&mut self, stmts: &[Stmt], env: Environment) -> Result<(), ValueError> {
        self.environment = env;

        for stmt in stmts {
            match self.execute(stmt) {
                Ok(_) => {}
                Err(error) => match error {
                    ValueError::Error(m) => self.errors.push(m),
                    ValueError::Return(v) => {
                        self.environment = *self.environment.enclosing.take().unwrap();
                        return Err(ValueError::Return(v));
                    }
                },
            }
        }
        self.environment = *self.environment.enclosing.take().unwrap();
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, ValueError> {
        match expr {
            Expr::Literal(v) => Ok(v.clone()),
            Expr::Logical(l, t, r) => self.visit_logical_expr(l, t, r),
            Expr::Grouping(e) => self.evaluate(e),
            Expr::Unary(t, e) => self.visit_unary_expr(t, e),
            Expr::Binary(l, t, r) => self.visit_binary_expr(l, t, r),
            Expr::Variable(t) => self.visit_variable_expr(t),
            Expr::Call(callee, paren, args) => self.visit_call_expr(callee, paren, args),
            Expr::Assign(t, e) => self.visit_assign_expr(t, e),
        }
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        token: &Token,
        right: &Expr,
    ) -> Result<Value, ValueError> {
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

    fn visit_unary_expr(&mut self, token: &Token, expr: &Expr) -> Result<Value, ValueError> {
        let right = self.evaluate(expr)?;

        match token.token_type {
            TokenType::Minus => {
                if let Some(n) = right.get_number() {
                    Ok(Value::Number(-n))
                } else {
                    let msg = format!(
                        "[line {}] Not a number for MINUS operation.",
                        token.line_number
                    );
                    Err(ValueError::Error(msg))
                }
            }
            TokenType::Bang => Ok(Value::Bool(!right.is_truthy())),
            _ => {
                let msg = format!(
                    "[line {}] Invalid operation for unary expression.",
                    token.line_number
                );
                Err(ValueError::Error(msg))
            }
        }
    }

    fn visit_variable_expr(&self, token: &Token) -> Result<Value, ValueError> {
        // self.environment.get(token) // search globals first
        match self.environment.get(token) {
            Ok(v) => Ok(v),
            Err(e) => Err(ValueError::Error(e)),
        }
    }

    fn visit_assign_expr(&mut self, token: &Token, expr: &Expr) -> Result<Value, ValueError> {
        let value = self.evaluate(expr)?;
        match self.environment.assign(token, value.clone()) {
            Ok(v) => Ok(v),
            Err(e) => Err(ValueError::Error(e)),
        }
    }

    fn visit_binary_expr(
        &mut self,
        left_expr: &Expr,
        token: &Token,
        right_expr: &Expr,
    ) -> Result<Value, ValueError> {
        let left = self.evaluate(left_expr)?;
        let right = self.evaluate(right_expr)?;

        match token.token_type {
            TokenType::Minus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number for minus operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number for division operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::Star => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number for multiply operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::Plus => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number or string for plus operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l > r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number or string for greater operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l >= r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number or string for greater equal operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::Less => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l < r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number or string for less operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                (Value::String(l), Value::String(r)) => Ok(Value::Bool(l <= r)),
                _ => {
                    let msg = format!(
                        "[line {}] Not a number or string for less equal operation",
                        token.line_number,
                    );
                    Err(ValueError::Error(msg))
                }
            },
            TokenType::BangEqual => Ok(Value::Bool(!left.is_equal(right))),
            TokenType::EqualEqual => Ok(Value::Bool(left.is_equal(right))),
            _ => {
                let msg = format!(
                    "[line {}] Invalid operation {} for binary expression.",
                    token.line_number, token.token_type
                );
                Err(ValueError::Error(msg))
            }
        }
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        args: &Vec<Expr>,
    ) -> Result<Value, ValueError> {
        let callee_func = self.evaluate(callee)?;
        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(self.evaluate(arg)?);
        }

        if let Some(func) = callee_func.is_callable() {
            if args.len() != func.arity() {
                let msg = format!(
                    "[line {}] Expected {} arguments, but got {}.",
                    paren.line_number,
                    func.arity(),
                    args.len()
                );
                return Err(ValueError::Error(msg));
            }

            func.call(self, &arguments)
        } else {
            let msg = format!(
                "[line {}] Can only call functions and classes.",
                paren.line_number
            );
            Err(ValueError::Error(msg))
        }
    }
}
