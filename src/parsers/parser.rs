use crate::errors::ErrorT;
use crate::parsers::expressions::{Expr, Value};
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;
use std::process::exit;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    pub exprs: Vec<Expr>,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            exprs: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while !self.is_at_end() {
            match self.expression() {
                Ok(e) => self.exprs.push(e),
                Err(e) => {
                    eprintln!("{e}");
                    exit(65);
                }
            };
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

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        let token_type = &self.peek().token_type;
        if self.is_at_end() || !types.contains(token_type) {
            return false;
        }

        self.advance();
        true
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
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
                    Err(ErrorT::new(self.previous().line_number, ')'.into()).error_brace())
                }
            }
            _ => Err(ErrorT::new(self.previous().line_number, ')'.into()).error_expr()),
        }
    }
}
