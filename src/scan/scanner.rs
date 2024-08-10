use std::{str::CharIndices, sync::Arc};

use itertools::PeekNth;

use crate::tokens::{Token, TokenInner};

// #[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Scanner {
    source: Arc<str>,
    // tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        // let mut source_chars = itertools::peek_nth(source.char_indices());

        // let tokens = Self::scan(source_chars, &source);

        Self {
            source: Arc::from(source),
        }
    }

    pub fn source_code(&self) -> Arc<str> {
        Arc::clone(&self.source)
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        let mut source_chars = itertools::peek_nth(self.source.char_indices());
        let mut tokens = Vec::new();
        while let Some((idx, ch)) = source_chars.next() {
            let token = match ch {
                white if white.is_whitespace() => continue,
                // > one char tokens
                '(' => Token::LeftParen {
                    inner: TokenInner::new(self.source_code(), '('.to_string(), idx),
                },
                ')' => Token::RightParen {
                    inner: TokenInner::new(self.source_code(), ')'.to_string(), idx),
                },
                '{' => Token::LeftBrace {
                    inner: TokenInner::new(self.source_code(), '{'.to_string(), idx),
                },
                '}' => Token::RightBrace {
                    inner: TokenInner::new(self.source_code(), '}'.to_string(), idx),
                },
                ',' => Token::Comma {
                    inner: TokenInner::new(self.source_code(), ','.to_string(), idx),
                },
                '.' => Token::Dot {
                    inner: TokenInner::new(self.source_code(), '.'.to_string(), idx),
                },
                '-' => Token::Minus {
                    inner: TokenInner::new(self.source_code(), '-'.to_string(), idx),
                },
                '+' => Token::Plus {
                    inner: TokenInner::new(self.source_code(), '+'.to_string(), idx),
                },
                ';' => Token::Semicolon {
                    inner: TokenInner::new(self.source_code(), ';'.to_string(), idx),
                },
                '*' => Token::Star {
                    inner: TokenInner::new(self.source_code(), '*'.to_string(), idx),
                },
                // > two char tokens
                '!' => self.parse_bang(&mut source_chars, idx),
                '=' => self.parse_equal(&mut source_chars, idx),
                '<' => self.parse_less(&mut source_chars, idx),
                '>' => self.parse_greater(&mut source_chars, idx),
                '/' => self.parse_slash(&mut source_chars, idx, &self.source),
                // > multi char tokens
                '"' => self.parse_string(&mut source_chars, idx, &self.source),
                digit if digit.is_ascii_digit() => self.parse_number(digit, &mut source_chars, idx),
                ident_start if ident_start.is_ascii_alphanumeric() => {
                    self.parse_ident(&mut source_chars, idx, ident_start)
                },
                other => self.parse_other(&mut source_chars, other, idx),
            };

            tokens.push(token);
        }

        tokens
    }

    fn keyword_or_ident(inner: TokenInner) -> Token {
        use Token::{
            And, Class, Else, False, For, Fun, Identifier, If, Nil, Or, Print, Return, Super, This,
            True, Var, While,
        };
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
            _ => Identifier { inner },
        }
    }

    /// !, !=
    fn parse_bang(&self, chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let bang = '!';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Bang {
                inner: TokenInner::new(self.source_code(), bang.to_string(), idx),
            },
            |_eq| Token::BangEqual {
                inner: TokenInner::new(self.source_code(), "!=".to_owned(), idx),
            },
        )
    }
    /// =, ==
    fn parse_equal(&self, chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let eq = '=';
        chars.next_if_eq(&(idx + 1, eq)).map_or_else(
            || Token::Equal {
                inner: TokenInner::new(self.source_code(), eq.to_string(), idx),
            },
            |_eq| Token::EqualEqual {
                inner: TokenInner::new(self.source_code(), "==".to_owned(), idx),
            },
        )
    }
    /// <, <=
    fn parse_less(&self, chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let less = '<';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Less {
                inner: TokenInner::new(self.source_code(), less.to_string(), idx),
            },
            |_eq| Token::LessEqual {
                inner: TokenInner::new(self.source_code(), "<=".to_owned(), idx),
            },
        )
    }
    /// >, >=
    fn parse_greater(&self, chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let greater = &'>';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Greater {
                inner: TokenInner::new(self.source_code(), greater.to_string(), idx),
            },
            |_eq| Token::GreaterEqual {
                inner: TokenInner::new(self.source_code(), ">=".to_owned(), idx),
            },
        )
    }

    // /, //, /* ... */
    fn parse_slash(&self, chars: &mut PeekNth<CharIndices>, idx: usize, source: &str) -> Token {
        let slash = '/';
        match chars.next_if_eq(&(idx + 1, slash)) {
            Some(_next) => {
                let comment = chars
                    .by_ref()
                    .take_while(|&(_, c)| c != '\n')
                    .map(|(_, c)| c)
                    .collect();
                Token::Comment {
                    inner: TokenInner::new(self.source_code(), comment, idx),
                }
            },
            None => chars.next_if_eq(&(idx + 1, '*')).map_or_else(
                || Token::Slash {
                    inner: TokenInner::new(self.source_code(), slash.to_string(), idx),
                },
                |_next| {
                    let (mut last_pre, mut last) = ('\0', '\0');

                    let mut count = 0;
                    while let Some(&(_, next)) = chars.peek_nth(count)
                        && let Some(&(_, next_next)) = chars.peek_nth(count + 1)
                    {
                        (last_pre, last) = (next, next_next);
                        if next == '*' && next_next == slash {
                            break;
                        }
                        // Count the number of character before "*/"
                        count += 1;
                    }

                    let b_comment: String = chars.by_ref().take(count).map(|(_, c)| c).collect();

                    // consume the next two characters regardless, even not ('*','/')
                    chars.next();
                    chars.next();

                    if last_pre == '*' && last == '/' {
                        Token::BlockComment {
                            inner: TokenInner::new(self.source_code(), b_comment, idx),
                        }
                    }
                    else {
                        Token::Invalid {
                            inner: TokenInner::new_invalid(
                                self.source_code(),
                                "Invalid block comment, not end with `*/`".to_owned(),
                                source.len() - idx,
                                idx,
                            ),
                        }
                    }
                },
            ),
        }
    }

    /// `"..."`, `"...`, `"...\"...\\\n..."`
    fn parse_string(&self, chars: &mut PeekNth<CharIndices>, idx: usize, source: &str) -> Token {
        let mut res_str = String::new();

        let mut last_matched = '\0';
        let mut need_escape = false;
        let mut str_end = false;

        loop {
            let string: String = chars
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
            if last_matched == '"' && str_end || chars.peek().is_none() {
                break;
            }
        }

        match last_matched {
            '"' => Token::String {
                inner: TokenInner::new(self.source_code(), res_str, idx),
            },
            // When does not end with '"' that may indicate EOF
            _ => Token::Invalid {
                inner: TokenInner::new_invalid(
                    self.source_code(),
                    r#"Invalid string token, not end with `"`"#.to_owned(),
                    source.len() - idx,
                    idx,
                ),
            },
        }
    }

    fn parse_number(&self, first: char, chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let mut its = Vec::with_capacity(4);
        its.push(first.to_string());

        let mut count = 0;
        while let Some(&(_, ch)) = chars.peek_nth(count)
            && ch.is_ascii_digit()
        {
            count += 1;
        }

        let dig_integer = chars.by_ref().take(count).map(|(_, c)| c).collect();

        // `take_while_ref` inner clone full iterator, so expensive
        // let dig_integer = sour_chars
        //     .take_while_ref(|&(_, ch)| ch.is_ascii_digit())
        //     .map(|(_, c)| c)
        //     .collect();

        its.push(dig_integer);

        if let Some(&(_, next)) = chars.peek_nth(0)
            && let Some(&(_, next_next)) = chars.peek_nth(1)
            && next == '.'
            && next_next.is_ascii_digit()
        {
            #[expect(clippy::unwrap_used, reason = "it must be `Some`")]
            let (_, dot) = chars.next().unwrap();
            its.push(dot.to_string());

            let mut count = 0;

            while let Some(&(_, ch)) = chars.peek_nth(count)
                && ch.is_ascii_digit()
            {
                count += 1;
            }

            let decimal = chars.by_ref().take(count).map(|(_, c)| c).collect();
            its.push(decimal);
        }

        let lexeme = its.join("");
        Token::Number {
            double: lexeme.parse().expect("parse double failed"),
            inner: TokenInner::new(self.source_code(), lexeme, idx),
        }
    }

    fn parse_ident(
        &self,
        source_chars: &mut PeekNth<CharIndices>,
        idx: usize,
        ident_start: char,
    ) -> Token {
        let mut count = 0;
        while let Some(&(_, c)) = source_chars.peek_nth(count)
            && c.is_ascii_alphanumeric()
        {
            count += 1;
        }
        let lexeme: String = source_chars.by_ref().take(count).map(|(_, c)| c).collect();
        let inner = TokenInner::new(self.source_code(), format!("{ident_start}{lexeme}"), idx);

        Self::keyword_or_ident(inner)
    }

    fn parse_other(
        &self,
        source_chars: &mut PeekNth<CharIndices>,
        other: char,
        idx: usize,
    ) -> Token {
        let mut count = 0;
        while let Some(&(_, c)) = source_chars.peek_nth(count) {
            if c.is_ascii_alphanumeric() || c.is_whitespace() {
                break;
            }
            count += 1;
        }
        let ot: String = source_chars.by_ref().take(count).map(|(_, c)| c).collect();
        Token::Invalid {
            inner: TokenInner::new_invalid(
                self.source_code(),
                format!("Unknown: {}{}", other, ot),
                count + 1, // add the `other` len
                idx,
            ),
        }
    }

    // pub fn tokens(&self) -> &[Token] {
    //     &self.tokens
    // }

    pub fn source(&self) -> &str {
        &self.source
    }
}
