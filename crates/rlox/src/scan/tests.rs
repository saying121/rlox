use std::rc::Rc;

use pretty_assertions::assert_eq;

use crate::{
    scan::scanner::Scanner,
    token::{Token, TokenInner},
};

#[test]
fn test_scan_missing_paren() {
    let source = Rc::from("(");
    let correct = vec![Token::LeftParen {
        inner: TokenInner::new_left_paren(Rc::clone(&source), 0),
    }];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}

#[test]
fn test_scan_string_escape() {
    let source = Rc::from("\"abcd\\\"\\\"\n\t\refg\";");
    let correct = vec![
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), "abcd\"\"\n\t\refg".len(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 16),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("\"abcd\\\"\\\"\nefg\";");
    let correct = vec![
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), "abcd\"\"\nefg".len(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 14),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from(r#""abcd\"\"efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), r#"abcd""efg"#.len(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 13),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from(r#""abcd\"efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), r#"abcd"efg"#.len(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 11),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from(r#""abcd\\\"efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), r#"abcd\"efg"#.len(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 13),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from(r#""abcd\\#efg";"#);
    let correct = vec![
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), r#"abcd\#efg"#.len(), 0),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 12),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}

#[test]
fn test_scan_string() {
    let source = Rc::from(r#"var a = "ab()cdefg";"#);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), "ab()cdefg".len(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 19),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(correct, sc.scan_tokens().collect::<Vec<_>>());

    let source = Rc::from(r#"var a = "abcdefg";"#);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::String {
            inner: TokenInner::new_string(Rc::clone(&source), "abcdefg".len(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 17),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from(r#"var a = "abcdefg;"#);
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Invalid {
            inner: TokenInner::new(Rc::clone(&source), 9, 8),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}

#[test]
fn test_line_col() {
    let source = "\n\n\nvar\n\n";
    let mut sc = Scanner::new(source);
    if let Token::Var { inner } = &sc.scan_tokens().next().unwrap() {
        let a = inner.get_col();
        assert_eq!(a, (4, 1));
    }

    let source = "\n\n\n   var\n\n";
    let mut sc = Scanner::new(source);
    if let Token::Var { inner } = &sc.scan_tokens().next().unwrap() {
        let a = inner.get_col();
        assert_eq!(a, (4, 4));
    }

    let source = "\"\"\"\n\n\n   var\n\n\"";
    let mut sc = Scanner::new(source);
    if let Token::Var { inner } = &sc.scan_tokens().next().unwrap() {
        let a = inner.get_col();
        assert_eq!(a, (4, 4));
    }

    let source = "\n\n\n  data\n\n";
    let mut sc = Scanner::new(source);
    if let Token::Identifier { inner } = &sc.scan_tokens().next().unwrap() {
        let a = inner.get_col();
        assert_eq!(a, (4, 3));
    }
}

#[test]
fn test_maximal_munch() {
    let source = Rc::from("var vara");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "vara".len(), 4),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("class classa");
    let correct = vec![
        Token::Class {
            inner: TokenInner::new_class(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "classa".len(), 6),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}
#[test]
fn other_char() {
    let source = Rc::from("var a =## 1.8;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Invalid {
            inner: TokenInner::new(Rc::clone(&source), 2, 7),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Rc::clone(&source), "1.8".len(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 13),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(correct, sc.scan_tokens().collect::<Vec<_>>());

    let source = Rc::from("var a =#  1.8;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Invalid {
            inner: TokenInner::new(Rc::clone(&source), 1, 7),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Rc::clone(&source), "1.8".len(), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 13),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(correct, sc.scan_tokens().collect::<Vec<_>>());
}

#[test]
fn test_scan_number() {
    let source = Rc::from("var a = 1.8;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Rc::clone(&source), "1.8".len(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 11),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("var a = 1.8.pow(1);");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Number {
            double: 1.8,
            inner: TokenInner::new(Rc::clone(&source), "1.8".len(), 8),
        },
        Token::Dot {
            inner: TokenInner::new_dot(Rc::clone(&source), 11),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "pow".len(), 12),
        },
        Token::LeftParen {
            inner: TokenInner::new_left_paren(Rc::clone(&source), 15),
        },
        Token::Number {
            double: 1.,
            inner: TokenInner::new(Rc::clone(&source), "1".len(), 16),
        },
        Token::RightParen {
            inner: TokenInner::new_right_paren(Rc::clone(&source), 17),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 18),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("var a = 1.0;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Rc::clone(&source), "1.0".len(), 8),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 11),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("var a = 19.;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Number {
            double: 19.0,
            inner: TokenInner::new(Rc::clone(&source), "19".len(), 8),
        },
        Token::Dot {
            inner: TokenInner::new_dot(Rc::clone(&source), 10),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 11),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}

#[test]
fn test_scan_comment() {
    let source = Rc::from("// this is a comment\nvar a = 10");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 21),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4 + 21),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6 + 21),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Rc::clone(&source), "10".len(), 8 + 21),
        },
        // MyTokenType::Semicolon {
        //     inner: TokenInner::new(";".to_owned(), 11),
        // },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("var a = 10 / 4;");
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Rc::clone(&source), "10".len(), 8),
        },
        Token::Slash {
            inner: TokenInner::new_slash(Rc::clone(&source), 11),
        },
        Token::Number {
            double: 4.0,
            inner: TokenInner::new(Rc::clone(&source), "4".len(), 13),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 14),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}

#[test]
fn test_scan_block_comment() {
    let source = Rc::from("/* this is a comment*/\nvar a = 10");
    let offset = 23;
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), offset),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4 + offset),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6 + offset),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Rc::clone(&source), "10".len(), 8 + offset),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("/* this is a comment */\nvar\ta\n=\r10");
    let offset = 24;
    let correct = vec![
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), offset),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 4 + offset),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 6 + offset),
        },
        Token::Number {
            double: 10.0,
            inner: TokenInner::new(Rc::clone(&source), "10".len(), 8 + offset),
        },
    ];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);

    let source = Rc::from("/* this is a comment/\nvar a = 10");
    let correct = vec![Token::Invalid {
        inner: TokenInner::new(Rc::clone(&source), 32, 0),
    }];
    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}

