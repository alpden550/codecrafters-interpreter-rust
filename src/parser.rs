use crate::models::expressions::Expr;
use crate::models::statements::Stmt;
use crate::models::token_types::TokenType;
use crate::models::tokens::Token;
use crate::models::values::Value;

pub struct Parser<'a> {
    pub tokens: &'a [Token],
    pub stmts: Vec<Stmt>,
    pub errors: Vec<String>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            stmts: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            match self.declaration() {
                Ok(e) => self.stmts.push(e),
                Err(e) => self.errors.push(e),
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

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        &self.peek().token_type == token_type
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, String> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        Err(format!("[line {}] {}", self.previous().line_number, msg))
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        let mut res = self.statement();
        if self.matches(&[TokenType::Var]) {
            res = self.var_declaration();
        }

        match res {
            Ok(s) => Ok(s),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::Identifier, "Expect variable name.")?;
        let token = self.previous().clone();
        let mut initializer = None;

        if self.matches(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(token.clone(), initializer))
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.matches(&[TokenType::LeftBrace]) {
            let stmts = self.block()?;
            return Ok(Stmt::Block(Box::new(stmts)));
        }

        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'if' condition.")?;

        if self.peek().token_type != TokenType::LeftBrace {
            return Err(format!(
                "[line {}] Expect {{ before if body",
                self.peek().line_number
            ));
        }
        let then_branch = self.statement()?;

        let mut else_branch = None;
        if self.matches(&[TokenType::Else]) {
            if self.peek().token_type != TokenType::LeftBrace {
                return Err(format!(
                    "[line {}] Expect {{ before else body",
                    self.peek().line_number
                ));
            }
            let e = self.statement()?;
            else_branch = Some(Box::new(e));
        }

        Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Print(expr))
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        if self.peek().token_type != TokenType::LeftBrace {
            return Err(format!(
                "[line {}] Expect {{ before while body",
                self.peek().line_number
            ));
        }

        let body = self.statement()?;
        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        Ok(Stmt::Expression(expr))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.logic_or()?;

        if self.matches(&[TokenType::Equal]) {
            let token_equal = self.previous().clone();
            let assignment = self.assignment()?;
            return match expr {
                Expr::Variable(t) => Ok(Expr::Assign(t, Box::new(assignment))),
                _ => Err(format!(
                    "[line {}] Invalid assignment target.",
                    token_equal.line_number
                )),
            };
        }

        Ok(expr)
    }

    fn logic_or(&mut self) -> Result<Expr, String> {
        let left = self.logic_and()?;

        while self.matches(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.logic_and()?;
            return Ok(Expr::Logical(Box::new(left), operator, Box::new(right)));
        }

        Ok(left)
    }

    fn logic_and(&mut self) -> Result<Expr, String> {
        let left = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            return Ok(Expr::Logical(Box::new(left), operator, Box::new(right)));
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;

        let comparison_tokens = &[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.matches(comparison_tokens) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.matches(&[
            TokenType::Nil,
            TokenType::True,
            TokenType::False,
            TokenType::String,
            TokenType::Number,
        ]) {
            let token = self.previous().clone();
            return match token.token_type {
                TokenType::Nil => Ok(Expr::Literal(Value::Nil)),
                TokenType::True => Ok(Expr::Literal(Value::Bool(true))),
                TokenType::False => Ok(Expr::Literal(Value::Bool(false))),
                TokenType::String => Ok(Expr::Literal(Value::String(
                    token.value.get_string().unwrap(),
                ))),
                TokenType::Number => Ok(Expr::Literal(Value::Number(
                    token.value.get_number().unwrap(),
                ))),
                _ => Err("Invalid value for literal type".to_string()),
            };
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        if self.matches(&[TokenType::RightParen]) {
            return Err(format!(
                "[line {}] found not expected ')'",
                self.previous().line_number
            ));
        }

        if self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        Err(format!(
            "[line {}] Expect expression.",
            self.peek().line_number
        ))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            let switched = [
                TokenType::Class,
                TokenType::Fun,
                TokenType::Var,
                TokenType::For,
                TokenType::If,
                TokenType::While,
                TokenType::Print,
                TokenType::Return,
            ];
            if switched.contains(&self.peek().token_type) {
                return;
            }

            self.advance();
        }
    }
}
