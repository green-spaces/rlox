use crate::{
    ast_enum::{ExprNode, LiteralNode},
    enum_stmt::{BlockNode, IfNode, StmtAcceptorMut, StmtNode, StmtVisitorMut, VarNode},
    token::{Token, TokenLiteral, TokenType},
};
use std::io::{self, Write};

use super::SyntaxError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<StmtNode>, SyntaxError> {
        let mut stmts = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => stmts.push(stmt),
                Err(SyntaxError::UnmatchedToken(token, msg)) => {
                    self.report(token.line, &token.lexeme, &msg);
                    self.syncchronize();
                }
                Err(SyntaxError::ExpectedToken(_, token, msg)) => {
                    self.report(token.line, &token.lexeme, &msg);
                    self.syncchronize();
                }
                Err(SyntaxError::InvalidAssignment(token)) => {
                    self.report(token.line, "", "Invalid assignement target");
                    self.syncchronize();
                }
            }
        }
        // TODO Do I need to return a copy of the errors here too?
        Ok(stmts)
    }

    #[allow(dead_code)]
    fn syncchronize(&mut self) {
        let _ = self.advance();
        while !self.is_at_end() {
            // Semicolon clearly indicates we have found the end of the current statement
            if self.previous().t_type == TokenType::Semicolon {
                return;
            }

            // Keywords that start statements
            if [
                TokenType::Class,
                TokenType::For,
                TokenType::Fun,
                TokenType::If,
                TokenType::Print,
                TokenType::Return,
                TokenType::Var,
                TokenType::While,
            ]
            .contains(&self.peek().t_type)
            {
                return;
            }

            let _ = self.advance();
        }
    }

    fn declaration(&mut self) -> Result<StmtNode, SyntaxError> {
        if self.matches(&[TokenType::Var]) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<StmtNode, SyntaxError> {
        let name = self.consume(TokenType::Identifier, "Expected identifier")?;
        let mut initializer = ExprNode::Literal(LiteralNode::Nil);

        if self.matches(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expected ':'")?;
        Ok(StmtNode::VarDec(VarNode::new(name, initializer)))
    }

    fn statement(&mut self) -> Result<StmtNode, SyntaxError> {
        if self.matches(&[TokenType::LeftBrace]) {
            return self.block_statement();
        }

        if self.matches(&[TokenType::If]) {
            return self.if_statement();
        }

        if self.matches(&[TokenType::Print]) {
            return self.print_statement();
        }

        if self.matches(&[TokenType::While]) {
            return self.while_statement();
        }

        if self.matches(&[TokenType::For]) {
            return self.for_statement();
        }

        self.expression_statement()
    }

    // Other statments (if ect cant be used in the initializer
    // for ( dec | expr stmt; expr? ; expr? ) statement
    fn for_statement(&mut self) -> Result<StmtNode, SyntaxError> {
        self.consume(
            TokenType::LeftParen,
            "For initializer must start with left paren '('",
        )?;

        // Get initializer, Statements will consume the semicolon
        let mut initializer = StmtNode::Expr(ExprNode::Literal(LiteralNode::Nil));
        if self.matches(&[TokenType::Var]) {
            initializer = self.var_declaration()?;
        } else if !self.check(&TokenType::Semicolon) {
            initializer = self.expression_statement()?;
        } else {
            self.consume(
                TokenType::Semicolon,
                "For initializer and condition must be seperated by a semicolon ';'",
            )?;
        }

        let mut condition = ExprNode::Literal(LiteralNode::True);
        if !self.check(&TokenType::Semicolon) {
            condition = self.expression()?;
        }

        self.consume(
            TokenType::Semicolon,
            "For condition and increment must be seperated by a semicolon ';'",
        )?;

        let increment = if !self.check(&TokenType::RightParen) {
            let increment_expr = self.expression()?;
            StmtNode::Expr(increment_expr)
        } else {
            StmtNode::Expr(ExprNode::Literal(LiteralNode::Nil))
        };

        self.consume(
            TokenType::RightParen,
            "For increment must be followed by a right paren ')'",
        )?;
        let body = self.statement()?;

        // Desugar to while loop
        let stmts = vec![body, increment];
        let while_body = StmtNode::Block(BlockNode(stmts));
        let while_stmt = StmtNode::new_while(condition, while_body);

        let loop_stmts = vec![initializer, while_stmt];

        println!("{loop_stmts:#?}");

        Ok(StmtNode::Block(BlockNode(loop_stmts)))
    }

    fn while_statement(&mut self) -> Result<StmtNode, SyntaxError> {
        self.consume(
            TokenType::LeftParen,
            "While condition must start with left paren '('",
        )?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "While condition must end with right paren ')'",
        )?;

        let body = self.statement()?;
        Ok(StmtNode::new_while(condition, body))
    }

    fn if_statement(&mut self) -> Result<StmtNode, SyntaxError> {
        self.consume(
            TokenType::LeftParen,
            "If statements require '(' before condition.",
        )?;
        let condition = self.comma_expression()?;
        self.consume(
            TokenType::RightParen,
            "If statements require ')' after condition.",
        )?;

        // TODO Should variable declarations be allowed as the main entry poin
        let then_branch = self.statement()?;

        let mut else_branch = None;
        if self.matches(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }
        let if_stmt = IfNode::new(condition, then_branch, else_branch);
        Ok(StmtNode::IfStmt(if_stmt))
    }

    fn block_statement(&mut self) -> Result<StmtNode, SyntaxError> {
        let mut stmts = Vec::new();

        while !self.matches(&[TokenType::RightBrace]) && !self.is_at_end() {
            stmts.push(self.declaration()?);
        }

        let prev = self.previous();
        if prev.t_type != TokenType::RightBrace {
            return Err(SyntaxError::ExpectedToken(
                TokenType::RightBrace,
                prev.clone(),
                "Expected closing brace '}'".to_string(),
            ));
        }

        Ok(StmtNode::Block(BlockNode(stmts)))
    }

    fn print_statement(&mut self) -> Result<StmtNode, SyntaxError> {
        let expr = self.comma_expression()?;
        self.consume(TokenType::Semicolon, "Expected ';'")?;
        return Ok(StmtNode::Print(expr));
    }

    fn expression_statement(&mut self) -> Result<StmtNode, SyntaxError> {
        let expr = self.comma_expression()?;

        self.consume(TokenType::Semicolon, "Expected ';'")?;
        Ok(StmtNode::Expr(expr))
    }

    /// Allows multiple expressions to be placed where only a single one is expected
    ///
    /// The left expression is evalueated and then discaded if a comma exists. The right most
    /// expression is returned
    ///
    /// Eg comma expr: expr (,expr)*
    fn comma_expression(&mut self) -> Result<ExprNode, SyntaxError> {
        // TODO Is this the correct functionality? The expression is remove but it will never be
        // evaluated
        let mut expr = self.expression()?;

        while self.matches(&[TokenType::Comma]) {
            expr = self.expression()?;
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<ExprNode, SyntaxError> {
        self.assignment()
    }

    // TODO Study this, the logic is a little convoluted
    // https://craftinginterpreters.com/statements-and-state.html#assignment-syntax
    fn assignment(&mut self) -> Result<ExprNode, SyntaxError> {
        let expr = self.logical_or()?;

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr {
                ExprNode::Variable(name) => {
                    return Ok(ExprNode::new_assign(name.clone(), value));
                }
                _ => {
                    return Err(SyntaxError::InvalidAssignment(equals));
                }
            }
        }
        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<ExprNode, SyntaxError> {
        let mut expr = self.logical_and()?;

        while self.matches(&[TokenType::Or]) {
            let token = self.previous().clone();
            let right = self.logical_and()?;
            expr = ExprNode::new_logical(expr, token, right);
        }
        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<ExprNode, SyntaxError> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]) {
            let token = self.previous().clone();
            let right = self.equality()?;
            expr = ExprNode::new_logical(expr, token, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<ExprNode, SyntaxError> {
        let mut expr = self.comparison()?;

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = ExprNode::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<ExprNode, SyntaxError> {
        let mut expr = self.term()?;
        while self.matches(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = ExprNode::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<ExprNode, SyntaxError> {
        let mut expr = self.factor()?;
        while self.matches(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = ExprNode::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<ExprNode, SyntaxError> {
        let mut expr = self.unary()?;
        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = ExprNode::new_binary(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<ExprNode, SyntaxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(ExprNode::new_unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<ExprNode, SyntaxError> {
        if self.matches(&[TokenType::False]) {
            return Ok(ExprNode::Literal(LiteralNode::False));
        }

        if self.matches(&[TokenType::True]) {
            return Ok(ExprNode::Literal(LiteralNode::True));
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(ExprNode::Literal(LiteralNode::Nil));
        }
        if self.matches(&[TokenType::Number, TokenType::String]) {
            match &self.previous().literal {
                TokenLiteral::Number(n) => {
                    return Ok(ExprNode::Literal(LiteralNode::Number(*n)));
                }
                TokenLiteral::String(s) => {
                    return Ok(ExprNode::Literal(LiteralNode::String(s.clone())));
                }
                _ => panic!("Shouldnt be None"),
            }
        }

        if self.matches(&[TokenType::Identifier]) {
            let token = self.previous().clone();
            return Ok(ExprNode::new_variable(token));
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect \')\' after expression")?;
            return Ok(ExprNode::new_grouping(expr));
        }

        let next = self.peek();
        Err(SyntaxError::UnmatchedToken(
            next.clone(),
            "Expected expression".to_string(),
        ))
    }

    fn consume(&mut self, tt: TokenType, err_msg: &str) -> Result<Token, SyntaxError> {
        if self.check(&tt) {
            let token = self.advance();
            Ok(token.clone())
        } else {
            Err(SyntaxError::ExpectedToken(
                tt,
                self.peek().clone(),
                err_msg.to_string(),
            ))
        }
    }

    fn report(&mut self, line: u64, location: &str, msg: &str) {
        let mut stderr = io::stderr();
        stderr
            .write_all(format!("[line {line}] Error around '{location}': {msg}\n").as_bytes())
            .unwrap();
        stderr.flush().unwrap();
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
