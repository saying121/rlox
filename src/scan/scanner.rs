#![allow(unfulfilled_lint_expectations, reason = "allow it")]
use std::{rc::Rc, str::CharIndices};

use itertools::PeekNth;

use crate::tokens::{Token, TokenInner};

// #[derive(Clone)]
#[derive(Debug)]
pub struct Scanner<'s> {
    source_chars: PeekNth<CharIndices<'s>>,
    source: &'s str,
}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s str) -> Self {
        Scanner {
            source_chars: itertools::peek_nth(source.char_indices()),
            source,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some((idx, ch)) = self.source_chars.next() {
            let token = match ch {
                white if white.is_whitespace() => continue,
                // > one char tokens
                '(' => Token::LeftParen {
                    inner: TokenInner::new_left_paren(self.origin(), idx),
                },
                ')' => Token::RightParen {
                    inner: TokenInner::new_right_paren(self.origin(), idx),
                },
                '{' => Token::LeftBrace {
                    inner: TokenInner::new_left_brace(self.origin(), idx),
                },
                '}' => Token::RightBrace {
                    inner: TokenInner::new_right_brace(self.origin(), idx),
                },
                ',' => Token::Comma {
                    inner: TokenInner::new_comma(self.origin(), idx),
                },
                '.' => Token::Dot {
                    inner: TokenInner::new_dot(self.origin(), idx),
                },
                '-' => Token::Minus {
                    inner: TokenInner::new_minus(self.origin(), idx),
                },
                '+' => Token::Plus {
                    inner: TokenInner::new_plus(self.origin(), idx),
                },
                ';' => Token::Semicolon {
                    inner: TokenInner::new_semicolon(self.origin(), idx),
                },
                '*' => Token::Star {
                    inner: TokenInner::new_star(self.origin(), idx),
                },
                // > two char tokens
                '!' => self.parse_bang(idx),
                '=' => self.parse_equal(idx),
                '<' => self.parse_less(idx),
                '>' => self.parse_greater(idx),
                '/' => {
                    let token = self.parse_slash(idx);
                    match token {
                        Token::BlockComment { .. } | Token::Comment { .. } => {
                            continue;
                        },
                        t => t,
                    }
                },
                // > multi char tokens
                '"' => self.parse_string(idx),
                digit if digit.is_ascii_digit() => self.parse_number(digit, idx),
                ident_start if ident_start.is_ascii_alphanumeric() => {
                    self.parse_ident(idx, ident_start)
                },
                other => self.parse_other(other, idx),
            };

            tokens.push(token);
        }

        tokens
    }

    fn keyword_or_ident(inner: TokenInner) -> Token {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match inner.lexeme() {
            "and" => And { inner },
            "or" => Or { inner },
            "class" => Class { inner },
            "super" => Super { inner },
            "this" => This { inner },
            "true" => True { inner },
            "false" => False { inner },
            "while" => While { inner },
            "for" => For { inner },
            "if" => If { inner },
            "else" => Else { inner },
            "print" => Print { inner },
            "fun" => Fun { inner },
            "return" => Return { inner },
            "var" => Var { inner },
            "nil" => Nil { inner },
            "break" => Break { inner },
            _ => Identifier { inner },
        }
    }

    /// !, !=
    fn parse_bang(&mut self, idx: usize) -> Token {
        self.source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Bang {
                inner: TokenInner::new_bang(self.origin(), idx),
            },
            |_eq| Token::BangEqual {
                inner: TokenInner::new_bang_equal(self.origin(), idx),
            },
        )
    }
    /// =, ==
    fn parse_equal(&mut self, idx: usize) -> Token {
        self.source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Equal {
                inner: TokenInner::new_equal(self.origin(), idx),
            },
            |_eq| Token::EqualEqual {
                inner: TokenInner::new_equal_equal(self.origin(), idx),
            },
        )
    }
    /// <, <=
    fn parse_less(&mut self, idx: usize) -> Token {
        self.source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Less {
                inner: TokenInner::new_less(self.origin(), idx),
            },
            |_eq| Token::LessEqual {
                inner: TokenInner::new_less_equal(self.origin(), idx),
            },
        )
    }
    /// >, >=
    fn parse_greater(&mut self, idx: usize) -> Token {
        self.source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Greater {
                inner: TokenInner::new_greater(self.origin(), idx),
            },
            |_eq| Token::GreaterEqual {
                inner: TokenInner::new_greater_equal(self.origin(), idx),
            },
        )
    }

    /// /, //, /* ... */
    fn parse_slash(&mut self, idx: usize) -> Token {
        let slash = '/';
        match self.source_chars.next_if_eq(&(idx + 1, slash)) {
            Some(_) => {
                let comment = self
                    .source_chars
                    .by_ref()
                    .take_while(|&(_, c)| c != '\n')
                    .map(|(_, c)| c)
                    .collect();
                Token::Comment {
                    inner: TokenInner::new(self.origin(), comment, idx),
                }
            },
            None => {
                match self.source_chars.next_if_eq(&(idx + 1, '*')) {
                    Some(_) => {
                        let (mut last_pre, mut last) = ('\0', '\0');

                        let mut count = 0;
                        while let Some(&(_, next)) = self.source_chars.peek_nth(count)
                            && let Some(&(_, next_next)) = self.source_chars.peek_nth(count + 1)
                        {
                            (last_pre, last) = (next, next_next);
                            if next == '*' && next_next == slash {
                                break;
                            }
                            // Count the number of character before "*/"
                            count += 1;
                        }

                        let b_comment: String = self
                            .source_chars
                            .by_ref()
                            .take(count)
                            .map(|(_, c)| c)
                            .collect();

                        // consume the next two characters regardless, even not ('*','/')
                        self.source_chars.next();
                        self.source_chars.next();

                        if last_pre == '*' && last == '/' {
                            Token::BlockComment {
                                inner: TokenInner::new(self.origin(), b_comment, idx),
                            }
                        }
                        else {
                            Token::Invalid {
                                inner: TokenInner::new_invalid(
                                    self.origin(),
                                    "Invalid block comment, not end with `*/`".to_owned(),
                                    self.source.len() - idx,
                                    idx,
                                ),
                            }
                        }
                    },
                    None => Token::Slash {
                        inner: TokenInner::new_slash(self.origin(), idx),
                    },
                }
            },
        }
    }

    /// `"..."`, `"...`, `"...\"...\\\n..."`
    fn parse_string(&mut self, idx: usize) -> Token {
        let mut res_str = String::new();

        let mut last_matched = '\0';
        let mut need_escape = false;
        let mut str_end = false;

        loop {
            let string: String = self
                .source_chars
                .by_ref()
                .take_while(|&(_, c)| {
                    last_matched = c;
                    if need_escape {
                        need_escape = false;
                        return true;
                    }

                    if c == '"' {
                        str_end = true;
                        false
                    }
                    else if c == '\\' {
                        need_escape = true;
                        false
                    }
                    else {
                        true
                    }
                })
                .map(|(_, c)| c)
                .collect();

            res_str.push_str(&string);
            if last_matched == '"' && str_end || self.source_chars.peek().is_none() {
                break;
            }
        }

        match last_matched {
            '"' => Token::String {
                inner: TokenInner::new(self.origin(), res_str, idx),
            },
            // When does not end with '"' that may indicate EOF
            _ => Token::Invalid {
                inner: TokenInner::new_invalid(
                    self.origin(),
                    r#"Invalid string token, not end with `"`"#.to_owned(),
                    self.source.len() - idx,
                    idx,
                ),
            },
        }
    }

    fn parse_number(&mut self, first: char, idx: usize) -> Token {
        let mut its = Vec::with_capacity(4);
        its.push(first.to_string());

        let mut count = 0;
        while let Some(&(_, ch)) = self.source_chars.peek_nth(count)
            && ch.is_ascii_digit()
        {
            count += 1;
        }

        let dig_integer = self
            .source_chars
            .by_ref()
            .take(count)
            .map(|(_, c)| c)
            .collect();

        // `take_while_ref` inner clone full iterator, so expensive
        // let dig_integer = sour_chars
        //     .take_while_ref(|&(_, ch)| ch.is_ascii_digit())
        //     .map(|(_, c)| c)
        //     .collect();

        its.push(dig_integer);

        if let Some(&(_, next)) = self.source_chars.peek_nth(0)
            && let Some(&(_, next_next)) = self.source_chars.peek_nth(1)
            && next == '.'
            && next_next.is_ascii_digit()
        {
            let (_, dot) = unsafe { self.source_chars.next().unwrap_unchecked() };
            its.push(dot.to_string());

            let mut count = 0;

            while let Some(&(_, ch)) = self.source_chars.peek_nth(count)
                && ch.is_ascii_digit()
            {
                count += 1;
            }

            let decimal = self
                .source_chars
                .by_ref()
                .take(count)
                .map(|(_, c)| c)
                .collect();
            its.push(decimal);
        }

        let lexeme = its.join("");
        Token::Number {
            double: lexeme.parse().expect("parse double failed"),
            inner: TokenInner::new(self.origin(), lexeme, idx),
        }
    }

    fn parse_ident(&mut self, idx: usize, ident_start: char) -> Token {
        let mut count = 0;
        while let Some(&(_, c)) = self.source_chars.peek_nth(count)
            && c.is_ascii_alphanumeric()
        {
            count += 1;
        }
        let lexeme: String = self
            .source_chars
            .by_ref()
            .take(count)
            .map(|(_, c)| c)
            .collect();
        let inner = TokenInner::new(self.origin(), format!("{ident_start}{lexeme}"), idx);

        Self::keyword_or_ident(inner)
    }

    fn parse_other(&mut self, other: char, idx: usize) -> Token {
        let mut count = 0;
        while let Some(&(_, c)) = self.source_chars.peek_nth(count) {
            if c.is_ascii_alphanumeric() || c.is_whitespace() {
                break;
            }
            count += 1;
        }
        let ot: String = self
            .source_chars
            .by_ref()
            .take(count)
            .map(|(_, c)| c)
            .collect();
        Token::Invalid {
            inner: TokenInner::new_invalid(
                self.origin(),
                format!("Unknown: {}{}", other, ot),
                count + 1, // add the `other` len
                idx,
            ),
        }
    }

    pub fn origin(&self) -> Rc<str> {
        Rc::from(self.source)
    }

    pub const fn source(&self) -> &str {
        self.source
    }
}
