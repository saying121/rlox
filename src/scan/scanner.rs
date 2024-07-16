use crate::tokens::{MyTokenType, TokenInner};

// #[derive(Clone)]
#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<MyTokenType>,

    start:   usize,
    current: usize,
    line:    usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut sour_chars = itertools::peek_nth(source.char_indices());

        let mut tokens = Vec::new();
        while let Some((idx, ch)) = sour_chars.next() {
            let token = match ch {
                left_paren @ '(' => MyTokenType::LeftParen {
                    inner: TokenInner::new(left_paren.to_string(), idx),
                },
                right_paren @ ')' => MyTokenType::RightParen {
                    inner: TokenInner::new(right_paren.to_string(), idx),
                },
                left_brace @ '{' => MyTokenType::LeftBrace {
                    inner: TokenInner::new(left_brace.to_string(), idx),
                },
                right_barce @ '}' => MyTokenType::RightBrace {
                    inner: TokenInner::new(right_barce.to_string(), idx),
                },
                comma @ ',' => MyTokenType::Comma {
                    inner: TokenInner::new(comma.to_string(), idx),
                },
                dot @ '.' => MyTokenType::Dot {
                    inner: TokenInner::new(dot.to_string(), idx),
                },
                minus @ '-' => MyTokenType::Minus {
                    inner: TokenInner::new(minus.to_string(), idx),
                },
                plus @ '+' => MyTokenType::Plus {
                    inner: TokenInner::new(plus.to_string(), idx),
                },
                semicollon @ ';' => MyTokenType::Semicolon {
                    inner: TokenInner::new(semicollon.to_string(), idx),
                },
                star @ '*' => MyTokenType::Star {
                    inner: TokenInner::new(star.to_string(), idx),
                },
                bang @ '!' => sour_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
                    || MyTokenType::Bang {
                        inner: TokenInner::new(bang.to_string(), idx),
                    },
                    |_eq| MyTokenType::BangEqual {
                        inner: TokenInner::new("!=".to_owned(), idx),
                    },
                ),
                eq @ '=' => sour_chars.next_if_eq(&(idx + 1, eq)).map_or_else(
                    || MyTokenType::Equal {
                        inner: TokenInner::new(eq.to_string(), idx),
                    },
                    |_eq| MyTokenType::EqualEqual {
                        inner: TokenInner::new("==".to_owned(), idx),
                    },
                ),
                less @ '<' => sour_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
                    || MyTokenType::Less {
                        inner: TokenInner::new(less.to_string(), idx),
                    },
                    |_eq| MyTokenType::LessEqual {
                        inner: TokenInner::new("<=".to_owned(), idx),
                    },
                ),
                greater @ '>' => sour_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
                    || MyTokenType::Greater {
                        inner: TokenInner::new(greater.to_string(), idx),
                    },
                    |_eq| MyTokenType::GreaterEqual {
                        inner: TokenInner::new(">=".to_owned(), idx),
                    },
                ),
                slash @ '/' => match sour_chars.next_if_eq(&(idx + 1, '/')) {
                    Some(_next) => {
                        let mut last = '\0';
                        let comment = sour_chars
                            .by_ref()
                            .take_while(|&(_, c)| {
                                last = c;
                                c != '\n'
                            })
                            .map(|(_, c)| c)
                            .collect();
                        match last {
                            '\n' => MyTokenType::Comment {
                                inner: TokenInner::new(comment, idx),
                            },
                            _ => MyTokenType::Invalid {
                                inner: TokenInner::new("Invalid comment".to_owned(), idx),
                            },
                        }
                    },
                    None => sour_chars.next_if_eq(&(idx + 1, '*')).map_or_else(
                        || MyTokenType::Slash {
                            inner: TokenInner::new(slash.to_string(), idx),
                        },
                        |_next| {
                            let (mut last_pre, mut last) = ('\0', '\0');

                            let mut count = 0;
                            while let Some(&(_, next)) = sour_chars.peek_nth(count)
                                && let Some(&(_, next_next)) = sour_chars.peek_nth(count + 1)
                            {
                                (last_pre, last) = (next, next_next);
                                if next == '*' && next_next == '/' {
                                    break;
                                }
                                count += 1;
                            }

                            let b_comment: String =
                                sour_chars.by_ref().take(count).map(|(_, c)| c).collect();
                            if last_pre == '*' && last == '/' {
                                sour_chars.next();
                                sour_chars.next();
                                MyTokenType::BlockComment {
                                    inner: TokenInner::new(b_comment, idx),
                                }
                            }
                            else {
                                MyTokenType::Invalid {
                                    inner: TokenInner::new("Invalid block comment".to_owned(), idx),
                                }
                            }
                        },
                    ),
                },
                white if white.is_whitespace() => continue,
                // ' ' | '\r' | '\t' | '\n' => continue,
                '"' => {
                    let mut last_matched = '\0';
                    let string = sour_chars
                        .by_ref()
                        .take_while(|&(_, c)| {
                            last_matched = c;
                            c != '"'
                        })
                        .map(|(_, c)| c)
                        .collect();
                    match last_matched {
                        '"' => MyTokenType::String {
                            inner: TokenInner::new(string, idx),
                        },
                        _ => MyTokenType::Invalid {
                            inner: TokenInner::new("Invalid string token".to_owned(), idx),
                        },
                    }
                },
                digit if digit.is_ascii_digit() => {
                    let mut its = Vec::with_capacity(4);
                    its.push(digit.to_string());

                    let mut count = 0;
                    while let Some(&(_, ch)) = sour_chars.peek_nth(count)
                        && ch.is_ascii_digit()
                    {
                        count += 1;
                    }

                    let dig_integer = sour_chars.by_ref().take(count).map(|(_, c)| c).collect();
                    its.push(dig_integer);

                    if let Some(&(_, next)) = sour_chars.peek_nth(0)
                        && let Some(&(_, next_next)) = sour_chars.peek_nth(1)
                        && next == '.'
                        && next_next.is_ascii_digit()
                    {
                        #[allow(clippy::unwrap_used)]
                        let (_, dot) = sour_chars.next().unwrap();
                        its.push(dot.to_string());

                        let mut count = 0;

                        while let Some(&(_, ch)) = sour_chars.peek_nth(count)
                            && ch.is_ascii_digit()
                        {
                            count += 1;
                        }

                        let small = sour_chars.by_ref().take(count).map(|(_, c)| c).collect();
                        its.push(small);
                    }

                    let lexeme = its.join("");
                    MyTokenType::Number {
                        double: lexeme.parse().expect("parse double failed"),
                        inner:  TokenInner::new(lexeme, idx),
                    }
                },
                other => MyTokenType::Invalid {
                    inner: TokenInner::new(other.to_string(), idx),
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

    fn is_block_comment_end() -> bool {
        unimplemented!()
    }

    // pub fn scan_tokens(&mut self) -> Vec<Token> {
    //     while !self.is_at_end() {
    //         self.start = self.current;
    //         self.scan_token();
    //     }
    //     self.tokens.push(Token::new(
    //         TokenType::Eof,
    //         String::new(),
    //         String::new(),
    //         self.line,
    //     ));
    //
    //     std::mem::take(&mut self.tokens)
    // }
    // fn scan_token(&mut self) {
    //     let c = self.advance();
    //     match c {
    //         '(' => self.add_token(TokenType::LeftParen),
    //         ')' => self.add_token(TokenType::RightParen),
    //         '{' => self.add_token(TokenType::LeftBrace),
    //         '}' => self.add_token(TokenType::RightBrace),
    //         ',' => self.add_token(TokenType::Comma),
    //         '.' => self.add_token(TokenType::Dot),
    //         '-' => self.add_token(TokenType::Minus),
    //         '+' => self.add_token(TokenType::Plus),
    //         ';' => self.add_token(TokenType::Semicolon),
    //         '*' => self.add_token(TokenType::Star),
    //         '!' => {
    //             let ty = if self.match_next('=') {
    //                 TokenType::BangEqual
    //             }
    //             else {
    //                 TokenType::Bang
    //             };
    //             self.add_token(ty);
    //         },
    //         '=' => {
    //             let ty = if self.match_next('=') {
    //                 TokenType::BangEqual
    //             }
    //             else {
    //                 TokenType::Equal
    //             };
    //             self.add_token(ty);
    //         },
    //         '<' => {
    //             let ty = if self.match_next('=') {
    //                 TokenType::LessEqual
    //             }
    //             else {
    //                 TokenType::Less
    //             };
    //             self.add_token(ty);
    //         },
    //         '>' => {
    //             let ty = if self.match_next('=') {
    //                 TokenType::GreaterEqual
    //             }
    //             else {
    //                 TokenType::Greater
    //             };
    //             self.add_token(ty);
    //         },
    //         _ => Lox::error(self.line, "Unexpected character.".to_owned()),
    //     }
    // }
    // fn match_next(&mut self, expected: char) -> bool {
    //     if self.is_at_end() {
    //         return false;
    //     }
    //     let Some(next) = self
    //         .source
    //         .get(self.current..self.current + 1)
    //     else {
    //         return false;
    //     };
    //     if next.chars().next().unwrap() != expected {
    //         return false;
    //     }
    //     self.current += 1;
    //     true
    // }
    // fn add_token(&mut self, ty: TokenType) {
    //     self._add_token(ty, String::new());
    // }
    // fn _add_token(&mut self, ty: TokenType, literal: String) {
    //     let text = self.source[self.start..self.current].to_owned();
    //     self.tokens
    //         .push(Token::new(ty, text, literal, self.line));
    // }
    // /// next char
    // fn advance(&mut self) -> char {
    //     self.current += 1;
    //     self.source
    //         .get(self.current - 1..self.current)
    //         .unwrap_or(" ")
    //         .chars()
    //         .next()
    //         .unwrap_or(' ')
    // }
    // fn is_at_end(&self) -> bool {
    //     self.current >= self.source.len()
    // }
}
