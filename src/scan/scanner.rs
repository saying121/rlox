use std::str::CharIndices;

use itertools::PeekNth;

use crate::tokens::{TokenInner, Token};

// #[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut source_chars = itertools::peek_nth(source.char_indices());

        let mut tokens = Vec::new();
        while let Some((idx, ch)) = source_chars.next() {
            let token = match ch {
                white if white.is_whitespace() => continue,
                // > one char tokens
                '(' => Token::LeftParen {
                    inner: TokenInner::new('('.to_string(), idx),
                },
                ')' => Token::RightParen {
                    inner: TokenInner::new(')'.to_string(), idx),
                },
                '{' => Token::LeftBrace {
                    inner: TokenInner::new('{'.to_string(), idx),
                },
                '}' => Token::RightBrace {
                    inner: TokenInner::new('}'.to_string(), idx),
                },
                ',' => Token::Comma {
                    inner: TokenInner::new(','.to_string(), idx),
                },
                '.' => Token::Dot {
                    inner: TokenInner::new('.'.to_string(), idx),
                },
                '-' => Token::Minus {
                    inner: TokenInner::new('-'.to_string(), idx),
                },
                '+' => Token::Plus {
                    inner: TokenInner::new('+'.to_string(), idx),
                },
                ';' => Token::Semicolon {
                    inner: TokenInner::new(';'.to_string(), idx),
                },
                '*' => Token::Star {
                    inner: TokenInner::new('*'.to_string(), idx),
                },
                // > two char tokens
                '!' => Self::parse_bang(&mut source_chars, idx),
                '=' => Self::parse_equal(&mut source_chars, idx),
                '<' => Self::parse_less(&mut source_chars, idx),
                '>' => Self::parse_greater(&mut source_chars, idx),
                '/' => Self::parse_slash(&mut source_chars, idx, &source),
                // > multi char tokens
                '"' => Self::parse_string(&mut source_chars, idx, &source),
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
    fn parse_bang(chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let bang = '!';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Bang {
                inner: TokenInner::new(bang.to_string(), idx),
            },
            |_eq| Token::BangEqual {
                inner: TokenInner::new("!=".to_owned(), idx),
            },
        )
    }
    /// =, ==
    fn parse_equal(chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let eq = '=';
        chars.next_if_eq(&(idx + 1, eq)).map_or_else(
            || Token::Equal {
                inner: TokenInner::new(eq.to_string(), idx),
            },
            |_eq| Token::EqualEqual {
                inner: TokenInner::new("==".to_owned(), idx),
            },
        )
    }
    /// <, <=
    fn parse_less(chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let less = '<';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Less {
                inner: TokenInner::new(less.to_string(), idx),
            },
            |_eq| Token::LessEqual {
                inner: TokenInner::new("<=".to_owned(), idx),
            },
        )
    }
    /// >, >=
    fn parse_greater(chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
        let greater = &'>';
        chars.next_if_eq(&(idx + 1, '=')).map_or_else(
            || Token::Greater {
                inner: TokenInner::new(greater.to_string(), idx),
            },
            |_eq| Token::GreaterEqual {
                inner: TokenInner::new(">=".to_owned(), idx),
            },
        )
    }

    // /, //, /* ... */
    fn parse_slash(chars: &mut PeekNth<CharIndices>, idx: usize, source: &str) -> Token {
        let slash = '/';
        match chars.next_if_eq(&(idx + 1, slash)) {
            Some(_next) => {
                let comment = chars
                    .by_ref()
                    .take_while(|&(_, c)| c != '\n')
                    .map(|(_, c)| c)
                    .collect();
                Token::Comment {
                    inner: TokenInner::new(comment, idx),
                }
            },
            None => chars.next_if_eq(&(idx + 1, '*')).map_or_else(
                || Token::Slash {
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
                        Token::BlockComment {
                            inner: TokenInner::new(b_comment, idx),
                        }
                    }
                    else {
                        Token::Invalid {
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

    /// `"..."`, `"...`, `"...\"...\\\n..."`
    fn parse_string(chars: &mut PeekNth<CharIndices>, idx: usize, source: &str) -> Token {
        let mut res_str = String::new();

        let mut last_matched = '\0';
        let mut need_escape = false;
        let mut str_end = false;

        loop {
            let string: String = chars
                .by_ref()
                .take_while(|&(_, c)| {
                    last_matched = c;
                    if c == '"' {
                        str_end = !need_escape; // If need to escape, don't terminate the string.
                        let need_take = need_escape; // If need to escape, take the char
                        need_escape = false;
                        need_take
                    }
                    else if c == '\\' {
                        str_end = false;
                        let need_take = need_escape; // If need to escape, take the char

                        // If current char escape, the next char does not
                        need_escape = !need_escape;
                        need_take
                    }
                    else {
                        need_escape = false;
                        str_end = false;
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
                inner: TokenInner::new(res_str, idx),
            },
            _ => Token::Invalid {
                inner: TokenInner::new_invalid(
                    r#"Invalid string token, not end with `"`"#.to_owned(),
                    source.len() - idx,
                    idx,
                ),
            },
        }
    }

    fn parse_number(first: char, chars: &mut PeekNth<CharIndices>, idx: usize) -> Token {
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
            inner:  TokenInner::new(lexeme, idx),
        }
    }

    fn parse_ident(
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
        let inner = TokenInner::new(format!("{ident_start}{lexeme}"), idx);

        Self::keyword_or_ident(inner)
    }

    fn parse_other(source_chars: &mut PeekNth<CharIndices>, other: char, idx: usize) -> Token {
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
                format!("Unknown: {}{}", other, ot),
                count + 1, // add the `other` len
                idx,
            ),
        }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
