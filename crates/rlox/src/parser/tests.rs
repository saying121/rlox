use std::rc::Rc;

use pretty_assertions::assert_eq;

use super::Parser;
use crate::{
    expr::{Binary, Exprs, Literal, LiteralType, Unary},
    scan::scanner,
    stmt::{Print, Stmts, Var},
    token::{Token, TokenInner},
};

#[test]
fn test_equal() {
    let source: Rc<str> = Rc::from("var a = 1==1;");
    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::EqualEqual {
                inner: TokenInner::new(Rc::clone(&source), "==".to_owned(), 9),
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
    let source: Rc<str> = Rc::from("print 6/3+16/2;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(6.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Plus {
            inner: TokenInner::new_plus(Rc::clone(&source), 9),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(16.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 12),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
        )),
    ))))];
    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    let source: Rc<str> = Rc::from("print 6/3-16*2;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(6.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Minus {
            inner: TokenInner::new_minus(Rc::clone(&source), 9),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(16.0))),
            Token::Star {
                inner: TokenInner::new_star(Rc::clone(&source), 12),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
        )),
    ))))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    let source: Rc<str> = Rc::from("print 6/3-16*-2;");
    let right = vec![Stmts::Print(Print::new(Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(6.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Minus {
            inner: TokenInner::new_minus(Rc::clone(&source), 9),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(16.0))),
            Token::Star {
                inner: TokenInner::new_star(Rc::clone(&source), 12),
            },
            Exprs::Unary(Unary::new(
                Token::Minus {
                    inner: TokenInner::new_minus(Rc::clone(&source), 13),
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
    let source: Rc<str> = Rc::from("var a=1+1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Plus {
                inner: TokenInner::new_plus(Rc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (exprs, _) = p.parse();
    assert_eq!(right, exprs);

    // plus strings
    let source: Rc<str> = Rc::from(r#"var a="ab"+"cd";"#);

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::String("ab".to_owned()))),
            Token::Plus {
                inner: TokenInner::new_plus(Rc::clone(&source), 10),
            },
            Exprs::Literal(Literal::new(LiteralType::String("cd".to_owned()))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    // minus
    let source: Rc<str> = Rc::from("var a=1-1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Minus {
                inner: TokenInner::new_minus(Rc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    // multiplication
    let source: Rc<str> = Rc::from("var a=1*1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Star {
                inner: TokenInner::new_star(Rc::clone(&source), 7),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ))];

    let mut p = Parser::new(tks);
    let (stmts, _) = p.parse();
    assert_eq!(right, stmts);

    // div
    let source: Rc<str> = Rc::from("var a=1/1;");

    let mut scan = scanner::Scanner::new(&source);
    let tks = scan.scan_tokens();

    let right = vec![Stmts::Var(Var::new(
        Token::Identifier {
            inner: TokenInner::new(Rc::clone(&source), "a".to_owned(), 4),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 7),
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
    let source: Rc<str> = Rc::from("(");

    let tks = vec![Token::LeftParen {
        inner: TokenInner::new(Rc::clone(&source), "(".to_owned(), 0),
    }];

    let mut p = Parser::new(tks);
    let (_, had_err) = p.parse();
    assert!(had_err);
}
