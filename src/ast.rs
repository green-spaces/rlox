//! That abstract syntax tree data structures for lox [source](http://craftinginterpreters.com/representing-code.html#a-grammar-for-lox-expressions)

// TODO are there other structures that could be used to define the expressions
/// The grammar the defines all possible expressions in lox
pub enum Expr {
    Literal(Literals),
    Unary(UnaryOperators, Box<Expr>),
    Binary(Box<Expr>, BinaryOperators, Box<Expr>),
    Grouping(Box<Expr>),
}

/// Literals
pub enum Literals {
    String(String),
    Number(f64),
    True,
    False,
    Nil
}

/// Unary Operators
pub enum UnaryOperators {
    Minus,
    Bang,
}

/// Binary Operators
pub enum BinaryOperators {
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
}
