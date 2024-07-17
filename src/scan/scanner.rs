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
                                count += 1;
                            }

                            let b_comment: String =
                                source_chars.by_ref().take(count).map(|(_, c)| c).collect();
                            if last_pre == '*' && last == '/' {
                                source_chars.next();
                                source_chars.next();
                                TokenType::BlockComment {
                                    inner: TokenInner::new(b_comment, idx),
                                }
                            }
                            else {
                                TokenType::Invalid {
                                    inner: TokenInner::new(
                                        "Invalid block comment, not end with `*/`".to_owned(),
                                        idx,
                                    ),
                                }
                            }
                        },
                    ),
                },
                white if white.is_whitespace() => continue,
                // ' ' | '\r' | '\t' | '\n' => continue,
                '"' => {
                    let mut last_matched = '\0';
                    let string = source_chars
                        .by_ref()
                        .take_while(|&(_, c)| {
                            last_matched = c;
                            c != '"'
                        })
                        .map(|(_, c)| c)
                        .collect();
                    match last_matched {
                        '"' => TokenType::String {
                            inner: TokenInner::new(string, idx),
                        },
                        _ => TokenType::Invalid {
                            inner: TokenInner::new(
                                r#"Invalid string token, not end with `"`"#.to_owned(),
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
                ident0 if ident0.is_ascii_alphanumeric() => {
                    let mut count = 0;
                    while let Some(&(_, c)) = source_chars.peek_nth(count)
                        && c.is_ascii_alphanumeric()
                    {
                        count += 1;
                    }
                    let lexeme: String =
                        source_chars.by_ref().take(count).map(|(_, c)| c).collect();
                    let inner = TokenInner::new(format!("{ident0}{lexeme}"), idx);

                    use TokenType::{
                        And, Class, Else, False, For, Fun, Identifier, If, Nil, Or, Print, Return,
                        Super, This, True, Var, While,
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
                },
                other => TokenType::Invalid {
                    inner: TokenInner::new(other.to_string(), idx),
                },
            };

            tokens.push(token);
        }

        Self { source, tokens }
    }

    pub fn tokens(&self) -> &[TokenType] {
        &self.tokens
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_line_col() {
        let source = "\n\n\nvar\n\n";
        let sc = Scanner::new(source.to_owned());
        if let TokenType::Var { inner } = &sc.tokens[0] {
            let a = inner.get_col(source);
            assert_eq!("[Line: 4, Column: 1], text: var", inner.show(source));
            assert_eq!(a, (4, 1));
        }

        let source = "\n\n\n   var\n\n";
        let sc = Scanner::new(source.to_owned());
        if let TokenType::Var { inner } = &sc.tokens[0] {
            let a = inner.get_col(source);
            assert_eq!(a, (4, 4));
            assert_eq!("[Line: 4, Column: 4], text: var", inner.show(source));
        }

        let source = "\n\n\n  data\n\n";
        let sc = Scanner::new(source.to_owned());
        if let TokenType::Identifier { inner } = &sc.tokens[0] {
            let a = inner.get_col(source);
            assert_eq!(a, (4, 3));
            assert_eq!("[Line: 4, Column: 3], text: data", inner.show(source));
        }
    }

    #[test]
    fn test_scan_number() {
        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::Number {
                double: 1.8,
                inner:  TokenInner::new("1.8".to_owned(), 8),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 11),
            },
        ];
        let sc = Scanner::new("var a = 1.8;".to_owned());
        assert_eq!(sc.tokens, correct);

        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::Number {
                double: 1.8,
                inner:  TokenInner::new("1.8".to_owned(), 8),
            },
            TokenType::Dot {
                inner: TokenInner::new(".".to_owned(), 11),
            },
            TokenType::Identifier {
                inner: TokenInner::new("pow".to_owned(), 12),
            },
            TokenType::LeftParen {
                inner: TokenInner::new("(".to_owned(), 15),
            },
            TokenType::Number {
                double: 1.,
                inner:  TokenInner::new("1".to_owned(), 16),
            },
            TokenType::RightParen {
                inner: TokenInner::new(")".to_owned(), 17),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 18),
            },
        ];
        let sc = Scanner::new("var a = 1.8.pow(1);".to_owned());
        assert_eq!(sc.tokens, correct);

        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::Number {
                double: 1.0,
                inner:  TokenInner::new("1.0".to_owned(), 8),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 11),
            },
        ];
        let sc = Scanner::new("var a = 1.0;".to_owned());
        assert_eq!(sc.tokens, correct);

        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::Number {
                double: 19.0,
                inner:  TokenInner::new("19".to_owned(), 8),
            },
            TokenType::Dot {
                inner: TokenInner::new(".".to_owned(), 10),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 11),
            },
        ];
        let sc = Scanner::new("var a = 19.;".to_owned());
        assert_eq!(sc.tokens, correct);
    }

    #[test]
    fn test_scan_string() {
        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::String {
                inner: TokenInner::new("abcdefg".to_owned(), 8),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 17),
            },
        ];
        let sc = Scanner::new(r#"var a = "abcdefg";"#.to_owned());
        assert_eq!(sc.tokens, correct);

        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::Invalid {
                inner: TokenInner::new(r#"Invalid string token, not end with `"`"#.to_owned(), 8),
            },
        ];
        let sc = Scanner::new(r#"var a = "abcdefg;"#.to_owned());
        assert_eq!(sc.tokens, correct);
    }

    #[test]
    fn test_scan_comment() {
        let correct = vec![
            TokenType::Comment {
                inner: TokenInner::new(" this is a comment".to_owned(), 0),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 21),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4 + 21),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6 + 21),
            },
            TokenType::Number {
                double: 10.0,
                inner:  TokenInner::new("10".to_owned(), 8 + 21),
            },
            // MyTokenType::Semicolon {
            //     inner: TokenInner::new(";".to_owned(), 11),
            // },
        ];
        let sc = Scanner::new("// this is a comment\nvar a = 10".to_owned());
        assert_eq!(sc.tokens, correct);

        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6),
            },
            TokenType::Number {
                double: 10.0,
                inner:  TokenInner::new("10".to_owned(), 8),
            },
            TokenType::Slash {
                inner: TokenInner::new("/".to_owned(), 11),
            },
            TokenType::Number {
                double: 4.0,
                inner:  TokenInner::new("4".to_owned(), 13),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 14),
            },
        ];
        let sc = Scanner::new("var a = 10 / 4;".to_owned());
        assert_eq!(sc.tokens, correct);
    }

    #[test]
    fn test_scan_block_comment() {
        let offset = 23;
        let correct = vec![
            TokenType::BlockComment {
                inner: TokenInner::new(" this is a comment".to_owned(), 0),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), offset),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4 + offset),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6 + offset),
            },
            TokenType::Number {
                double: 10.0,
                inner:  TokenInner::new("10".to_owned(), 8 + offset),
            },
        ];
        let sc = Scanner::new("/* this is a comment*/\nvar a = 10".to_owned());
        assert_eq!(sc.tokens, correct);

        let offset = 24;
        let correct = vec![
            TokenType::BlockComment {
                inner: TokenInner::new(" this is a comment ".to_owned(), 0),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), offset),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 4 + offset),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 6 + offset),
            },
            TokenType::Number {
                double: 10.0,
                inner:  TokenInner::new("10".to_owned(), 8 + offset),
            },
        ];
        let sc = Scanner::new("/* this is a comment */\nvar\ta\n=\r10".to_owned());
        assert_eq!(sc.tokens, correct);

        let correct = vec![
            TokenType::Invalid {
                inner: TokenInner::new("Invalid block comment, not end with `*/`".to_owned(), 0),
            },
            TokenType::Number {
                double: 0.0,
                inner:  TokenInner::new("0".to_owned(), 31),
            },
        ];
        let sc = Scanner::new("/* this is a comment/\nvar a = 10".to_owned());
        assert_eq!(sc.tokens, correct);
    }

    #[test]
    fn test_scan_double_token() {
        let correct = vec![
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 0),
            },
            TokenType::Identifier {
                inner: TokenInner::new("one".to_owned(), 4),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 8),
            },
            TokenType::Identifier {
                inner: TokenInner::new("a".to_owned(), 10),
            },
            TokenType::BangEqual {
                inner: TokenInner::new("!=".to_owned(), 12),
            },
            TokenType::Identifier {
                inner: TokenInner::new("b".to_owned(), 15),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 16),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 18),
            },
            TokenType::Identifier {
                inner: TokenInner::new("two".to_owned(), 22),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 26),
            },
            TokenType::Bang {
                inner: TokenInner::new("!".to_owned(), 28),
            },
            TokenType::True {
                inner: TokenInner::new("true".to_owned(), 30),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 34),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 36),
            },
            TokenType::Identifier {
                inner: TokenInner::new("three".to_owned(), 40),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 46),
            },
            TokenType::Number {
                double: 1.0,
                inner:  TokenInner::new("1".to_owned(), 48),
            },
            TokenType::EqualEqual {
                inner: TokenInner::new("==".to_owned(), 50),
            },
            TokenType::Number {
                double: 2.0,
                inner:  TokenInner::new("2".to_owned(), 53),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 54),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 56),
            },
            TokenType::Identifier {
                inner: TokenInner::new("four".to_owned(), 60),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 65),
            },
            TokenType::Number {
                double: 1.0,
                inner:  TokenInner::new("1".to_owned(), 67),
            },
            TokenType::Less {
                inner: TokenInner::new("<".to_owned(), 69),
            },
            TokenType::Number {
                double: 2.0,
                inner:  TokenInner::new("2".to_owned(), 71),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 72),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 74),
            },
            TokenType::Identifier {
                inner: TokenInner::new("five".to_owned(), 78),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 83),
            },
            TokenType::Number {
                double: 1.0,
                inner:  TokenInner::new("1".to_owned(), 85),
            },
            TokenType::LessEqual {
                inner: TokenInner::new("<=".to_owned(), 87),
            },
            TokenType::Number {
                double: 2.0,
                inner:  TokenInner::new("2".to_owned(), 90),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 91),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 93),
            },
            TokenType::Identifier {
                inner: TokenInner::new("six".to_owned(), 97),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 101),
            },
            TokenType::Number {
                double: 1.0,
                inner:  TokenInner::new("1".to_owned(), 103),
            },
            TokenType::Greater {
                inner: TokenInner::new(">".to_owned(), 105),
            },
            TokenType::Number {
                double: 2.0,
                inner:  TokenInner::new("2".to_owned(), 107),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 108),
            },
            TokenType::Var {
                inner: TokenInner::new("var".to_owned(), 110),
            },
            TokenType::Identifier {
                inner: TokenInner::new("seven".to_owned(), 114),
            },
            TokenType::Equal {
                inner: TokenInner::new("=".to_owned(), 120),
            },
            TokenType::Number {
                double: 1.0,
                inner:  TokenInner::new("1".to_owned(), 122),
            },
            TokenType::GreaterEqual {
                inner: TokenInner::new(">=".to_owned(), 124),
            },
            TokenType::Number {
                double: 2.0,
                inner:  TokenInner::new("2".to_owned(), 127),
            },
            TokenType::Semicolon {
                inner: TokenInner::new(";".to_owned(), 128),
            },
        ];

        #[expect(clippy::needless_raw_strings, reason = "need")]
        let sc = Scanner::new(
            r#"var one = a != b;
var two = ! true;
var three = 1 == 2;
var four = 1 < 2;
var five = 1 <= 2;
var six = 1 > 2;
var seven = 1 >= 2;
"#
            .to_owned(),
        );
        assert_eq!(sc.tokens, correct);
    }
}
