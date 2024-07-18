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
                left_paren @ '(' => TokenType::LeftParen {
                    inner: TokenInner::new(left_paren.to_string(), idx),
                },
                right_paren @ ')' => TokenType::RightParen {
                    inner: TokenInner::new(right_paren.to_string(), idx),
                },
                left_brace @ '{' => TokenType::LeftBrace {
                    inner: TokenInner::new(left_brace.to_string(), idx),
                },
                right_barce @ '}' => TokenType::RightBrace {
                    inner: TokenInner::new(right_barce.to_string(), idx),
                },
                comma @ ',' => TokenType::Comma {
                    inner: TokenInner::new(comma.to_string(), idx),
                },
                dot @ '.' => TokenType::Dot {
                    inner: TokenInner::new(dot.to_string(), idx),
                },
                minus @ '-' => TokenType::Minus {
                    inner: TokenInner::new(minus.to_string(), idx),
                },
                plus @ '+' => TokenType::Plus {
                    inner: TokenInner::new(plus.to_string(), idx),
                },
                semicollon @ ';' => TokenType::Semicolon {
                    inner: TokenInner::new(semicollon.to_string(), idx),
                },
                star @ '*' => TokenType::Star {
                    inner: TokenInner::new(star.to_string(), idx),
                },
                // > two-char-tokens
                bang @ '!' => source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
                    || TokenType::Bang {
                        inner: TokenInner::new(bang.to_string(), idx),
                    },
                    |_eq| TokenType::BangEqual {
                        inner: TokenInner::new("!=".to_owned(), idx),
                    },
                ),
                eq @ '=' => source_chars.next_if_eq(&(idx + 1, eq)).map_or_else(
                    || TokenType::Equal {
                        inner: TokenInner::new(eq.to_string(), idx),
                    },
                    |_eq| TokenType::EqualEqual {
                        inner: TokenInner::new("==".to_owned(), idx),
                    },
                ),
                less @ '<' => source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
                    || TokenType::Less {
                        inner: TokenInner::new(less.to_string(), idx),
                    },
                    |_eq| TokenType::LessEqual {
                        inner: TokenInner::new("<=".to_owned(), idx),
                    },
                ),
                greater @ '>' => source_chars.next_if_eq(&(idx + 1, '=')).map_or_else(
                    || TokenType::Greater {
                        inner: TokenInner::new(greater.to_string(), idx),
                    },
                    |_eq| TokenType::GreaterEqual {
                        inner: TokenInner::new(">=".to_owned(), idx),
                    },
                ),
                slash @ '/' => match source_chars.next_if_eq(&(idx + 1, '/')) {
                    Some(_next) => {
                        let comment = source_chars
                            .by_ref()
                            .take_while(|&(_, c)| c != '\n')
                            .map(|(_, c)| c)
                            .collect();
                        TokenType::Comment {
                            inner: TokenInner::new(comment, idx),
                        }
                    },
                    None => source_chars.next_if_eq(&(idx + 1, '*')).map_or_else(
                        || TokenType::Slash {
                            inner: TokenInner::new(slash.to_string(), idx),
                        },
                        |_next| {
                            let (mut last_pre, mut last) = ('\0', '\0');

                            let mut count = 0;
                            while let Some(&(_, next)) = source_chars.peek_nth(count)
                                && let Some(&(_, next_next)) = source_chars.peek_nth(count + 1)
                            {
                                (last_pre, last) = (next, next_next);
                                if next == '*' && next_next == '/' {
                                    break;
                                }
                                // Count the number of character before "*/"
                                count += 1;
                            }

                            let b_comment: String =
                                source_chars.by_ref().take(count).map(|(_, c)| c).collect();

                            // consume the next two characters regardless, even not ('*','/')
                            source_chars.next();
                            source_chars.next();

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
                },
                white if white.is_whitespace() => continue,
                '"' => {
                    let mut res_str = String::new();
                    let mut last_matched = '\0';
                    let mut need_escape = false;

                    loop {
                        let mut end_flag = false;

                        let string: String = source_chars
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
                        if last_matched == '"' && end_flag || source_chars.peek().is_none() {
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
                },
                digit if digit.is_ascii_digit() => {
                    let mut its = Vec::with_capacity(4);
                    its.push(digit.to_string());

                    let mut count = 0;
                    while let Some(&(_, ch)) = source_chars.peek_nth(count)
                        && ch.is_ascii_digit()
                    {
                        count += 1;
                    }

                    let dig_integer = source_chars.by_ref().take(count).map(|(_, c)| c).collect();

                    // `take_while_ref` clone full iterator, so expensive
                    // let dig_integer = sour_chars
                    //     .take_while_ref(|&(_, ch)| ch.is_ascii_digit())
                    //     .map(|(_, c)| c)
                    //     .collect();

                    its.push(dig_integer);

                    if let Some(&(_, next)) = source_chars.peek_nth(0)
                        && let Some(&(_, next_next)) = source_chars.peek_nth(1)
                        && next == '.'
                        && next_next.is_ascii_digit()
                    {
                        #[expect(clippy::unwrap_used, reason = "it must be `Some`")]
                        let (_, dot) = source_chars.next().unwrap();
                        its.push(dot.to_string());

                        let mut count = 0;

                        while let Some(&(_, ch)) = source_chars.peek_nth(count)
                            && ch.is_ascii_digit()
                        {
                            count += 1;
                        }

                        let small = source_chars.by_ref().take(count).map(|(_, c)| c).collect();
                        its.push(small);
                    }

                    let lexeme = its.join("");
                    TokenType::Number {
                        double: lexeme.parse().expect("parse double failed"),
                        inner:  TokenInner::new(lexeme, idx),
                    }
                },
                ident_start if ident_start.is_ascii_alphanumeric() => {
                    let mut count = 0;
                    while let Some(&(_, c)) = source_chars.peek_nth(count)
                        && c.is_ascii_alphanumeric()
                    {
                        count += 1;
                    }
                    let lexeme: String =
                        source_chars.by_ref().take(count).map(|(_, c)| c).collect();
                    let inner = TokenInner::new(format!("{ident_start}{lexeme}"), idx);

                    Self::keyword_or_ident(inner)
                },
                other => {
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
                },
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

    pub fn tokens(&self) -> &[TokenType] {
        &self.tokens
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}
