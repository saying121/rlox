use std::sync::Arc;

use pretty_assertions::assert_eq;

use crate::{
    scan::scanner::Scanner,
    tokens::{Token, TokenInner},
};

#[test]
fn test_scan_missing_paren() {
    let source = Arc::from("(");
    let correct = vec![Token::LeftParen {
        inner: TokenInner::new(Arc::clone(&source), "(".to_owned(), 0),
    }];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_scan_string_escape() {
    let source = Arc::from("\"abcd\\\"\\\"\n\t\refg\";");
    let correct = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), "abcd\"\"\n\t\refg".to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 16),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("\"abcd\\\"\\\"\nefg\";");
    let correct = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), "abcd\"\"\nefg".to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 14),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from(r#""abcd\"\"efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), r#"abcd""efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from(r#""abcd\"efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), r#"abcd"efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from(r#""abcd\\\"efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), r#"abcd\"efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from(r#""abcd\\#efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), r#"abcd\#efg"#.to_owned(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 12),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_scan_string() {
    let source = Arc::from(r#"var a = "ab()cdefg";"#);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), "ab()cdefg".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 19),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from(r#"var a = "abcdefg";"#);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::String {
            inner: TokenInner::new(Arc::clone(&source), "abcdefg".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 17),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from(r#"var a = "abcdefg;"#);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Invalid {
            inner: TokenInner::new_invalid(
                Arc::clone(&source),
                r#"Invalid string token, not end with `"`"#.to_owned(),
                9,
                8,
            ),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_line_col() {
    let source = "\n\n\nvar\n\n";
    let sc = Scanner::new(source.to_owned());
    if let Token::Var { inner } = &sc.scan_tokens()[0] {
        let a = inner.get_col();
        assert_eq!(a, (4, 1));
    }

    let source = "\n\n\n   var\n\n";
    let sc = Scanner::new(source.to_owned());
    if let Token::Var { inner } = &sc.scan_tokens()[0] {
        let a = inner.get_col();
        assert_eq!(a, (4, 4));
    }

    let source = "\"\"\"\n\n\n   var\n\n\"";
    let sc = Scanner::new(source.to_owned());
    if let Token::Var { inner } = &sc.scan_tokens()[0] {
        let a = inner.get_col();
        assert_eq!(a, (4, 4));
    }

    let source = "\n\n\n  data\n\n";
    let sc = Scanner::new(source.to_owned());
    if let Token::Identifier { inner } = &sc.scan_tokens()[0] {
        let a = inner.get_col();
        assert_eq!(a, (4, 3));
    }
}

#[test]
fn test_maximal_munch() {
    let source = Arc::from("var vara");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "vara".to_owned(), 4),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("class classa");
    let correct = vec![
        Token::Class {
            inner: TokenInner::new(Arc::clone(&source), "class".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "classa".to_owned(), 6),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}
#[test]
fn other_char() {
    let source = Arc::from("var a =## 1.8;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Invalid {
            inner: TokenInner::new_invalid(Arc::clone(&source), "Unknown: ##".to_owned(), 2, 7),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Arc::clone(&source), "1.8".to_owned(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("var a =#  1.8;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Invalid {
            inner: TokenInner::new_invalid(Arc::clone(&source), "Unknown: #".to_owned(), 1, 7),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Arc::clone(&source), "1.8".to_owned(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 13),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_scan_number() {
    let source = Arc::from("var a = 1.8;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Arc::clone(&source), "1.8".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("var a = 1.8.pow(1);");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Arc::clone(&source), "1.8".to_owned(), 8),
        },
        Token::Dot {
            inner: TokenInner::new(Arc::clone(&source), ".".to_owned(), 11),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "pow".to_owned(), 12),
        },
        Token::LeftParen {
            inner: TokenInner::new(Arc::clone(&source), "(".to_owned(), 15),
        },
        Token::Number {
            double: 1.,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 16),
        },
        Token::RightParen {
            inner: TokenInner::new(Arc::clone(&source), ")".to_owned(), 17),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 18),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("var a = 1.0;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1.0".to_owned(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("var a = 19.;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Number {
            double: 19.0,
            inner: TokenInner::new(Arc::clone(&source), "19".to_owned(), 8),
        },
        Token::Dot {
            inner: TokenInner::new(Arc::clone(&source), ".".to_owned(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 11),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_scan_comment() {
    let source = Arc::from("// this is a comment\nvar a = 10");
    let correct = vec![
        Token::Comment {
            inner: TokenInner::new(Arc::clone(&source), " this is a comment".to_owned(), 0),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 21),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4 + 21),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6 + 21),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Arc::clone(&source), "10".to_owned(), 8 + 21),
        },
        // MyTokenType::Semicolon {
        //     inner: TokenInner::new(";".to_owned(), 11),
        // },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("var a = 10 / 4;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Arc::clone(&source), "10".to_owned(), 8),
        },
        Token::Slash {
            inner: TokenInner::new(Arc::clone(&source), "/".to_owned(), 11),
        },
        Token::Number {
            double: 4.0,
            inner: TokenInner::new(Arc::clone(&source), "4".to_owned(), 13),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 14),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_scan_block_comment() {
    let source = Arc::from("/* this is a comment*/\nvar a = 10");
    let offset = 23;
    let correct = vec![
        Token::BlockComment {
            inner: TokenInner::new(Arc::clone(&source), " this is a comment".to_owned(), 0),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), offset),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4 + offset),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6 + offset),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Arc::clone(&source), "10".to_owned(), 8 + offset),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("/* this is a comment */\nvar\ta\n=\r10");
    let offset = 24;
    let correct = vec![
        Token::BlockComment {
            inner: TokenInner::new(Arc::clone(&source), " this is a comment ".to_owned(), 0),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), offset),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4 + offset),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 6 + offset),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Arc::clone(&source), "10".to_owned(), 8 + offset),
        },
    ];
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);

    let source = Arc::from("/* this is a comment/\nvar a = 10");
    let correct = vec![
        Token::Invalid {
            inner: TokenInner::new_invalid(
                Arc::clone(&source),
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
    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}

#[test]
fn test_scan_double_token() {
    #[expect(clippy::needless_raw_strings, reason = "need")]
    let source = Arc::from(
        r#"var one = a != b;
var two = ! true;
var three = 1 == 2;
var four = 1 < 2;
var five = 1 <= 2;
var six = 1 > 2;
var seven = 1 >= 2;
"#,
    );
    let correct = vec![
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "one".to_owned(), 4),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 8),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 10),
        },
        Token::BangEqual {
            inner: TokenInner::new(Arc::clone(&source), "!=".to_owned(), 12),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "b".to_owned(), 15),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 16),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 18),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "two".to_owned(), 22),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 26),
        },
        Token::Bang {
            inner: TokenInner::new(Arc::clone(&source), "!".to_owned(), 28),
        },
        Token::True {
            inner: TokenInner::new(Arc::clone(&source), "true".to_owned(), 30),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 34),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 36),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "three".to_owned(), 40),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 46),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 48),
        },
        Token::EqualEqual {
            inner: TokenInner::new(Arc::clone(&source), "==".to_owned(), 50),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 53),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 54),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 56),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "four".to_owned(), 60),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 65),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 67),
        },
        Token::Less {
            inner: TokenInner::new(Arc::clone(&source), "<".to_owned(), 69),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 71),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 72),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 74),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "five".to_owned(), 78),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 83),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 85),
        },
        Token::LessEqual {
            inner: TokenInner::new(Arc::clone(&source), "<=".to_owned(), 87),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 90),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 91),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 93),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "six".to_owned(), 97),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 101),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 103),
        },
        Token::Greater {
            inner: TokenInner::new(Arc::clone(&source), ">".to_owned(), 105),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 107),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 108),
        },
        Token::Var {
            inner: TokenInner::new(Arc::clone(&source), "var".to_owned(), 110),
        },
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "seven".to_owned(), 114),
        },
        Token::Equal {
            inner: TokenInner::new(Arc::clone(&source), "=".to_owned(), 120),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Arc::clone(&source), "1".to_owned(), 122),
        },
        Token::GreaterEqual {
            inner: TokenInner::new(Arc::clone(&source), ">=".to_owned(), 124),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Arc::clone(&source), "2".to_owned(), 127),
        },
        Token::Semicolon {
            inner: TokenInner::new(Arc::clone(&source), ";".to_owned(), 128),
        },
    ];

    let sc = Scanner::new(source.to_string());
    assert_eq!(sc.scan_tokens(), correct);
}
