use crate::{ast_enum::ExprNode, token::Token, RunTimeError};

#[derive(Debug)]
pub enum StmtNode {
    Print(ExprNode),
    Expr(ExprNode),
    Var(VarNode),
}

pub trait StmtAcceptorMut<V: StmtVisitorMut> {
    fn accept(&self, visitor: &mut V) -> Result<(), RunTimeError>;
}

pub trait StmtVisitorMut {
    fn visit_print(&mut self, expr_node: &ExprNode) -> Result<(), RunTimeError>;

    fn visit_expr(&mut self, expr_node: &ExprNode) -> Result<(), RunTimeError>;

    fn visit_var_dec(&mut self, var_node: &VarNode) -> Result<(), RunTimeError>;
}

impl<V> StmtAcceptorMut<V> for StmtNode
where
    V: StmtVisitorMut,
{
    fn accept(&self, visitor: &mut V) -> Result<(), RunTimeError> {
        match self {
            Self::Print(expr) => visitor.visit_print(expr),
            Self::Expr(expr) => visitor.visit_expr(expr),
            Self::Var(node) => visitor.visit_var_dec(node),
        }
    }
}

#[derive(Debug)]
pub struct VarNode {
    pub name: Token,
    pub initializer: ExprNode,
}

impl VarNode {
    pub fn new(name: Token, initializer: ExprNode) -> Self {
        Self { name, initializer }
    }
}
