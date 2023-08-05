use crate::ast_enum::{
    AstNodeAccept, AstNodeVisitor, BinaryNode, GroupingNode, LiteralNode, UnaryNode,
};

#[derive(Copy, Clone)]
pub struct Rpn {}

impl AstNodeVisitor for Rpn {
    type Output = String;
    fn visit_variable(&self, value: &crate::token::Token) -> Self::Output {
        todo!()
    }

    fn visit_unary(&self, value: &UnaryNode) -> Self::Output {
        let res = value.right.accept(*self);
        if res.is_empty() {
            return res;
        }
        format!("{res} {}", value.operator.lexeme)
    }

    fn visit_binary(&self, value: &BinaryNode) -> Self::Output {
        let left_str = value.left.accept(*self);
        let right_str = value.right.accept(*self);
        if left_str.is_empty() || right_str.is_empty() {
            return "".to_string();
        }
        format!("{left_str} {right_str} {}", value.operator.lexeme)
    }

    fn visit_literal(&self, value: &LiteralNode) -> Self::Output {
        match value {
            LiteralNode::Number(n) => n.to_string(),
            _ => "".to_string(),
        }
    }

    fn visit_grouping(&self, value: &GroupingNode) -> Self::Output {
        value.inner.accept(*self)
    }
}
