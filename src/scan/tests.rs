use pretty_assertions::assert_eq;

use crate::{
    scan::scanner::Scanner,
    tokens::{TokenInner, Token},
};

#[test]
fn test_scan_string_escape() {
    let correct = vec![
        Token::String {
            inner: TokenInner::new("abcd\"\"\n\t\refg".to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 16),
        },
    ];
    let sc = Scanner::new("\"abcd\\\"\\\"\n\t\refg\";".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::String {
            inner: TokenInner::new("abcd\"\"\nefg".to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 14),
        },
    ];
    let sc = Scanner::new("\"abcd\\\"\\\"\nefg\";".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::String {
            inner: TokenInner::new(r#"abcd""efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(r#""abcd\"\"efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::String {
            inner: TokenInner::new(r#"abcd"efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new(r#""abcd\"efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::String {
            inner: TokenInner::new(r#"abcd\"efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(r#""abcd\\\"efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::String {
            inner: TokenInner::new(r#"abcd\#efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 12),
        },
    ];
    let sc = Scanner::new(r#""abcd\\#efg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);
}

#[test]
fn test_scan_string() {
    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::String {
            inner: TokenInner::new("ab()cdefg".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 19),
        },
    ];
    let sc = Scanner::new(r#"var a = "ab()cdefg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::String {
            inner: TokenInner::new("abcdefg".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 17),
        },
    ];
    let sc = Scanner::new(r#"var a = "abcdefg";"#.to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Invalid {
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
    if let Token::Var { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!("[Line: 4, Column: 1], text: var", inner.show(source));
        assert_eq!(a, (4, 1));
    }

    let source = "\n\n\n   var\n\n";
    let sc = Scanner::new(source.to_owned());
    if let Token::Var { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!(a, (4, 4));
        assert_eq!("[Line: 4, Column: 4], text: var", inner.show(source));
    }

    let source = "\"\"\"\n\n\n   var\n\n\"";
    let sc = Scanner::new(source.to_owned());
    if let Token::Var { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!(a, (4, 4));
        assert_eq!("[Line: 4, Column: 4], text: var", inner.show(source));
    }

    let source = "\n\n\n  data\n\n";
    let sc = Scanner::new(source.to_owned());
    if let Token::Identifier { inner } = &sc.tokens()[0] {
        let a = inner.get_col(source);
        assert_eq!(a, (4, 3));
        assert_eq!("[Line: 4, Column: 3], text: data", inner.show(source));
    }
}

#[test]
fn test_maximal_munch() {
    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("vara".to_owned(), 4),
        },
    ];
    let sc = Scanner::new("var vara".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Class {
            inner: TokenInner::new("class".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("classa".to_owned(), 6),
        },
    ];
    let sc = Scanner::new("class classa".to_owned());
    assert_eq!(sc.tokens(), correct);
}
#[test]
fn other_char() {
    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Invalid {
            inner: TokenInner::new_invalid("Unknown: ##".to_owned(), 2, 7),
        },
        Token::Number {
            double: 1.8,
            inner:  TokenInner::new("1.8".to_owned(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new("var a =## 1.8;".to_owned());
    assert_eq!(sc.tokens(), correct);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Invalid {
            inner: TokenInner::new_invalid("Unknown: #".to_owned(), 1, 7),
        },
        Token::Number {
            double: 1.8,
            inner:  TokenInner::new("1.8".to_owned(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new("var a =#  1.8;".to_owned());
    assert_eq!(sc.tokens(), correct);
}

#[test]
fn test_scan_number() {
    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Number {
            double: 1.8,
            inner:  TokenInner::new("1.8".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new("var a = 1.8;".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Number {
            double: 1.8,
            inner:  TokenInner::new("1.8".to_owned(), 8),
        },
        Token::Dot {
            inner: TokenInner::new(".".to_owned(), 11),
        },
        Token::Identifier {
            inner: TokenInner::new("pow".to_owned(), 12),
        },
        Token::LeftParen {
            inner: TokenInner::new("(".to_owned(), 15),
        },
        Token::Number {
            double: 1.,
            inner:  TokenInner::new("1".to_owned(), 16),
        },
        Token::RightParen {
            inner: TokenInner::new(")".to_owned(), 17),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 18),
        },
    ];
    let sc = Scanner::new("var a = 1.8.pow(1);".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Number {
            double: 1.0,
            inner:  TokenInner::new("1.0".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new("var a = 1.0;".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Number {
            double: 19.0,
            inner:  TokenInner::new("19".to_owned(), 8),
        },
        Token::Dot {
            inner: TokenInner::new(".".to_owned(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new("var a = 19.;".to_owned());
    assert_eq!(sc.tokens(), correct);
}

#[test]
fn test_scan_comment() {
    let correct = vec![
        Token::Comment {
            inner: TokenInner::new(" this is a comment".to_owned(), 0),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 21),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4 + 21),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6 + 21),
        },
        Token::Number {
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
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6),
        },
        Token::Number {
            double: 10.0,
            inner:  TokenInner::new("10".to_owned(), 8),
        },
        Token::Slash {
            inner: TokenInner::new("/".to_owned(), 11),
        },
        Token::Number {
            double: 4.0,
            inner:  TokenInner::new("4".to_owned(), 13),
        },
        Token::Semicolon {
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
        Token::BlockComment {
            inner: TokenInner::new(" this is a comment".to_owned(), 0),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), offset),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4 + offset),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6 + offset),
        },
        Token::Number {
            double: 10.0,
            inner:  TokenInner::new("10".to_owned(), 8 + offset),
        },
    ];
    let sc = Scanner::new("/* this is a comment*/\nvar a = 10".to_owned());
    assert_eq!(sc.tokens(), correct);

    let offset = 24;
    let correct = vec![
        Token::BlockComment {
            inner: TokenInner::new(" this is a comment ".to_owned(), 0),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), offset),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 4 + offset),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 6 + offset),
        },
        Token::Number {
            double: 10.0,
            inner:  TokenInner::new("10".to_owned(), 8 + offset),
        },
    ];
    let sc = Scanner::new("/* this is a comment */\nvar\ta\n=\r10".to_owned());
    assert_eq!(sc.tokens(), correct);

    let correct = vec![
        Token::Invalid {
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
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new("one".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 8),
        },
        Token::Identifier {
            inner: TokenInner::new("a".to_owned(), 10),
        },
        Token::BangEqual {
            inner: TokenInner::new("!=".to_owned(), 12),
        },
        Token::Identifier {
            inner: TokenInner::new("b".to_owned(), 15),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 16),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 18),
        },
        Token::Identifier {
            inner: TokenInner::new("two".to_owned(), 22),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 26),
        },
        Token::Bang {
            inner: TokenInner::new("!".to_owned(), 28),
        },
        Token::True {
            inner: TokenInner::new("true".to_owned(), 30),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 34),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 36),
        },
        Token::Identifier {
            inner: TokenInner::new("three".to_owned(), 40),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 46),
        },
        Token::Number {
            double: 1.0,
            inner:  TokenInner::new("1".to_owned(), 48),
        },
        Token::EqualEqual {
            inner: TokenInner::new("==".to_owned(), 50),
        },
        Token::Number {
            double: 2.0,
            inner:  TokenInner::new("2".to_owned(), 53),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 54),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 56),
        },
        Token::Identifier {
            inner: TokenInner::new("four".to_owned(), 60),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 65),
        },
        Token::Number {
            double: 1.0,
            inner:  TokenInner::new("1".to_owned(), 67),
        },
        Token::Less {
            inner: TokenInner::new("<".to_owned(), 69),
        },
        Token::Number {
            double: 2.0,
            inner:  TokenInner::new("2".to_owned(), 71),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 72),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 74),
        },
        Token::Identifier {
            inner: TokenInner::new("five".to_owned(), 78),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 83),
        },
        Token::Number {
            double: 1.0,
            inner:  TokenInner::new("1".to_owned(), 85),
        },
        Token::LessEqual {
            inner: TokenInner::new("<=".to_owned(), 87),
        },
        Token::Number {
            double: 2.0,
            inner:  TokenInner::new("2".to_owned(), 90),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 91),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 93),
        },
        Token::Identifier {
            inner: TokenInner::new("six".to_owned(), 97),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 101),
        },
        Token::Number {
            double: 1.0,
            inner:  TokenInner::new("1".to_owned(), 103),
        },
        Token::Greater {
            inner: TokenInner::new(">".to_owned(), 105),
        },
        Token::Number {
            double: 2.0,
            inner:  TokenInner::new("2".to_owned(), 107),
        },
        Token::Semicolon {
            inner: TokenInner::new(";".to_owned(), 108),
        },
        Token::Var {
            inner: TokenInner::new("var".to_owned(), 110),
        },
        Token::Identifier {
            inner: TokenInner::new("seven".to_owned(), 114),
        },
        Token::Equal {
            inner: TokenInner::new("=".to_owned(), 120),
        },
        Token::Number {
            double: 1.0,
            inner:  TokenInner::new("1".to_owned(), 122),
        },
        Token::GreaterEqual {
            inner: TokenInner::new(">=".to_owned(), 124),
        },
        Token::Number {
            double: 2.0,
            inner:  TokenInner::new("2".to_owned(), 127),
        },
        Token::Semicolon {
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
