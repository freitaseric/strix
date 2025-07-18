use crate::{
    err::StrixError,
    expr::Expr,
    tokenizer::{Literal, Token, TokenType},
    visitor::Visitor,
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn interpret(&mut self, expression: &Expr) -> Result<Literal, StrixError> {
        self.evaluate(expression)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Literal, StrixError> {
        expr.accept(self)
    }

    fn is_truthy(&self, value: &Literal) -> bool {
        match value {
            Literal::Nil => false,
            Literal::Bool(b) => *b,
            _ => true,
        }
    }
}

impl Visitor<Result<Literal, StrixError>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Box<Expr>,
        operator: &crate::tokenizer::Token,
        right: &Box<Expr>,
    ) -> Result<Literal, StrixError> {
        let left_val = self.evaluate(left)?;
        let right_val = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => {
                number_op(operator.clone(), left_val, right_val, |a, b| a - b).map(Literal::Number)
            }
            TokenType::Slash => {
                number_op(operator.clone(), left_val, right_val, |a, b| a / b).map(Literal::Number)
            }
            TokenType::Star => {
                number_op(operator.clone(), left_val, right_val, |a, b| a * b).map(Literal::Number)
            }
            TokenType::Plus => match (left_val, right_val) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => Ok(Literal::String(l + &r)),
                _ => Err(StrixError::InterpreterError(
                    operator.clone(),
                    "The operands must be two numbers or two strings.".to_string(),
                )),
            },

            // Operadores de Comparação
            TokenType::Greater => {
                number_op(operator.clone(), left_val, right_val, |a, b| a > b).map(Literal::Bool)
            }
            TokenType::GreaterEqual => {
                number_op(operator.clone(), left_val, right_val, |a, b| a >= b).map(Literal::Bool)
            }
            TokenType::Less => {
                number_op(operator.clone(), left_val, right_val, |a, b| a < b).map(Literal::Bool)
            }
            TokenType::LessEqual => {
                number_op(operator.clone(), left_val, right_val, |a, b| a <= b).map(Literal::Bool)
            }

            // Operadores de Igualdade
            TokenType::BangEqual => Ok(Literal::Bool(!is_equal(left_val, right_val))),
            TokenType::EqualEqual => Ok(Literal::Bool(is_equal(left_val, right_val))),

            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(&mut self, expression: &Box<Expr>) -> Result<Literal, StrixError> {
        self.evaluate(&expression)
    }

    fn visit_literal_expr(&mut self, value: &Literal) -> Result<Literal, StrixError> {
        Ok(value.clone())
    }

    fn visit_unary_expr(
        &mut self,
        operator: &crate::tokenizer::Token,
        right: &Box<Expr>,
    ) -> Result<Literal, StrixError> {
        let right_val = self.evaluate(&right)?;

        match operator.token_type {
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(&right_val))),
            TokenType::Minus => match right_val {
                Literal::Number(n) => Ok(Literal::Number(-n)),
                _ => Err(StrixError::InterpreterError(
                    operator.clone(),
                    "The operator must be a number".to_string(),
                )),
            },
            _ => unreachable!(),
        }
    }
}

fn number_op<F, T>(op: Token, l: Literal, r: Literal, fun: F) -> Result<T, StrixError>
where
    F: Fn(f64, f64) -> T,
{
    if let (Literal::Number(left_num), Literal::Number(right_num)) = (l, r) {
        Ok(fun(left_num, right_num))
    } else {
        Err(StrixError::InterpreterError(
            op,
            "Operands must be numbers.".to_string(),
        ))
    }
}

fn is_equal(a: Literal, b: Literal) -> bool {
    match (a, b) {
        (Literal::Nil, Literal::Nil) => true,
        (Literal::Nil, _) => false,
        (a, b) => a == b,
    }
}
