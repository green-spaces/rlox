use crate::{
    ast_enum::ExprNode,
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ExprNode {
        self.expression()
    }

    fn expression(&mut self) -> ExprNode {
        self.equality()
    }

    fn equality(&mut self) -> ExprNode {
        let mut expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = ExprNode::new_binary(expr, operator, right);
        }
        expr
    }

    fn comparison(&mut self) -> ExprNode {
        let mut expr = self.term();
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = ExprNode::new_binary(expr, operator, right);
        }
        expr
    }

    fn term(&mut self) -> ExprNode {
        let mut expr = self.factor();
        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = ExprNode::new_binary(expr, operator, right);
        }
        expr
    }

    fn factor(&mut self) -> ExprNode {
        let mut expr = self.unary();
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = ExprNode::new_binary(expr, operator, right);
        }
        expr
    }

    fn unary(&mut self) -> ExprNode {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return ExprNode::new_unary(operator, right);
        }
        self.primary()
    }

    fn primary(&mut self) -> ExprNode {
        if self.matches(&[TokenType::False]) {
            return ExprNode::Literal(crate::ast_enum::LiteralNode::False);
        }

        if self.matches(&[TokenType::True]) {
            return ExprNode::Literal(crate::ast_enum::LiteralNode::True);
        }
        if self.matches(&[TokenType::Nil]) {
            return ExprNode::Literal(crate::ast_enum::LiteralNode::Nil);
        }
        if self.matches(&[TokenType::Number, TokenType::String]) {
            match &self.previous().literal {
                crate::token::Literal::Number(n) => {
                    return ExprNode::Literal(crate::ast_enum::LiteralNode::Number(*n));
                }
                crate::token::Literal::String(s) => {
                    return ExprNode::Literal(crate::ast_enum::LiteralNode::String(s.clone()));
                }
                _ => panic!("Shouldnt be none"),
            }
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect \')\' after expression");
            return ExprNode::new_grouping(expr);
        }

        unreachable!("One of the primary if statments should have matched")
    }

    // TODO Should return token
    fn consume(&mut self, tt: TokenType, err_msg: &str) {
        if self.check(&tt) {
            let _ = self.advance();
        } else {
            self.report(err_msg);
        }
    }

    fn report(&mut self, msg: &str) {
        todo!();
    }

    fn matches(&mut self, ops: &[TokenType]) -> bool {
        for op in ops.iter() {
            if self.check(op) {
                let _ = self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, tt: &TokenType) -> bool {
        if self.peek().t_type == TokenType::Eof {
            return false;
        }
        &self.peek().t_type == tt
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.peek().t_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }
}
