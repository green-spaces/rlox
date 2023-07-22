use crate::{
    token::{Literal, Token, TokenType},
    Lox,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<ScannerError>,
    line: u64,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens = Vec::new();
        let errors = Vec::new();
        let line = 1;
        let start = 0;
        let current = 0;

        Self {
            source,
            tokens,
            errors,
            line,
            start,
            current,
        }
    }

    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<ScannerError>) {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_token = Token::new(TokenType::Eof, "".to_string(), Literal::None, self.line);
        self.tokens.push(eof_token);

        (self.tokens.clone(), self.errors.clone())
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::None),
            ')' => self.add_token(TokenType::RightParen, Literal::None),
            '{' => self.add_token(TokenType::LeftBrace, Literal::None),
            '}' => self.add_token(TokenType::RightBrace, Literal::None),
            ',' => self.add_token(TokenType::Comma, Literal::None),
            '.' => self.add_token(TokenType::Dot, Literal::None),
            '-' => self.add_token(TokenType::Minus, Literal::None),
            '+' => self.add_token(TokenType::Plus, Literal::None),
            ';' => self.add_token(TokenType::Semicolon, Literal::None),
            '*' => self.add_token(TokenType::Star, Literal::None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Bang, Literal::None);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Equal, Literal::None);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Greater, Literal::None);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, Literal::None);
                } else {
                    self.add_token(TokenType::Less, Literal::None);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_eof() {
                        let _ = self.advance();
                    }
                    self.line += 1;
                } else {
                    self.add_token(TokenType::Slash, Literal::None);
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            _ => self
                .errors
                .push(ScannerError::UnrecognizedSymbol(self.line, c)),
        }
    }

    fn advance(&mut self) -> char {
        // TODO This assumes utf8 encoding and is suspect
        let c = self.source.as_bytes()[self.current].into();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            '\n'
        } else {
            self.source.as_bytes()[self.current].into()
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_eof() {
            return false;
        }

        let next_char: char = self.source.as_bytes()[self.current].into();
        if next_char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, t_type: TokenType, literal: Literal) {
        let lexeme = self.source.get(self.start..self.current).unwrap().into();
        let token = Token::new(t_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    /// An unreconized symbol was found
    UnrecognizedSymbol(u64, char),
}
