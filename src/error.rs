use crate::token::Token;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnmatchedToken(Token),
}

#[derive(Debug, Clone)]
pub enum RunTimeError {}
