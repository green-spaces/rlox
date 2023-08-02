use crate::ast::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};

pub trait AstVisitor
where
    Self: Sized,
{
    type Output;

    fn visit_binary<L: AstAcceptor<Self>, R: AstAcceptor<Self>>(
        &mut self,
        binary: &BinaryExpr<L, R>,
    ) -> Self::Output;

    fn visit_unary<R: AstAcceptor<Self>>(&mut self, unary: &UnaryExpr<R>) -> Self::Output;
    fn visit_grouping<E: AstAcceptor<Self>>(&mut self, grouping: &GroupingExpr<E>) -> Self::Output;
    fn visit_literal(&mut self, literal: &LiteralExpr) -> Self::Output;
}

pub trait AstAcceptor<V: AstVisitor> {
    fn accept(&self, visitor: &mut V) -> <V as AstVisitor>::Output;
}

pub trait AstAcceptor2 {
    fn accept<O>(&self, visitor: Box<dyn AstVisitor>) -> String;
}

///  A pretty printer for expressions
pub struct PrettyPrinter {}

impl AstVisitor for PrettyPrinter {
    type Output = String;

    fn visit_binary<L: AstAcceptor<Self>, R: AstAcceptor<Self>>(
        &mut self,
        binary: &BinaryExpr<L, R>,
    ) -> Self::Output {
        let left_str = binary.left.accept(self);
        let right_str = binary.right.accept(self);
        format!("( {:?} {left_str} {right_str} )", binary.operator.lexeme)
    }

    fn visit_unary<R: AstAcceptor<Self>>(&mut self, unary: &UnaryExpr<R>) -> Self::Output {
        let right_str = unary.right.accept(self);
        format!("( {:?} {right_str} )", unary.operator.lexeme)
    }

    fn visit_literal(&mut self, literal: &LiteralExpr) -> Self::Output {
        format!("{:?}", literal.value)
    }

    fn visit_grouping<E: AstAcceptor<Self>>(&mut self, grouping: &GroupingExpr<E>) -> Self::Output {
        let expr_str = grouping.accept(self);
        format!("group {expr_str} ")
    }
}

///  A pretty printer for expressions
pub struct NotPrettyPrinter {}

impl AstVisitor for NotPrettyPrinter {
    type Output = String;

    fn visit_binary<L: AstAcceptor<Self>, R: AstAcceptor<Self>>(
        &mut self,
        binary: &BinaryExpr<L, R>,
    ) -> Self::Output {
        let left_str = binary.left.accept(self);
        let right_str = binary.right.accept(self);
        let out = format!("{left_str} + {right_str}");
        out
    }

    fn visit_unary<R: AstAcceptor<Self>>(&mut self, unary: &UnaryExpr<R>) -> Self::Output {
        todo!()
    }

    fn visit_literal(&mut self, literal: &LiteralExpr) -> Self::Output {
        todo!()
    }

    fn visit_grouping<E: AstAcceptor<Self>>(&mut self, grouping: &GroupingExpr<E>) -> Self::Output {
        todo!()
    }
}
