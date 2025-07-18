use crate::{
    err::StrixError,
    expr::Expr,
    tokenizer::{Literal, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn from(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) -> Result<Expr, StrixError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, StrixError> {
        let mut expr = self.comparision();

        while self.expect(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparision()?;
            expr = Ok(Expr::new_binary(Box::new(expr?), operator, Box::new(right)));
        }

        expr
    }

    fn expect(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn comparision(&mut self) -> Result<Expr, StrixError> {
        let mut expr = self.term();

        while self.expect(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Ok(Expr::new_binary(Box::new(expr?), operator, Box::new(right)));
        }

        expr
    }

    fn term(&mut self) -> Result<Expr, StrixError> {
        let mut expr = self.factor();

        while self.expect(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Ok(Expr::new_binary(Box::new(expr?), operator, Box::new(right)));
        }

        expr
    }

    fn factor(&mut self) -> Result<Expr, StrixError> {
        let mut expr = self.unary();

        while self.expect(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Ok(Expr::new_binary(Box::new(expr?), operator, Box::new(right)));
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, StrixError> {
        if self.expect(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Ok(Expr::new_unary(operator, Box::new(right?)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, StrixError> {
        if self.expect(&[TokenType::False]) {
            return Ok(Expr::new_literal(Literal::Bool(false)));
        }
        if self.expect(&[TokenType::True]) {
            return Ok(Expr::new_literal(Literal::Bool(true)));
        }
        if self.expect(&[TokenType::Nil]) {
            return Ok(Expr::new_literal(Literal::Nil));
        }

        if self.expect(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::new_literal(
                self.previous().literal.unwrap_or(Literal::Nil),
            ));
        }

        if self.expect(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(
                &TokenType::RightParen,
                "Expect ')' after expression.".to_string(),
            )?;
            return Ok(Expr::new_grouping(Box::new(expr)));
        }

        Err(StrixError::ParserError(
            self.peek(),
            "Expected expression.".to_string(),
        ))
    }

    fn consume(&mut self, token_type: &TokenType, message: String) -> Result<Token, StrixError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(StrixError::ParserError(self.peek(), message))
        }
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Expr, StrixError> {
        self.expression()
    }
}
