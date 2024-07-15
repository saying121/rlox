use std::fmt::Display;

use crate::tokens::token_type::TokenType;

pub struct Token {
    r#type:  TokenType,
    lexeme:  String,
    literal: String,
    line:    usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Self { r#type, lexeme, literal, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} {} {}", self.r#type, self.lexeme, self.literal).fmt(f)
    }
}
