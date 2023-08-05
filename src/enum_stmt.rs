use crate::{ast_enum::ExprNode, RunTimeError};

#[derive(Debug)]
pub enum StmtNode {
    Print(ExprNode),
    Expr(ExprNode),
}

pub trait StmtAcceptorMut<V: StmtVisitorMut> {
    fn accept(&self, visitor: &mut V) -> Result<(), RunTimeError>;
}

pub trait StmtVisitorMut {
    fn visit_print(&mut self, expr_node: &ExprNode) -> Result<(), RunTimeError>;

    fn visit_expr(&mut self, expr_node: &ExprNode) -> Result<(), RunTimeError>;
}

impl<V> StmtAcceptorMut<V> for StmtNode
where
    V: StmtVisitorMut,
{
    fn accept(&self, visitor: &mut V) -> Result<(), RunTimeError> {
        match self {
            Self::Print(expr) => visitor.visit_print(expr),
            Self::Expr(expr) => visitor.visit_expr(expr),
        }
    }
}
