use std::rc::Rc;

use super::Interpreter;
use crate::{
    expr::{Binary, Exprs, Literal, LiteralType, Unary},
    lox::Lox,
    token::{Token, TokenInner},
};

#[test]
fn test_logic() {
    let mut inter = Interpreter::default();

    let source: Rc<str> = Rc::from("!true");

    let exprs = Exprs::Unary(Unary::new(
        Token::Bang {
            inner: TokenInner::new_bang(source, 0),
        },
        Exprs::Literal(Literal::new(LiteralType::Bool(true))),
    ));

    let res = inter.evaluate(&exprs).unwrap();
    let correct = LiteralType::Bool(false);
    assert_eq!(res, correct);

    let source: Rc<str> = Rc::from("!false");

    let exprs = Exprs::Unary(Unary::new(
        Token::Bang {
            inner: TokenInner::new_bang(source, 0),
        },
        Exprs::Literal(Literal::new(LiteralType::Bool(false))),
    ));

    let res = inter.evaluate(&exprs).unwrap();
    let correct = LiteralType::Bool(true);
    assert_eq!(res, correct);
}

#[test]
fn test_plus_minus_multi_div() {
    let mut inter = Interpreter::default();

    // plus
    let source: Rc<str> = Rc::from("1+1");

    let exprs = Exprs::Binary(Binary::new(
        Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        Token::Plus {
            inner: TokenInner::new_plus(source, 1),
        },
        Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
    ));

    let res = inter.evaluate(&exprs).unwrap();
    let correct = LiteralType::Number(2.0);
    assert_eq!(res, correct);

    // minus
    let source: Rc<str> = Rc::from("1-1");

    let exprs = Exprs::Binary(Binary::new(
        Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        Token::Minus {
            inner: TokenInner::new_minus(source, 1),
        },
        Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
    ));
    let res = inter.evaluate(&exprs).unwrap();
    assert_eq!(res, LiteralType::Number(0.0));

    // multiplication
    let source: Rc<str> = Rc::from("8*2");
    let exprs = Exprs::Binary(Binary::new(
        Exprs::Literal(Literal::new(LiteralType::Number(8.0))),
        Token::Star {
            inner: TokenInner::new_star(source, 1),
        },
        Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
    ));
    let res = inter.evaluate(&exprs).unwrap();
    assert_eq!(res, LiteralType::Number(16.0));

    // div
    let source: Rc<str> = Rc::from("2/3");

    let exprs = Exprs::Binary(Binary::new(
        Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
        Token::Slash {
            inner: TokenInner::new_slash(source, 1),
        },
        Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
    ));
    let res = inter.evaluate(&exprs).unwrap();
    assert_eq!(res, LiteralType::Number(2. / 3.));

    let source: Rc<str> = Rc::from("2/3+ 2/1");
    let exprs = Exprs::Binary(Binary::new(
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 1),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(3.0))),
        )),
        Token::Plus {
            inner: TokenInner::new_plus(Rc::clone(&source), 3),
        },
        Exprs::Binary(Binary::new(
            Exprs::Literal(Literal::new(LiteralType::Number(2.0))),
            Token::Slash {
                inner: TokenInner::new_slash(Rc::clone(&source), 6),
            },
            Exprs::Literal(Literal::new(LiteralType::Number(1.0))),
        )),
    ));
    let a = inter.evaluate(&exprs).unwrap();
    match a {
        LiteralType::Number(v) => {
            assert_eq!(v, 2. / 3. + 2. / 1.);
        },
        _ => unreachable!(),
    }
}

#[test]
fn test_break() {
    let mut lox = Lox::default();
    lox.run(
        "
while (true) {
    break;
}
",
        false,
    )
    .unwrap();
    lox.run(
        "
for (var i = 1; i < 5; i = i + 1) {
    print i;
    if (i > 2) {
        break;
    }
}
",
        false,
    )
    .unwrap();
}

#[test]
fn test_define_del() {
    let mut lox = Lox::default();
    lox.run(
        "
var a = 1;
",
        false,
    )
    .unwrap();
}
