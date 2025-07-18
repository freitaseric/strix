use crate::{
    err::StrixError,
    tokenizer::{Literal, Token, TokenType, get_keyword_token},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn from(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            current: 0,
            line: 0,
            start: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(err) = self.scan_token() {
                eprintln!("{:#?}", err)
            }
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), StrixError> {
        if let Some(c) = self.advance() {
            match c {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => {
                    if self.expect('=') {
                        self.add_token(TokenType::BangEqual, None);
                    } else {
                        self.add_token(TokenType::Bang, None);
                    }
                }
                '=' => {
                    if self.expect('=') {
                        self.add_token(TokenType::EqualEqual, None);
                    } else {
                        self.add_token(TokenType::Equal, None);
                    }
                }
                '<' => {
                    if self.expect('=') {
                        self.add_token(TokenType::LessEqual, None);
                    } else {
                        self.add_token(TokenType::Less, None);
                    }
                }
                '>' => {
                    if self.expect('=') {
                        self.add_token(TokenType::GreaterEqual, None);
                    } else {
                        self.add_token(TokenType::Greater, None);
                    }
                }
                '/' => {
                    if self.expect('/') {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }

                ' ' | '\r' | '\t' => { /* Ignore whitespaces */ }

                '\n' => {
                    self.line += 1;
                }

                '"' => return self.string(),

                'o' => {
                    if self.expect('r') {
                        self.add_token(TokenType::Or, None);
                    }
                }

                _ => {
                    if c.is_numeric() {
                        self.number();
                    } else if c.is_alphabetic() {
                        self.identifier();
                    } else {
                        return Err(StrixError::ScannerError(
                            self.line,
                            "Unexpected character.".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;

        self.source.chars().nth(self.current)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal,
            self.line,
        ));
    }

    fn expect(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        };

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn string(&mut self) -> Result<(), StrixError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(StrixError::ScannerError(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current + 1];
        self.add_token(TokenType::String, Some(Literal::String(value.to_string())));

        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        if let Ok(value) = &self.source[self.start..self.current].parse::<f64>() {
            self.add_token(TokenType::Number, Some(Literal::Number(value.to_owned())));
        }
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        if let Some(token_type) = get_keyword_token(text) {
            self.add_token(token_type, None);
        } else {
            self.add_token(TokenType::Identifier, None);
        }
    }
}
