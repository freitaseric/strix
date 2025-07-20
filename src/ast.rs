use crate::{
    expr::Expr,
    tokenizer::{Literal, Token},
    visitor::Visitor,
};

pub struct AstPrinter;

#[allow(dead_code)]
impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&mut self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut buffer = String::new();
        buffer.push('(');
        buffer.push_str(name);

        for expr in exprs {
            buffer.push(' ');
            buffer.push_str(&expr.accept(self));
        }

        buffer.push(')');
        buffer
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(
        &mut self,
        left: &Box<Expr>,
        operator: &Token,
        right: &Box<Expr>,
    ) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&mut self, expression: &Box<Expr>) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal_expr(&mut self, value: &crate::tokenizer::Literal) -> String {
        match value {
            Literal::Nil => "nil".to_string(),
            Literal::Number(n) => n.to_string(),
            Literal::String(s) => s.clone(),
            Literal::Bool(b) => b.to_string(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Box<Expr>) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
}
