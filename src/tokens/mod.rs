#![allow(unfulfilled_lint_expectations, reason = "allow it")]

use std::{fmt::Display, mem, sync::Arc};

use strum::EnumString;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenInner {
    origin: Arc<str>,
    lexeme: String,
    len: usize,
    /// start char offset
    offset: usize,
}

impl TokenInner {
    pub fn new_greater_equal(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: ">=".to_owned(),
            len: 2,
            offset,
        }
    }
    pub fn new_greater(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: ">".to_owned(),
            len: 1,
            offset,
        }
    }
    pub fn new_less_equal(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: "<=".to_owned(),
            len: 2,
            offset,
        }
    }
    pub fn new_less(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: "<".to_owned(),
            len: 1,
            offset,
        }
    }
    pub fn new_equal_equal(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: "==".to_owned(),
            len: 2,
            offset,
        }
    }
    pub fn new_equal(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: "=".to_owned(),
            len: 1,
            offset,
        }
    }
    pub fn new_bang_equal(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: "!=".to_owned(),
            len: 2,
            offset,
        }
    }
    pub fn new_bang(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '!'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_slash(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '/'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_star(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '*'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_semicolon(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: ';'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_plus(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '+'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_minus(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '-'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_dot(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '.'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_comma(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: ','.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_left_brace(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '{'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_right_brace(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '}'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_left_paren(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: '('.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new_right_paren(origin: Arc<str>, offset: usize) -> Self {
        Self {
            origin,
            lexeme: ')'.to_string(),
            len: 1,
            offset,
        }
    }
    pub fn new(origin: Arc<str>, lexeme: String, offset: usize) -> Self {
        let len = lexeme.len();
        Self {
            origin,
            lexeme,
            len,
            offset,
        }
    }
    pub const fn new_invalid(origin: Arc<str>, lexeme: String, len: usize, offset: usize) -> Self {
        Self {
            origin,
            lexeme,
            len,
            offset,
        }
    }
    pub fn get_col(&self) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (_, ch) in self.origin.char_indices().take(self.offset) {
            if ch == '\n' {
                line += 1;
                col = 1;
            }
            else {
                col += 1;
            }
        }
        (line, col)
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn lexeme_take(&mut self) -> String {
        mem::take(&mut self.lexeme)
    }
}

// multiple cursor magic moment
#[derive(Clone)]
#[derive(Debug)]
#[derive(EnumString)]
#[derive(PartialEq, PartialOrd)]
pub enum Token {
    // Single_character tokens
    LeftParen { inner: TokenInner },
    RightParen { inner: TokenInner },
    LeftBrace { inner: TokenInner },
    RightBrace { inner: TokenInner },
    Comma { inner: TokenInner },
    Dot { inner: TokenInner },
    Minus { inner: TokenInner },
    Plus { inner: TokenInner },
    Semicolon { inner: TokenInner },
    Slash { inner: TokenInner },
    Star { inner: TokenInner },

    // One or two character tokens.
    Bang { inner: TokenInner },         // !
    BangEqual { inner: TokenInner },    // !=
    Equal { inner: TokenInner },        // =
    EqualEqual { inner: TokenInner },   // ==
    Greater { inner: TokenInner },      // >
    GreaterEqual { inner: TokenInner }, // >=
    Less { inner: TokenInner },         // <
    LessEqual { inner: TokenInner },    // <=

    // Literals
    Identifier { inner: TokenInner },
    String { inner: TokenInner },
    Number { double: f64, inner: TokenInner },

    // Keywords
    And { inner: TokenInner },
    Class { inner: TokenInner },
    Else { inner: TokenInner },
    Fun { inner: TokenInner },
    For { inner: TokenInner },
    If { inner: TokenInner },
    Nil { inner: TokenInner },
    Or { inner: TokenInner },
    Print { inner: TokenInner },
    Return { inner: TokenInner },
    Super { inner: TokenInner },
    This { inner: TokenInner },
    True { inner: TokenInner },
    False { inner: TokenInner },
    Var { inner: TokenInner },
    While { inner: TokenInner },

    Eof { inner: TokenInner },

    Comment { inner: TokenInner },
    BlockComment { inner: TokenInner },

    Invalid { inner: TokenInner },
}

impl Display for TokenInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, col) = self.get_col();
        f.write_fmt(format_args!(
            "[Line: {line}, Column: {col}], code: `{}`",
            self.lexeme
        ))
    }
}

macro_rules! token_enums_match {
    ($($arm:ident), *) => {

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match self {
            $(
                | $arm { inner, .. }
            )*
            => f.write_str(&inner.to_string()),
        }
    }
}

impl Token {
    pub const fn inner(&self) -> &TokenInner {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match self {
            $(
                | $arm { inner, .. }
            )*
            => inner,
        }
    }
}

    };
}

token_enums_match!(
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
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    And,
    Class,
    Else,
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
    False,
    Var,
    While,
    Eof,
    Comment,
    BlockComment,
    Invalid,
    Number
);

impl Token {
    pub const fn is_keyword(&self) -> bool {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;

        macro_rules! match_arms {
            ($($arm:ident), *) => {
                matches!(
                    self,
                    $(
                    |   $arm { .. }
                    )*
                )
            };
        }

        match_arms!(
            And, Class, Else, Fun, For, If, Nil, Or, Print, Return, Super, This, True, False, Var,
            While
        )
    }
}
