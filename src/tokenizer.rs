use std::fmt;

use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    => TokenType::And,
    "class"  => TokenType::Class,
    "else"   => TokenType::Else,
    "false"  => TokenType::False,
    "for"    => TokenType::For,
    "fun"    => TokenType::Fun,
    "if"     => TokenType::If,
    "nil"    => TokenType::Nil,
    "or"     => TokenType::Or,
    "print"  => TokenType::Print,
    "return" => TokenType::Return,
    "super"  => TokenType::Super,
    "this"   => TokenType::This,
    "true"   => TokenType::True,
    "var"    => TokenType::Var,
    "while"  => TokenType::While,
};

pub fn get_keyword_token(keyword: &str) -> Option<TokenType> {
    KEYWORDS.get(keyword).cloned()
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_str = match &self.literal {
            Some(lit) => lit.to_string(),
            None => "nil".to_string(),
        };

        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_str)
    }
}
