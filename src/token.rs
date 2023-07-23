#[derive(Debug, Clone)]
pub struct Token {
    t_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: u64,
}

impl Token {
    pub fn new(t_type: TokenType, lexeme: String, literal: Literal, line: u64) -> Self {
        Self {
            t_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:?} {} {:?}",
            self.t_type, self.lexeme, self.literal
        ))
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    None,
    String(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
