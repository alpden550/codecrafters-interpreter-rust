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
        let res;

        if self.matches(&[TokenType::Fun]) {
            res = self.function("function");
        } else if self.matches(&[TokenType::Var]) {
            res = self.var_declaration();
        } else {
            res = self.statement();
        }

        match res {
            Ok(s) => Ok(s),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, String> {
        self.consume(
            TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
        )?;
        let name = self.previous().clone();
        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
        )?;

        let mut parameters = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                self.consume(TokenType::Identifier, "Expect parameter name.")?;
                let param = self.previous().clone();
                parameters.push(param);
                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;

        let body = self.block()?;
        let func = Stmt::Function(name.clone(), parameters, Box::new(body));
        Ok(func)
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
        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }

        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.matches(&[TokenType::Return]) {
            return self.return_statement();
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

    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.matches(&[TokenType::Semicolon]) {
            None
        } else if self.matches(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.check(&TokenType::Semicolon) {
            Expr::Literal(Value::Bool(true))
        } else {
            self.expression()?
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if self.check(&TokenType::RightParen) {
            None
        } else {
            Some(self.expression()?)
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(i) = increment {
            body = Stmt::Block(Box::new(vec![body, Stmt::Expression(i)]));
        }

        body = Stmt::While(condition, Box::new(body));

        if let Some(i) = initializer {
            body = Stmt::Block(Box::new(vec![i, body]));
        }

        Ok(body)
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

    fn return_statement(&mut self) -> Result<Stmt, String> {
        let token = self.previous().clone();
        let value;
        if !self.check(&TokenType::Semicolon) {
            value = Some(self.expression()?);
        } else {
            value = None;
        }

        self.consume(
            TokenType::Semicolon,
            format!(
                "[line {}] Expect ';' after return value.",
                token.line_number
            )
            .as_str(),
        )?;

        let return_stmt = Stmt::Return(token, value);
        Ok(return_stmt)
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
        let while_stmt = Stmt::While(condition, Box::new(body));
        Ok(while_stmt)
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

        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = vec![];

        if !self.check(&TokenType::RightParen) {
            arguments.push(self.expression()?);
            while self.matches(&[TokenType::Comma]) {
                if arguments.len() >= 255 {
                    return Err(format!(
                        "[line {}] Can't have more than 255 arguments.",
                        self.peek().line_number
                    ));
                }

                arguments.push(self.expression()?)
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        let func = Expr::Call(Box::new(callee), paren.clone(), Box::new(arguments));
        Ok(func)
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
