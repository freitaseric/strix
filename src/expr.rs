use crate::{
    tokenizer::{Literal, Token},
    visitor::Visitor,
};

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn new_binary(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Self::Binary {
            left,
            operator,
            right,
        }
    }
    pub fn new_grouping(expression: Box<Expr>) -> Self {
        Self::Grouping { expression }
    }
    pub fn new_literal(value: Literal) -> Self {
        Self::Literal { value }
    }
    pub fn new_unary(operator: Token, right: Box<Expr>) -> Self {
        Self::Unary { operator, right }
    }

    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}
