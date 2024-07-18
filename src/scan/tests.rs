use pretty_assertions::assert_eq;

use super::scanner::*;
use crate::tokens::{TokenInner, TokenType};

#[test]
fn test_scan_string_escape() {
    let correct = vec![
        TokenType::String {
            inner: TokenInner::new(r#"abcd"efg"#.to_owned(), 0),
        },
        TokenType::Semicolon {
            inner: TokenInner::new(";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new(r#""abcd\"efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        TokenType::String {
            inner: TokenInner::new(r#"abcd\"efg"#.to_owned(), 0),
        },
        TokenType::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(r#""abcd\\\"efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        TokenType::String {
            inner: TokenInner::new(r#"abcd\#efg"#.to_owned(), 0),
        },
        TokenType::Semicolon {
            inner: TokenInner::new(";".to_owned(), 12),
        },
    ];
    let sc = Scanner::new(r#""abcd\\#efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);
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
            inner: TokenInner::new("ab()cdefg".to_owned(), 8),
        },
        TokenType::Semicolon {
            inner: TokenInner::new(";".to_owned(), 19),
        },
    ];
    let sc = Scanner::new(r#"var a = "ab()cdefg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

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
    assert_eq!(sc.tokens(), correct);

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
            inner: TokenInner::new_invalid(
                r#"Invalid string token, not end with `"`"#.to_owned(),
                9,
                8,
            ),
        },
    ];
    let sc = Scanner::new(r#"var a = "abcdefg;"#.to_owned());
    assert_eq!(sc.tokens(), correct);
}

#[test]
fn test_line_col() {
    let source = "\n\n\nvar\n\n";
    let sc = Scanner::new(source.to_owned());
    if let TokenType::Var { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!("[Line: 4, Column: 1], text: var", inner.show(source));
        assert_eq!(a, (4, 1));
    }

    let source = "\n\n\n   var\n\n";
    let sc = Scanner::new(source.to_owned());
    if let TokenType::Var { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!(a, (4, 4));
        assert_eq!("[Line: 4, Column: 4], text: var", inner.show(source));
    }

    let source = "\n\n\n  data\n\n";
    let sc = Scanner::new(source.to_owned());
    if let TokenType::Identifier { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!(a, (4, 3));
        assert_eq!("[Line: 4, Column: 3], text: data", inner.show(source));
    }
}

#[test]
fn test_maximal_munch() {
    let correct = vec![
        TokenType::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        TokenType::Identifier {
            inner: TokenInner::new("vara".to_owned(), 4),
        },
    ];
    let sc = Scanner::new("var vara".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        TokenType::Class {
            inner: TokenInner::new("class".to_owned(), 0),
        },
        TokenType::Identifier {
            inner: TokenInner::new("classa".to_owned(), 6),
        },
    ];
    let sc = Scanner::new("class classa".to_owned());
    assert_eq!(sc.tokens(), correct);
}
#[test]
fn other_char() {
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
            inner: TokenInner::new_invalid("Unknown: ##".to_owned(), 2, 7),
        },
        TokenType::Number {
            double: 1.8,
            inner:  TokenInner::new("1.8".to_owned(), 10),
        },
        TokenType::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new("var a =## 1.8;".to_owned());
    assert_eq!(sc.tokens(), correct);
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
            inner: TokenInner::new_invalid("Unknown: #".to_owned(), 1, 7),
        },
        TokenType::Number {
            double: 1.8,
            inner:  TokenInner::new("1.8".to_owned(), 10),
        },
        TokenType::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new("var a =#  1.8;".to_owned());
    assert_eq!(sc.tokens(), correct);
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
    assert_eq!(sc.tokens(), correct);

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
    assert_eq!(sc.tokens(), correct);

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
    assert_eq!(sc.tokens(), correct);

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
    assert_eq!(sc.tokens(), correct);
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
    assert_eq!(sc.tokens(), correct);

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
    assert_eq!(sc.tokens(), correct);
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
    assert_eq!(sc.tokens(), correct);

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
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        TokenType::Invalid {
            inner: TokenInner::new_invalid(
                "Invalid block comment, not end with `*/`".to_owned(),
                32,
                0,
            ),
        },
        // TokenType::Number {
        //     double: 0.0,
        //     inner:  TokenInner::new("0".to_owned(), 31),
        // },
    ];
    let sc = Scanner::new("/* this is a comment/\nvar a = 10".to_owned());
    assert_eq!(sc.tokens(), correct);
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
    assert_eq!(sc.tokens(), correct);
}
