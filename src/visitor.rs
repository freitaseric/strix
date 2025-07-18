use crate::{
    expr::Expr,
    tokenizer::{Literal, Token},
};

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> T;
    fn visit_grouping_expr(&mut self, expression: &Box<Expr>) -> T;
    fn visit_literal_expr(&mut self, value: &Literal) -> T;
    fn visit_unary_expr(&mut self, operator: &Token, right: &Box<Expr>) -> T;
}
