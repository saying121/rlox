#![allow(unfulfilled_lint_expectations, reason = "allow it")]

use strum::{Display, EnumString};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenInner {
    lexeme: String,
    len:    usize,
    /// start char offset
    offset: usize,
}

impl TokenInner {
    pub fn new(lexeme: String, offset: usize) -> Self {
        let len = lexeme.len();
        Self {
            lexeme,
            len,
            offset,
        }
    }
    pub const fn new_invalid(lexeme: String, len: usize, offset: usize) -> Self {
        Self {
            lexeme,
            len,
            offset,
        }
    }
    pub fn get_col(&self, origin: &str) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for ch in origin.chars().take(self.offset) {
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
    pub fn show(&self, origin: &str) -> String {
        let (line, col) = self.get_col(origin);
        format!("[Line: {line}, Column: {col}], text: {}", self.lexeme)
    }

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }
}

// multiple cursor magic moment
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(EnumString, Display)]
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

impl Token {
    pub const fn inner(&self) -> &TokenInner {
        match self {
            Self::LeftParen { inner }
            | Self::RightParen { inner }
            | Self::LeftBrace { inner }
            | Self::RightBrace { inner }
            | Self::Comma { inner }
            | Self::Dot { inner }
            | Self::Minus { inner }
            | Self::Plus { inner }
            | Self::Semicolon { inner }
            | Self::Slash { inner }
            | Self::Star { inner }
            | Self::Bang { inner }
            | Self::BangEqual { inner }
            | Self::Equal { inner }
            | Self::EqualEqual { inner }
            | Self::Greater { inner }
            | Self::GreaterEqual { inner }
            | Self::Less { inner }
            | Self::LessEqual { inner }
            | Self::Identifier { inner }
            | Self::String { inner }
            | Self::Number { inner, .. }
            | Self::And { inner }
            | Self::Class { inner }
            | Self::Else { inner }
            | Self::Fun { inner }
            | Self::For { inner }
            | Self::If { inner }
            | Self::Nil { inner }
            | Self::Or { inner }
            | Self::Print { inner }
            | Self::Return { inner }
            | Self::Super { inner }
            | Self::This { inner }
            | Self::True { inner }
            | Self::False { inner }
            | Self::Var { inner }
            | Self::While { inner }
            | Self::Eof { inner }
            | Self::Comment { inner }
            | Self::BlockComment { inner }
            | Self::Invalid { inner } => inner,
        }
    }
}

impl Token {
    pub const fn is_keyword(&self) -> bool {
        #[expect(clippy::enum_glob_use, reason = "just in function")]
        use Token::*;

        match self {
            And { inner }
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
            | While { inner } => {
                _ = inner;
                true
            },
            _ => false,
        }
    }
}
