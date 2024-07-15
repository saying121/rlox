use anyhow::Result;

use crate::{
    lox::Lox,
    tokens::{token::Token, token_type::TokenType},
};

// #[derive(Clone)]
#[derive(Default)]
pub struct Scanner {
    source:     String,
    tokens:     Vec<Token>,

    start:   usize,
    current: usize,
    line:    usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let sour_chars = source.char_indices().peekable();
        // sour_chars.by_ref

        let mut tokens  = Vec::new();
        while let Some((idx,ch)) = sour_chars.next() {
            let token=match ch {
                '('=>TokenType::LeftParen,
                ')'=>TokenType::RightParen,
                '{'=>TokenType::LeftBrace,
                '}'=>TokenType::RightBrace,
                '+' =>TokenType::Plus,
                _ => {
                    TokenType::Invalid(String::new())
                },
            };

            tokens.push(token);
        }

        Self {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            String::new(),
            self.line,
        ));

        std::mem::take(&mut self.tokens)
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let ty = if self.match_next('=') {
                    TokenType::BangEqual
                }
                else {
                    TokenType::Bang
                };
                self.add_token(ty);
            },
            '=' => {
                let ty = if self.match_next('=') {
                    TokenType::BangEqual
                }
                else {
                    TokenType::Equal
                };
                self.add_token(ty);
            },
            '<' => {
                let ty = if self.match_next('=') {
                    TokenType::LessEqual
                }
                else {
                    TokenType::Less
                };
                self.add_token(ty);
            },
            '>' => {
                let ty = if self.match_next('=') {
                    TokenType::GreaterEqual
                }
                else {
                    TokenType::Greater
                };
                self.add_token(ty);
            },
            _ => Lox::error(self.line, "Unexpected character.".to_owned()),
        }
    }
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let Some(next) = self
            .source
            .get(self.current..self.current + 1)
        else {
            return false;
        };
        if next.chars().next().unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
    fn add_token(&mut self, ty: TokenType) {
        self._add_token(ty, String::new());
    }
    fn _add_token(&mut self, ty: TokenType, literal: String) {
        let text = self.source[self.start..self.current].to_owned();
        self.tokens
            .push(Token::new(ty, text, literal, self.line));
    }
    /// next char
    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .get(self.current - 1..self.current)
            .unwrap_or(" ")
            .chars()
            .next()
            .unwrap_or(' ')
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
