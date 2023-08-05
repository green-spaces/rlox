use crate::token::{Token, TokenType};

#[derive(Debug, Clone)]
pub enum SyntaxError {
    /// Contains the token that didnt match any grammar rules
    UnmatchedToken(Token, String),
    /// Contains the expected token type, the token found, the a message
    ExpectedToken(TokenType, Token, String),
}

#[derive(Debug, Clone)]
pub enum RunTimeError {
    InvalidBangValue(Token, String),
}
