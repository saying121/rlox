use std::sync::Arc;

use super::Interpreter;
use crate::{
    expr::{Binary, Exprs, Literal, LiteralType, Unary}, parser::Parser, scan::scanner::Scanner, tokens::{Token, TokenInner}
};

#[test]
fn test_logic() {
    let inter = Interpreter;

    let source: Arc<str> = Arc::from("!true");

    let exprs = Exprs::Unary(Unary {
        operator: Token::Bang {
            inner: TokenInner::new(source, "!".to_owned(), 0),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Bool(true),
        })),
    });

    let res = inter.interpret(&exprs).unwrap();
    let correct = LiteralType::Bool(false);
    assert_eq!(res, correct);

    let source: Arc<str> = Arc::from("!false");

    let exprs = Exprs::Unary(Unary {
        operator: Token::Bang {
            inner: TokenInner::new(source, "!".to_owned(), 0),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Bool(false),
        })),
    });

    let res = inter.interpret(&exprs).unwrap();
    let correct = LiteralType::Bool(true);
    assert_eq!(res, correct);
}

#[test]
fn test_plus_minus_multi_div() {
    let inter = Interpreter;

    // plus
    let source: Arc<str> = Arc::from("1+1");

    let exprs = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::Plus {
            inner: TokenInner::new(source, "+".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });

    let res = inter.interpret(&exprs).unwrap();
    let correct = LiteralType::Number(2.0);
    assert_eq!(res, correct);

    // minus
    let source: Arc<str> = Arc::from("1-1");

    let exprs = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
        operator: Token::Minus {
            inner: TokenInner::new(source, "-".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(1.0),
        })),
    });
    let res = inter.interpret(&exprs).unwrap();
    assert_eq!(res, LiteralType::Number(0.0));

    // multiplication
    let source: Arc<str> = Arc::from("8*2");
    let exprs = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(8.0),
        })),
        operator: Token::Star {
            inner: TokenInner::new(source, "*".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(2.0),
        })),
    });
    let res = inter.interpret(&exprs).unwrap();
    assert_eq!(res, LiteralType::Number(16.0));

    // div
    let source: Arc<str> = Arc::from("2/3");

    let exprs = Exprs::Binary(Binary {
        left: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(2.0),
        })),
        operator: Token::Slash {
            inner: TokenInner::new(source, "/".to_owned(), 1),
        },
        right: Box::new(Exprs::Literal(Literal {
            value: LiteralType::Number(3.0),
        })),
    });
    let res = inter.interpret(&exprs).unwrap();
    assert_eq!(res, LiteralType::Number(2. / 3.));

    let source: Arc<str> = Arc::from("2/3+ 2/1");
    let mut sc = Scanner::new(&source);
    let tks = sc.scan_tokens();
    let mut pars = Parser::new(tks);
    let exprs = pars.parse().unwrap();
    dbg!(&exprs);
    // inter.interpret(&exprs)
    // dbg!(&tks);
}
