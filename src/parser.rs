use crate::{
    ast::{BinaryExpr, LiteralExpr, Literals, UnaryExpr},
    ast_visitor::{AstAcceptor, AstVisitor},
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

    fn expression<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        self.equality()
    }

    fn equality<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        let mut expr = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            let expr = BinaryExpr {
                left: expr,
                operator,
                right,
            };
        }
        expr
    }

    fn comparison<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        let mut expr = self.term();
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            let expr = BinaryExpr {
                left: expr,
                operator,
                right,
            };
        }
        expr
    }

    fn term<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        let mut expr = self.factor();
        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            let expr = BinaryExpr {
                left: expr,
                operator,
                right,
            };
        }
        expr
    }

    fn factor<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        let mut expr = self.unary();
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            let expr = BinaryExpr {
                left: expr,
                operator,
                right,
            };
        }
        expr
    }

    fn unary<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return UnaryExpr { operator, right };
        }
        self.primary()
    }

    fn primary<A: AstAcceptor<V>, V: AstVisitor>(&mut self) -> A {
        if self.matches(&[TokenType::False]) {
            return LiteralExpr {
                value: Literals::False,
            };
        }
        
        if self.matches(&[TokenType::True]) {
            return LiteralExpr {
                value: Literals::True,
            };
        }
        if self.matches(&[TokenType::Nil]) {
            return LiteralExpr {
                value: Literals::Nil,
            };
        }
        if self.matches(&[TokenType::Number, TokenType::String]) {
            let value = match self.previous().literal {
                crate::token::Literal::Number(n) => Literals::Number(n),
                crate::token::Literal::String(s) => Literals::String(s),
                _ => panic!("Shouldnt be none"),
            };
            // TODO Is it okay that literal has a none variant?
            return LiteralExpr {
                value,
            };
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect \')\' after expression");
            return expr;
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
