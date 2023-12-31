use std::collections::HashMap;

use crate::token::{Token, TokenLiteral, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    errors: Vec<ScannerError>,
    line: u64,
    start: usize,
    current: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens = Vec::new();
        let errors = Vec::new();
        let line = 1;
        let start = 0;
        let current = 0;
        let keywords = keyword_map();

        Self {
            source,
            tokens,
            errors,
            line,
            start,
            current,
            keywords,
        }
    }

    /// Scans all the tokens, returns all tokens and errors found in the source
    pub fn scan_tokens(&mut self) -> (Vec<Token>, Vec<ScannerError>) {
        // Consume all the tokens in a single scan
        while !self.is_eof() {
            // Reset start so add_token knows where to start the token from
            self.start = self.current;
            self.scan_token();
        }

        let eof_token = Token::new(
            TokenType::Eof,
            "".to_string(),
            TokenLiteral::None,
            self.line,
        );
        self.tokens.push(eof_token);

        (self.tokens.clone(), self.errors.clone())
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, TokenLiteral::None),
            ')' => self.add_token(TokenType::RightParen, TokenLiteral::None),
            '{' => self.add_token(TokenType::LeftBrace, TokenLiteral::None),
            '}' => self.add_token(TokenType::RightBrace, TokenLiteral::None),
            ',' => self.add_token(TokenType::Comma, TokenLiteral::None),
            '.' => self.add_token(TokenType::Dot, TokenLiteral::None),
            '-' => self.add_token(TokenType::Minus, TokenLiteral::None),
            '+' => self.add_token(TokenType::Plus, TokenLiteral::None),
            ';' => self.add_token(TokenType::Semicolon, TokenLiteral::None),
            '*' => self.add_token(TokenType::Star, TokenLiteral::None),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual, TokenLiteral::None);
                } else {
                    self.add_token(TokenType::Bang, TokenLiteral::None);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual, TokenLiteral::None);
                } else {
                    self.add_token(TokenType::Equal, TokenLiteral::None);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual, TokenLiteral::None);
                } else {
                    self.add_token(TokenType::Greater, TokenLiteral::None);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual, TokenLiteral::None);
                } else {
                    self.add_token(TokenType::Less, TokenLiteral::None);
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_eof() {
                        let _ = self.advance();
                    }
                    self.line += 1;
                } else if self.match_char('*') {
                    loop {
                        if self.is_eof() {
                            // This is an error state
                            break;
                        }

                        if self.peek() == '\n' {
                            self.line += 1;
                        }

                        if self.peek() == '*' && self.peek_next() == '/' {
                            let _ = self.advance();
                            let _ = self.advance();
                            break;
                        }

                        let _ = self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, TokenLiteral::None);
                }
            }
            ' ' | '\t' | '\r' => {}
            '\n' => self.line += 1,
            '"' => self.tokenize_string_literal(),
            d if is_digit(d) => self.tokenize_number_literal(),
            a if is_alpha(a) => self.tokenize_identifier(),
            _ => self
                .errors
                .push(ScannerError::UnrecognizedSymbol(self.line, c)),
        }
    }

    /// Consumes the next char in the source
    fn advance(&mut self) -> char {
        // TODO This assumes utf8 encoding and is suspect
        let c = self.source.as_bytes()[self.current].into();
        self.current += 1;
        c
    }

    /// Gets the next character without consuming it
    fn peek(&self) -> char {
        if self.is_eof() {
            '\n'
        } else {
            self.source.as_bytes()[self.current].into()
        }
    }

    /// Gets the next next character without consuming it
    fn peek_next(&self) -> char {
        if self.is_eof() {
            '\n'
        } else {
            match self.source.as_bytes().get(self.current + 1) {
                Some(&c) => c.into(),
                None => '\0',
            }
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

    /// Tokenizes string literals
    ///
    /// String literals can span multiple lines
    fn tokenize_string_literal(&mut self) {
        while self.peek() != '"' && !self.is_eof() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            let _ = self.advance();
        }

        if self.is_eof() {
            self.errors
                .push(ScannerError::UnterminatedString(self.line));
            return;
        }

        // Consume the '"'
        let _ = self.advance();

        // Get the stirng without the quotes
        let value = self
            .source
            .get((self.start + 1)..(self.current - 1))
            .unwrap()
            .to_string();
        self.add_token(TokenType::String, TokenLiteral::String(value));
    }

    /// Tokenizes number literals.
    ///
    /// Forms: 42 or 42.24
    fn tokenize_number_literal(&mut self) {
        while is_digit(self.peek()) {
            let _ = self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            let _ = self.advance();
            while is_digit(self.peek()) {
                let _ = self.advance();
            }
        }

        let value = self.source.get(self.start..self.current).unwrap();
        self.add_token(
            TokenType::Number,
            TokenLiteral::Number(value.parse().unwrap()),
        );
    }

    /// Consumes all the chareacters that are attached to an identifier token
    ///
    /// Identifier tokens can be keywords or user defined identifiers
    ///
    /// Identifiers must start with a alpha character and contain alphanumeric chars
    fn tokenize_identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            let _ = self.advance();
        }
        let maybe_keyword = self.source.get(self.start..self.current).unwrap();
        let t_type = self
            .keywords
            .get(maybe_keyword)
            .cloned()
            .unwrap_or(TokenType::Identifier);
        self.add_token(t_type, TokenLiteral::None)
    }

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Creates the token and adds it to the list of known tokens
    ///
    /// Includes the line of the source code the token is on and the exact lexeme
    fn add_token(&mut self, t_type: TokenType, literal: TokenLiteral) {
        let lexeme = self.source.get(self.start..self.current).unwrap().into();
        let token = Token::new(t_type, lexeme, literal, self.line);
        self.tokens.push(token);
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

fn keyword_map() -> HashMap<String, TokenType> {
    [
        ("and".to_string(), TokenType::And),
        ("class".to_string(), TokenType::Class),
        ("else".to_string(), TokenType::Else),
        ("false".to_string(), TokenType::False),
        ("for".to_string(), TokenType::For),
        ("fun".to_string(), TokenType::Fun),
        ("if".to_string(), TokenType::If),
        ("nil".to_string(), TokenType::Nil),
        ("or".to_string(), TokenType::Or),
        ("print".to_string(), TokenType::Print),
        ("return".to_string(), TokenType::Return),
        ("super".to_string(), TokenType::Super),
        ("this".to_string(), TokenType::This),
        ("true".to_string(), TokenType::True),
        ("var".to_string(), TokenType::Var),
        ("while".to_string(), TokenType::While),
    ]
    .into_iter()
    .collect()
}

#[derive(Debug, Clone)]
pub enum ScannerError {
    /// An unreconized symbol was found
    UnrecognizedSymbol(u64, char),
    UnterminatedString(u64),
}
