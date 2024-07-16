use strum::{Display, EnumString};

pub mod token;
pub mod token_type;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenInner {
    lexeme: String,
    /// start char offset
    offset: usize,
}

impl TokenInner {
    pub fn new(lexeme: String, offset: usize) -> Self {
        Self { lexeme, offset }
    }
    fn get_col(&self, origin: &str) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for ch in origin.chars().take(self.offset) {
            if ch == '\n' {
                line += 1;
                col = 1
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
}

// multiple cursor magic moment
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(EnumString, Display)]
pub enum MyTokenType {
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
    Bang { inner: TokenInner },
    BangEqual { inner: TokenInner },
    Equal { inner: TokenInner },
    EqualEqual { inner: TokenInner },
    Greater { inner: TokenInner },
    GreaterEqual { inner: TokenInner },
    Less { inner: TokenInner },
    LessEqual { inner: TokenInner },

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
