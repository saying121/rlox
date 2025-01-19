use std::rc::Rc;

use super::*;
use crate::{
    expr::{Binary, Grouping, Literal, Unary},
    token::{Token, TokenInner},
};

#[test]
fn print_test() {
    let source = Rc::from("-123 * 45.67;");
    let expression: Exprs = Exprs::Binary(Binary::new(
        Exprs::Unary(Unary::new(
            Token::Minus {
                inner: TokenInner::new_minus(Rc::clone(&source), 0),
            },
            Exprs::Literal(Literal::new(crate::expr::LiteralType::Number(123.))),
        )),
        Token::Star {
            inner: TokenInner::new_star(Rc::clone(&source), 5),
        },
        Exprs::Grouping(Grouping::new(Exprs::Literal(Literal::new(
            crate::expr::LiteralType::Number(45.67),
        )))),
    ));
    let stmt = Stmts::Expression(Expression::new(expression));
    let mut asp = AstPrinter;
    let res = asp.print(&[stmt]);
    assert_eq!("(; (* (- 123) (group 45.67)))", res);
}
