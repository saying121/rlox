use std::str::CharIndices;

use itertools::PeekNth;

use crate::tokens::{TokenInner, TokenType};

// #[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut source_chars = itertools::peek_nth(source.char_indices());

        let mut tokens = Vec::new();
        while let Some((idx, ch)) = source_chars.next() {
            let token = match ch {
                white if white.is_whitespace() => continue,
                // > one char tokens
                '(' => TokenType::LeftParen {
                    inner: TokenInner::new('('.to_string(), idx),
                },
                ')' => TokenType::RightParen {
                    inner: TokenInner::new(')'.to_string(), idx),
                },
                '{' => TokenType::LeftBrace {
                    inner: TokenInner::new('{'.to_string(), idx),
                },
                '}' => TokenType::RightBrace {
                    inner: TokenInner::new('}'.to_string(), idx),
                },
                ',' => TokenType::Comma {
                    inner: TokenInner::new(','.to_string(), idx),
                },
                '.' => TokenType::Dot {
                    inner: TokenInner::new('.'.to_string(), idx),
                },
                '-' => TokenType::Minus {
                    inner: TokenInner::new('-'.to_string(), idx),
                },
                '+' => TokenType::Plus {
                    inner: TokenInner::new('+'.to_string(), idx),
                },
                ';' => TokenType::Semicolon {
                    inner: TokenInner::new(';'.to_string(), idx),
                },
                '*' => TokenType::Star {
                    inner: TokenInner::new('*'.to_string(), idx),
                },
                // > two char tokens
                '!' => Self::parse_bang(&mut source_chars, idx),
                '=' => Self::parse_equal(&mut source_chars, idx),
                '<' => Self::parse_less(&mut source_chars, idx),
                '>' => Self::parse_greater(&mut source_chars, idx),
                '/' => Self::parse_slash(&mut source_chars, idx, &source),
                // > multi char tokens
                '"' => Self::parse_quote(&mut source_chars, idx, &source),
                digit if digit.is_ascii_digit() => {
                    Self::parse_number(digit, &mut source_chars, idx)
                },
                ident_start if ident_start.is_ascii_alphanumeric() => {
                    Self::parse_ident(&mut source_chars, idx, ident_start)
                },
                other => Self::parse_other(&mut source_chars, other, idx),
            };

            tokens.push(token);
        }

        Self { source, tokens }
    }

    fn keyword_or_ident(inner: TokenInner) -> TokenType {
        use TokenType::{
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
    fn parse_bang(chars: &mut PeekNth<CharIndices>, idx: usize) -> TokenType {
        let bang = '!';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || TokenType::Bang {
                inner: TokenInner::new(bang.to_string(), idx),
            },
            |_eq| TokenType::BangEqual {
                inner: TokenInner::new("!=".to_owned(), idx),
            },
        )
    }
    /// =, ==
    fn parse_equal(chars: &mut PeekNth<CharIndices>, idx: usize) -> TokenType {
        let eq = '=';
        chars.next_if_eq(&(idx + 1, eq)).map_or_else(
            || TokenType::Equal {
                inner: TokenInner::new(eq.to_string(), idx),
            },
            |_eq| TokenType::EqualEqual {
                inner: TokenInner::new("==".to_owned(), idx),
            },
        )
    }
    /// <, <=
    fn parse_less(chars: &mut PeekNth<CharIndices>, idx: usize) -> TokenType {
        let less = '<';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || TokenType::Less {
                inner: TokenInner::new(less.to_string(), idx),
            },
            |_eq| TokenType::LessEqual {
                inner: TokenInner::new("<=".to_owned(), idx),
            },
        )
    }
    /// >, >=
    fn parse_greater(chars: &mut PeekNth<CharIndices>, idx: usize) -> TokenType {
        let greater = &'>';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || TokenType::Greater {
                inner: TokenInner::new(greater.to_string(), idx),
            },
            |_eq| TokenType::GreaterEqual {
                inner: TokenInner::new(">=".to_owned(), idx),
            },
        )
    }

    // /, //, /* ... */
    fn parse_slash(chars: &mut PeekNth<CharIndices>, idx: usize, source: &str) -> TokenType {
        let slash = '/';
        match chars.next_if_eq(&(idx + 1, slash)) {
            Some(_next) => {
                let comment = chars
                    .by_ref()
                    .take_while(|&(_, c)| c != '\n')
                    .map(|(_, c)| c)
                    .collect();
                TokenType::Comment {
                    inner: TokenInner::new(comment, idx),
                }
            },
            None => chars.next_if_eq(&(idx + 1, '*')).map_or_else(
                || TokenType::Slash {
                    inner: TokenInner::new(slash.to_string(), idx),
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
                        TokenType::BlockComment {
                            inner: TokenInner::new(b_comment, idx),
                        }
                    }
                    else {
                        TokenType::Invalid {
                            inner: TokenInner::new_invalid(
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

    // "...", "..., "...\"...\\..."
    fn parse_quote(chars: &mut PeekNth<CharIndices>, idx: usize, source: &str) -> TokenType {
        let mut res_str = String::new();
        let mut last_matched = '\0';
        let mut need_escape = false;

        loop {
            let mut end_flag = false;

            let string: String = chars
                .by_ref()
                .take_while(|&(_, c)| {
                    last_matched = c;
                    if c == '"' {
                        if need_escape {
                            end_flag = false;
                            need_escape = false;
                            true
                        }
                        else {
                            end_flag = true;
                            false
                        }
                    }
                    else if c == '\\' {
                        end_flag = false;
                        if need_escape {
                            need_escape = false;
                            true
                        }
                        else {
                            need_escape = true;
                            false
                        }
                    }
                    else {
                        need_escape = false;
                        end_flag = false;
                        true
                    }
                })
                .map(|(_, c)| c)
                .collect();

            res_str.push_str(&string);
            if last_matched == '"' && end_flag || chars.peek().is_none() {
                break;
            }
        }

        match last_matched {
            '"' => TokenType::String {
                inner: TokenInner::new(res_str, idx),
            },
            _ => TokenType::Invalid {
                inner: TokenInner::new_invalid(
                    r#"Invalid string token, not end with `"`"#.to_owned(),
                    source.len() - idx,
                    idx,
                ),
            },
        }
    }

    fn parse_number(digit: char, chars: &mut PeekNth<CharIndices>, idx: usize) -> TokenType {
        let mut its = Vec::with_capacity(4);
        its.push(digit.to_string());

        let mut count = 0;
        while let Some(&(_, ch)) = chars.peek_nth(count)
            && ch.is_ascii_digit()
        {
            count += 1;
        }

        let dig_integer = chars.by_ref().take(count).map(|(_, c)| c).collect();

        // `take_while_ref` clone full iterator, so expensive
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

            let small = chars.by_ref().take(count).map(|(_, c)| c).collect();
            its.push(small);
        }

        let lexeme = its.join("");
        TokenType::Number {
            double: lexeme.parse().expect("parse double failed"),
            inner:  TokenInner::new(lexeme, idx),
        }
    }

    fn parse_ident(
        source_chars: &mut PeekNth<CharIndices>,
        idx: usize,
        ident_start: char,
    ) -> TokenType {
        let mut count = 0;
        while let Some(&(_, c)) = source_chars.peek_nth(count)
            && c.is_ascii_alphanumeric()
        {
            count += 1;
        }
        let lexeme: String = source_chars.by_ref().take(count).map(|(_, c)| c).collect();
        let inner = TokenInner::new(format!("{ident_start}{lexeme}"), idx);

        Self::keyword_or_ident(inner)
    }

    fn parse_other(source_chars: &mut PeekNth<CharIndices>, other: char, idx: usize) -> TokenType {
        let mut count = 0;
        while let Some(&(_, c)) = source_chars.peek_nth(count) {
            if c.is_ascii_alphanumeric() || c.is_whitespace() {
                break;
            }
            count += 1;
        }
        let ot: String = source_chars.by_ref().take(count).map(|(_, c)| c).collect();
        TokenType::Invalid {
            inner: TokenInner::new_invalid(
                format!("Unknown: {}{}", other, ot),
                count + 1, // add the `other` len
                idx,
            ),
        }
    }

    pub fn tokens(&self) -> &[TokenType] {
        &self.tokens
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
