use std::{env, process};

use crate::strix::Strix;

mod ast;
mod err;
mod expr;
mod parser;
mod scanner;
mod strix;
mod tokenizer;
mod visitor;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut strix = Strix::new();

    if args.len() > 1 {
        println!("Usage: strix [script]");
        process::exit(64);
    } else if args.len() == 1 {
        strix.run_file(&args[0]);
    } else {
        strix.run_prompt();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::AstPrinter,
        expr::Expr,
        tokenizer::{Literal, Token, TokenType},
    };

    #[test]
    fn pretty_print_ast() {
        // Represents the expression: -123 * (45.67)
        let expression = Expr::Binary {
            left: Box::new(Expr::Unary {
                operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
                right: Box::new(Expr::Literal {
                    value: Literal::Number(123.0),
                }),
            }),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Literal::Number(45.67),
                }),
            }),
        };

        // Creates an instance of the visitor
        let mut printer = AstPrinter;

        // Uses the `accept` method to start the visitation process
        let result = expression.accept(&mut printer);

        // Prints the result
        println!("{}", result);
        assert_eq!(result, "(* (- 123) (group 45.67))".to_string())
    }
}