#[test]
fn test_scan_double_token() {
    #[expect(clippy::needless_raw_strings, reason = "need")]
    let source = Rc::from(
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
            inner: TokenInner::new_var(Rc::clone(&source), 0),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "one".len(), 4),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 8),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".len(), 10),
        },
        Token::BangEqual {
            inner: TokenInner::new_bang_equal(Rc::clone(&source), 12),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "b".len(), 15),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 16),
        },
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 18),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "two".len(), 22),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 26),
        },
        Token::Bang {
            inner: TokenInner::new_bang(Rc::clone(&source), 28),
        },
        Token::True {
            inner: TokenInner::new_true(Rc::clone(&source), 30),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 34),
        },
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 36),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "three".len(), 40),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 46),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Rc::clone(&source), "1".len(), 48),
        },
        Token::EqualEqual {
            inner: TokenInner::new_equal_equal(Rc::clone(&source), 50),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Rc::clone(&source), "2".len(), 53),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 54),
        },
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 56),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "four".len(), 60),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 65),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Rc::clone(&source), "1".len(), 67),
        },
        Token::Less {
            inner: TokenInner::new_less(Rc::clone(&source), 69),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Rc::clone(&source), "2".len(), 71),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 72),
        },
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 74),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "five".len(), 78),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 83),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Rc::clone(&source), "1".len(), 85),
        },
        Token::LessEqual {
            inner: TokenInner::new_less_equal(Rc::clone(&source), 87),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Rc::clone(&source), "2".len(), 90),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 91),
        },
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 93),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "six".len(), 97),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 101),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Rc::clone(&source), "1".len(), 103),
        },
        Token::Greater {
            inner: TokenInner::new_greater(Rc::clone(&source), 105),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Rc::clone(&source), "2".len(), 107),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 108),
        },
        Token::Var {
            inner: TokenInner::new_var(Rc::clone(&source), 110),
        },
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "seven".len(), 114),
        },
        Token::Equal {
            inner: TokenInner::new_equal(Rc::clone(&source), 120),
        },
        Token::Number {
            double: 1.0,
            inner: TokenInner::new(Rc::clone(&source), "1".len(), 122),
        },
        Token::GreaterEqual {
            inner: TokenInner::new_greater_equal(Rc::clone(&source), 124),
        },
        Token::Number {
            double: 2.0,
            inner: TokenInner::new(Rc::clone(&source), "2".len(), 127),
        },
        Token::Semicolon {
            inner: TokenInner::new_semicolon(Rc::clone(&source), 128),
        },
    ];

    let mut sc = Scanner::new(&source);
    assert_eq!(sc.scan_tokens().collect::<Vec<_>>(), correct);
}
