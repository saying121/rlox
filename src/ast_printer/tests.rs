use std::rc::Rc;

use super::*;
use crate::{
    expr::{Binary, Grouping, Literal, Unary},
    tokens::{Token, TokenInner},
};

#[test]
fn print_test() {
    let source = Rc::from("");
    let expression: Exprs = Exprs::Binary(Binary::new(
        Exprs::Unary(Unary::new(
            Token::Minus {
                inner: TokenInner::new(Rc::clone(&source), '-'.to_string(), 1),
            },
            Exprs::Literal(Literal::new(crate::expr::LiteralType::Number(123.))),
        )),
        Token::Star {
            inner: TokenInner::new(Rc::clone(&source), '*'.to_string(), 1),
        },
        Exprs::Grouping(Grouping::new(Exprs::Literal(Literal::new(
            crate::expr::LiteralType::Number(45.67),
        )))),
    ));
    let mut asp = AstPrinter;
    let res = asp.print(&expression);
    assert_eq!("(* (- 123) (group 45.67))", res);
}
