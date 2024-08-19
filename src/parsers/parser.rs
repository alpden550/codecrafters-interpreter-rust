use crate::errors::ExitCode;
use crate::parsers::expressions::{Expr, Stmt, Value};
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;
use std::process::exit;

#[derive(Default, Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    pub exprs: Vec<Expr>,
    pub stmts: Vec<Stmt>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            exprs: Vec::new(),
            stmts: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            match self.expression() {
                Ok(e) => self.exprs.push(e),
                Err(e) => {
                    eprintln!("{e}");
                    exit(ExitCode::ExitError as i32);
                }
            };
        }
    }

    pub fn parse_stmts(&mut self) {
        while !self.is_at_end() {
            match self.declaration() {
                Ok(s) => self.stmts.push(s),
                Err(e) => {
                    eprintln!("{e}");
                    exit(ExitCode::ExitError as i32);
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, error: String) -> Result<&Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            return Ok(self.advance());
        }

        Err(format!("[line {}] {}", self.previous().line_number, error))
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        let token_type = &self.peek().token_type;
        if self.is_at_end() || !types.contains(token_type) {
            return false;
        }

        self.advance();
        true
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.equality()?;
        if self.match_tokens(&[TokenType::Equal]) {
            let expr_value = self.assignment()?;
            return match expr {
                Expr::Var(t) => Ok(Expr::Assign(t, Box::new(expr_value))),
                _ => Err("Invalid assignment target.".to_string()),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut left = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right))
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.term()?;

        let comparison_tokens = &[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.match_tokens(comparison_tokens) {
            let operator = self.previous().clone();
            let right = self.term()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right))
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut left = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right))
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut left = self.unary()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            left = Expr::Binary(Box::new(left), operator, Box::new(right));
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary(operator, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.previous().token_type == TokenType::Identifier {
            return Ok(Expr::Var(self.previous().clone()));
        }

        let token = self.advance();
        match token.token_type {
            TokenType::Nil => Ok(Expr::Literal(Value::Nil)),
            TokenType::True => Ok(Expr::Literal(Value::Bool(true))),
            TokenType::False => Ok(Expr::Literal(Value::Bool(false))),
            TokenType::String => {
                let string = token.value.clone().unwrap();
                Ok(Expr::Literal(Value::String(string)))
            }
            TokenType::Number => {
                let number = token.value.clone().unwrap().parse().unwrap();
                Ok(Expr::Literal(Value::Number(number)))
            }
            TokenType::LeftParen => {
                let expr = self.expression()?;
                if self.match_tokens(&[TokenType::RightParen]) {
                    Ok(Expr::Grouping(Box::new(expr)))
                } else {
                    Err(format!(
                        "[line {}] Expect ')' after expression.",
                        self.previous().line_number
                    ))
                }
            }
            _ => Err(format!(
                "[line {}] Error at ')': Expect expression.",
                self.previous().line_number
            )),
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        let token = self.advance();
        if token.token_type == TokenType::Var {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token = self
            .consume(TokenType::Identifier, "Expect variable name.".to_string())?
            .clone();

        if !(self.peek().token_type == TokenType::Equal) {
            return Err(format!(
                "[line {}]: expect assigning.",
                self.previous().line_number
            ));
        }
        let initializer = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.".to_string(),
        )?;

        Ok(Stmt::Variable(token, initializer))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        let token = self.previous();
        match token.token_type {
            TokenType::Print => self.print_statement(),
            _ => {
                let value = self.expression_statement()?;
                Ok(value)
            }
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string())?;

        match expr {
            Expr::Literal(Value::String(_)) => Ok(Stmt::Print(expr)),
            _ => Err(format!(
                "[line {}] print statement supports only strings",
                self.previous().line_number
            )),
        }
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after expression.".to_string(),
        )?;

        Ok(Stmt::Expression(expr))
    }
}
