use crate::token::Token;

pub trait AstNodeVisitor {
    type Output;

    fn visit_literal(&self, value: &LiteralNode) -> Self::Output;
    fn visit_unary(&self, value: &UnaryNode) -> Self::Output;
    fn visit_binary(&self, value: &BinaryNode) -> Self::Output;
    fn visit_grouping(&self, value: &GroupingNode) -> Self::Output;
}

pub trait AstNodeAccept<V: AstNodeVisitor> {
    fn accept(&self, visitor: V) -> <V as AstNodeVisitor>::Output;
}

impl<V: AstNodeVisitor> AstNodeAccept<V> for ExprNode {
    fn accept(&self, visitor: V) -> <V as AstNodeVisitor>::Output {
        match self {
            Self::Literal(l) => visitor.visit_literal(&l),
            Self::Unary(u) => visitor.visit_unary(&u),
            Self::Binary(b) => visitor.visit_binary(&b),
            Self::Grouping(g) => visitor.visit_grouping(&g),
        }
    }
}

pub enum ExprNode {
    Literal(LiteralNode),
    Unary(UnaryNode),
    Binary(BinaryNode),
    Grouping(GroupingNode),
}

impl ExprNode {
    pub fn new_unary(operator: Token, right: ExprNode) -> Self {
        Self::Unary(UnaryNode {
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_binary(left: ExprNode, operator: Token, right: ExprNode) -> Self {
        Self::Binary(BinaryNode {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    pub fn new_grouping(inner: ExprNode) -> Self {
        Self::Grouping(GroupingNode {
            inner: Box::new(inner),
        })
    }
}

pub struct UnaryNode {
    pub operator: Token,
    pub right: Box<ExprNode>,
}

pub struct BinaryNode {
    pub left: Box<ExprNode>,
    pub operator: Token,
    pub right: Box<ExprNode>,
}

pub struct GroupingNode {
    pub inner: Box<ExprNode>,
}

/// Literals
#[derive(Debug)]
pub enum LiteralNode {
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

///  A pretty printer for expressions
#[derive(Copy, Clone)]
pub struct PrettyPrinter {}

impl AstNodeVisitor for PrettyPrinter {
    type Output = String;

    fn visit_binary(&self, binary: &BinaryNode) -> Self::Output {
        let left_str = binary.left.accept(*self);
        let right_str = binary.right.accept(*self);
        format!("( {:?} {left_str} {right_str} )", binary.operator.lexeme)
    }

    fn visit_unary(&self, unary: &UnaryNode) -> Self::Output {
        let right_str = unary.right.accept(*self);
        format!("( {:?} {right_str} )", unary.operator.lexeme)
    }

    fn visit_literal(&self, literal: &LiteralNode) -> Self::Output {
        format!("{:?}", literal)
    }

    fn visit_grouping(&self, grouping: &GroupingNode) -> Self::Output {
        let expr_str = grouping.inner.accept(*self);
        format!("group {expr_str} ")
    }
}
