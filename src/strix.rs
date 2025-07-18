use std::{
    fs,
    io::{self, Write},
    process,
};

use crate::{ast::AstPrinter, parser::Parser, scanner::Scanner};

pub struct Strix {
    had_error: bool,
}

impl Strix {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    fn run(&mut self, source: String) {
        let mut scanner = Scanner::from(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::from(tokens.clone());
        match parser.parse() {
            Ok(expr) => {
                let mut ast = AstPrinter;
                println!("{:#?}", ast.print(expr))
            }
            Err(err) => {
                self.had_error = true;
                eprintln!("{:#?}", err)
            }
        };
    }

    pub fn run_prompt(&mut self) {
        loop {
            println!("Strix Language REPL | v1.0");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Unable to read line");

            let line = input.trim();

            if line == "Strix::clear()" || line == ".clear" {
                clearscreen::clear().unwrap();
                continue;
            }

            if line == "Strix::exit()" || line == ".exit" {
                println!("Bye!");
                process::exit(0);
            }

            self.run(String::from(line));
            self.had_error = false;
        }
    }

    pub fn run_file(&mut self, path: &String) {
        match fs::read(path) {
            Ok(bytes) => {
                self.run(String::from_utf8(bytes).expect("Invalid UTF-8 byte found"));

                if self.had_error {
                    process::exit(65);
                }
            }

            Err(err) => panic!("{err:#?}"),
        };
    }
}
