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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match self {
            LeftParen { inner }
            | RightParen { inner }
            | LeftBrace { inner }
            | RightBrace { inner }
            | Comma { inner }
            | Dot { inner }
            | Minus { inner }
            | Plus { inner }
            | Semicolon { inner }
            | Slash { inner }
            | Star { inner }
            | Bang { inner }
            | BangEqual { inner }
            | Equal { inner }
            | EqualEqual { inner }
            | Greater { inner }
            | GreaterEqual { inner }
            | Less { inner }
            | LessEqual { inner }
            | Identifier { inner }
            | String { inner }
            | And { inner }
            | Class { inner }
            | Else { inner }
            | Fun { inner }
            | For { inner }
            | If { inner }
            | Nil { inner }
            | Or { inner }
            | Print { inner }
            | Return { inner }
            | Super { inner }
            | This { inner }
            | True { inner }
            | False { inner }
            | Var { inner }
            | While { inner }
            | Eof { inner }
            | Comment { inner }
            | BlockComment { inner }
            | Invalid { inner }
            | Number { inner, .. } => f.write_str(&inner.to_string()),
        }
    }
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

impl Token {
    pub const fn inner(&self) -> &TokenInner {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match self {
            LeftParen { inner }
            | RightParen { inner }
            | LeftBrace { inner }
            | RightBrace { inner }
            | Comma { inner }
            | Dot { inner }
            | Minus { inner }
            | Plus { inner }
            | Semicolon { inner }
            | Slash { inner }
            | Star { inner }
            | Bang { inner }
            | BangEqual { inner }
            | Equal { inner }
            | EqualEqual { inner }
            | Greater { inner }
            | GreaterEqual { inner }
            | Less { inner }
            | LessEqual { inner }
            | Identifier { inner }
            | String { inner }
            | And { inner }
            | Class { inner }
            | Else { inner }
            | Fun { inner }
            | For { inner }
            | If { inner }
            | Nil { inner }
            | Or { inner }
            | Print { inner }
            | Return { inner }
            | Super { inner }
            | This { inner }
            | True { inner }
            | False { inner }
            | Var { inner }
            | While { inner }
            | Eof { inner }
            | Comment { inner }
            | BlockComment { inner }
            | Invalid { inner }
            | Number { inner, .. } => inner,
        }
    }
}

impl Token {
    pub const fn is_keyword(&self) -> bool {
        #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;

        matches!(
            self,
            And { .. }
                | Class { .. }
                | Else { .. }
                | Fun { .. }
                | For { .. }
                | If { .. }
                | Nil { .. }
                | Or { .. }
                | Print { .. }
                | Return { .. }
                | Super { .. }
                | This { .. }
                | True { .. }
                | False { .. }
                | Var { .. }
                | While { .. }
        )
    }
}
