use std::{error::Error, fmt};

use crate::tokenizer::{Token, TokenType};

#[derive(Debug)]
pub enum StrixError {
    ParserError(Token, String),
    ScannerError(usize, String),
}

impl fmt::Display for StrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParserError(t, m) => {
                if t.token_type == TokenType::Eof {
                    write!(f, "[line {}] Error at the end: {}", t.line, m)
                } else {
                    write!(f, "[line {}] Error at '{}': {}", t.line, t.lexeme, m)
                }
            }
            StrixError::ScannerError(l, m) => write!(f, "[line {l}] Error: {m}"),
        }
    }
}

impl Error for StrixError {}
