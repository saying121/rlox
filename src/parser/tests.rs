use std::sync::Arc;

use pretty_assertions::assert_eq;

use super::Parser;
use crate::{
    expr::{Binary, Exprs, Literal, LiteralType, Unary},
    scan::scanner,
    stmt::{Print, Stmts, Var},
    tokens::{Token, TokenInner},
};

#[test]
fn test_equal() {
    let source: Arc<str> = Arc::from("var a = 1==1;");
    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::EqualEqual {
                inner: TokenInner::new(Arc::clone(&source), "==".to_owned(), 9),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);
}

#[test]
fn test_precedence() {
    let source: Arc<str> = Arc::from("print 6/3+16/2;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(6.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Plus {
            inner: TokenInner::new_plus(Arc::clone(&source), 9),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(16.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 12),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
        )),
    ))))];
    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    let source: Arc<str> = Arc::from("print 6/3-16*2;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(6.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Minus {
            inner: TokenInner::new_minus(Arc::clone(&source), 9),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(16.0))),
            Token::Star {
                inner: TokenInner::new_star(Arc::clone(&source), 12),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
        )),
    ))))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    let source: Arc<str> = Arc::from("print 6/3-16*-2;");
    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(6.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Minus {
            inner: TokenInner::new_minus(Arc::clone(&source), 9),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(16.0))),
            Token::Star {
                inner: TokenInner::new_star(Arc::clone(&source), 12),
            },
            Exprs::Unary(Unary::new (
                Token::Minus {
                    inner: TokenInner::new_minus(Arc::clone(&source), 13),
                },
                Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
            )),
        )),
    ))))];

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let mut p = Parser::new(tks);
    let (exprs, _) = p.parse();
    assert_eq!(right, exprs);
}

#[test]
fn test_plus_minus_multi_div() {
    // plus
    let source: Arc<str> = Arc::from("var a=1+1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Plus {
                inner: TokenInner::new_plus(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (exprs, _) = p.parse();
    assert_eq!(right, exprs);

    // plus strings
    let source: Arc<str> = Arc::from(r#"var a="ab"+"cd";"#);

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::String("ab".to_owned()))),
            Token::Plus {
                inner: TokenInner::new_plus(Arc::clone(&source), 10),
            },
            Exprs::Literal(Literal::new(LiteralType::String("cd".to_owned()))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    // minus
    let source: Arc<str> = Arc::from("var a=1-1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Minus {
                inner: TokenInner::new_minus(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    // multiplication
    let source: Arc<str> = Arc::from("var a=1*1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Star {
                inner: TokenInner::new_star(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    // div
    let source: Arc<str> = Arc::from("var a=1/1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Arc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Arc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);
}

#[test]
fn test_paren() {
    let source: Arc<str> = Arc::from("(");

    let tks = vec![Token::LeftParen {
        inner: TokenInner::new(Arc::clone(&source), "(".to_owned(), 0),
    }];

    let mut p = Parser::new(tks);
    let (_, had_err) = p.parse();
    assert!(had_err);
}
