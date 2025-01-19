#![allow(unfulfilled_lint_expectations, reason = "allow it")]

use std::{fmt::Display, rc::Rc};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenInner {
    origin: Rc<str>,
    lexeme: Rc<str>,
    /// start char offset
    offset: usize,
}

impl TokenInner {
    pub fn new_string(origin: Rc<str>, len: usize, offset: usize) -> Self {
        // plus 1 trim start '"'
        let trim = offset + 1;
        let lexeme: Rc<str> = Rc::from(&origin[trim..trim + len]);
        Self {
            origin,
            lexeme,
            offset,
        }
    }

    pub fn new_true(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "true".len(), offset)
    }

    pub fn new_class(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "class".len(), offset)
    }

    pub fn new_var(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "var".len(), offset)
    }

    pub fn new_print(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "print".len(), offset)
    }

    pub fn new_greater_equal(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, ">=".len(), offset)
    }

    pub fn new_greater(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, ">".len(), offset)
    }

    pub fn new_less_equal(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "<=".len(), offset)
    }

    pub fn new_less(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "<".len(), offset)
    }

    pub fn new_equal_equal(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "==".len(), offset)
    }

    pub fn new_equal(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "=".len(), offset)
    }

    pub fn new_bang_equal(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, "!=".len(), offset)
    }

    pub fn new_bang(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '!'.len_utf8(), offset)
    }

    pub fn new_slash(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '/'.len_utf8(), offset)
    }

    pub fn new_star(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '*'.len_utf8(), offset)
    }

    pub fn new_semicolon(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, ';'.len_utf8(), offset)
    }

    pub fn new_plus(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '+'.len_utf8(), offset)
    }

    pub fn new_minus(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '-'.len_utf8(), offset)
    }

    pub fn new_dot(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '.'.len_utf8(), offset)
    }

    pub fn new_comma(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, ','.len_utf8(), offset)
    }

    pub fn new_left_brace(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '{'.len_utf8(), offset)
    }

    pub fn new_right_brace(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '}'.len_utf8(), offset)
    }

    pub fn new_left_paren(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, '('.len_utf8(), offset)
    }

    pub fn new_right_paren(origin: Rc<str>, offset: usize) -> Self {
        Self::new(origin, ')'.len_utf8(), offset)
    }

    pub fn new(origin: Rc<str>, len: usize, offset: usize) -> Self {
        let lexeme: Rc<str> = Rc::from(&origin[offset..offset + len]);
        Self {
            origin,
            lexeme,
            offset,
        }
    }

    pub fn new_invalid(origin: Rc<str>, len: usize, offset: usize) -> Self {
        Self::new(origin, len, offset)
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

    pub fn lexeme_owned(&self) -> String {
        self.lexeme().to_owned()
    }
}

// multiple cursor magic moment
#[derive(Clone)]
#[derive(Debug)]
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

    Break { inner: TokenInner },

    Invalid { inner: TokenInner },
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        macro_rules! hash_variant {
            ($($variant:ident), *,) => {
                match self {
                    $(Self::$variant { inner } => inner.hash(state),)*
                    Self::Number { double, inner } => {
                        double.to_bits().hash(state);
                        inner.hash(state);
                    },
                }

            };
        }
        hash_variant!(
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
            Break,
            Invalid,
        );
    }
}

impl Eq for Token {}

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
    ($($arm:ident,) *) => {

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // #[expect(clippy::enum_glob_use, reason = "just in this block")]
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
        // #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match self {
            $(
                | $arm { inner, .. }
            )*
            => inner,
        }
    }
    pub fn lexeme(&self) -> &str {
        // #[expect(clippy::enum_glob_use, reason = "just in this block")]
        use Token::*;
        match self {
            $(
                | $arm { inner, .. }
            )*
            => inner.lexeme(),
        }
    }
    pub fn into_inner(self) -> TokenInner {
        // #[expect(clippy::enum_glob_use, reason = "just in this block")]
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
    Number,
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
    Break,
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
            While, Break
        )
    }
}
