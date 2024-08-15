use crate::parsers::expressions::{Expr, Literal};
use crate::tokens::token::Token;
use crate::tokens::token_type::TokenType;

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
                Err(_) => {}
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
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token = self.advance();
        match token.token_type {
            TokenType::Nil => Ok(Expr::Literal(Literal::Nil)),
            TokenType::True => Ok(Expr::Literal(Literal::Bool(true))),
            TokenType::False => Ok(Expr::Literal(Literal::Bool(false))),
            TokenType::String => {
                let string = token.value.clone().unwrap();
                Ok(Expr::Literal(Literal::String(string)))
            }
            TokenType::Number => {
                let number = token.value.clone().unwrap().parse().unwrap();
                Ok(Expr::Literal(Literal::Number(number)))
            }
            TokenType::LeftParen => {
                let expr = self.expression()?;
                if self.match_tokens(&[TokenType::RightParen]) {
                    println!("expr={expr}");
                    Ok(Expr::Grouping(Box::new(expr)))
                } else {
                    Err("Expect ')' after expression.".to_string())
                }
            }
            _ => Err("Unexpected token type".to_string()),
        }
    }
}
