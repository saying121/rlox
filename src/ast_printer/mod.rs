use std::string::String;

use crate::expr::{Expr, ExprVisitor, Exprs};

// #[derive(Debug)]
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Exprs) -> String {
        expr.accept(self)
        // expr.acc
    }

    fn parenthesize<'exp, I>(&mut self, name: &str, exprs: I) -> String
    where
        I: IntoIterator<Item = &'exp Exprs>,
    {
        let mut res = format!("({name}");
        for ele in exprs {
            res.push(' ');
            res.push_str(&ele.accept(self));
        }
        res.push(')');

        res
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> String {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &crate::expr::Binary) -> String {
        let exprs = [expr.left(), expr.right()];
        self.parenthesize(expr.operator().inner().lexeme(), exprs)
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> String {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &crate::expr::Get) -> String {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expr::Grouping) -> String {
        self.parenthesize("group", [expr.expression()])
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::Literal) -> String {
        expr.value().to_string()
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::Logical) -> String {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &crate::expr::Set) -> String {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &crate::expr::Super) -> String {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &crate::expr::This) -> String {
        "this".to_owned()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> String {
        self.parenthesize(expr.operator().inner().lexeme(), [expr.right()])
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> String {
        expr.name_str().to_owned()
    }
}

#[cfg(test)]
mod tests {
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
}
