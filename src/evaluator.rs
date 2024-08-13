use crate::tokens::token::Token;

pub fn evaluate(token: &Token) -> String {
    token.name.clone()
}
