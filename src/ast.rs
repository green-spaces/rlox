//! That abstract syntax tree data structures for lox [source](http://craftinginterpreters.com/representing-code.html#a-grammar-for-lox-expressions)

use std::boxed::Box;

use super::ast_visitor::AstAcceptor;
use crate::ast_visitor::AstVisitor;

pub struct BinaryExpr<L, R> {
    pub left: L,
    pub operator: BinaryOperators,
    pub right: R,
}

impl<V, L, R> AstAcceptor<V> for BinaryExpr<L, R>
where
    V: AstVisitor,
    L: AstAcceptor<V>,
    R: AstAcceptor<V>,
{
    fn accept(&self, visitor: &mut V) -> <V as AstVisitor>::Output {
        visitor.visit_binary(&self)
    }
}

pub struct GroupingExpr<E> {
    pub expr: E,
}

impl<E, V> AstAcceptor<V> for GroupingExpr<E>
where
    V: AstVisitor,
    E: AstAcceptor<V>,
{
    fn accept(&self, visitor: &mut V) -> <V as AstVisitor>::Output {
        visitor.visit_grouping(self)
    }
}

pub struct UnaryExpr<R> {
    pub operator: UnaryOperators,
    pub right: R,
}

impl<V, R> AstAcceptor<V> for UnaryExpr<R>
where
    V: AstVisitor,
    R: AstAcceptor<V>,
{
    fn accept(&self, visitor: &mut V) -> <V as AstVisitor>::Output {
        visitor.visit_unary(self)
    }
}

pub struct LiteralExpr {
    pub value: Literals,
}

impl<V> AstAcceptor<V> for LiteralExpr
where
    V: AstVisitor,
{
    fn accept(&self, visitor: &mut V) -> <V as AstVisitor>::Output {
        visitor.visit_literal(self)
    }
}

// TODO are there other structures that could be used to define the expressions
/// The grammar the defines all possible expressions in lox
pub enum Expr {
    Literal(Literals),
    Unary(UnaryOperators, Box<Expr>),
    Binary(Box<Expr>, BinaryOperators, Box<Expr>),
    Grouping(Box<Expr>),
}

/// Literals
#[derive(Debug)]
pub enum Literals {
    String(String),
    Number(f64),
    True,
    False,
    Nil,
}

/// Unary Operators
#[derive(Debug)]
pub enum UnaryOperators {
    Minus,
    Bang,
}

/// Binary Operators
#[derive(Debug)]
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
