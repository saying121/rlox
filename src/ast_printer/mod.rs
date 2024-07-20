use std::string::String;

use crate::expr::{Expr, Exprs, Visitor};

// #[derive(Debug)]
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Exprs) -> String {
        expr.accept(self)
        // expr.acc
    }

    fn parenthesize<'exp, I>(&self, name: &str, exprs: I) -> String
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

impl Visitor<String> for AstPrinter {
    fn visit_assign_expr(&self, expr: &crate::expr::Assign) -> String {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &crate::expr::Binary) -> String {
        let exprs = [&*expr.left, &*expr.right];
        self.parenthesize(expr.operator.inner().lexeme(), exprs)
    }

    fn visit_call_expr(&self, expr: &crate::expr::Call) -> String {
        todo!()
    }

    fn visit_get_expr(&self, expr: &crate::expr::Get) -> String {
        todo!()
    }

    fn visit_grouping_expr(&self, expr: &crate::expr::Grouping) -> String {
        self.parenthesize("group", [&*expr.expression])
    }

    fn visit_literal_expr(&self, expr: &crate::expr::Literal) -> String {
        expr.value.to_string()
    }

    fn visit_logical_expr(&self, expr: &crate::expr::Logical) -> String {
        todo!()
    }

    fn visit_set_expr(&self, expr: &crate::expr::Set) -> String {
        todo!()
    }

    fn visit_super_expr(&self, expr: &crate::expr::Super) -> String {
        todo!()
    }

    fn visit_this_expr(&self, expr: &crate::expr::This) -> String {
        "this".to_owned()
    }

    fn visit_unary_expr(&self, expr: &crate::expr::Unary) -> String {
        self.parenthesize(expr.operator.inner().lexeme(), [&*expr.right])
    }

    fn visit_variable_expr(&self, expr: &crate::expr::Variable) -> String {
        expr.name.inner().lexeme().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{Binary, Grouping, Literal, Unary},
        tokens::{Token, TokenInner},
    };

    #[test]
    fn print_test() {
        let expression: Exprs = Binary::new(
            Unary::new(
                Token::Minus {
                    inner: TokenInner::new('-'.to_string(), 1),
                },
                Literal::new(123).into(),
            )
            .into(),
            Token::Star {
                inner: TokenInner::new('*'.to_string(), 1),
            },
            Grouping::new(Literal::new(45.67).into()).into(),
        )
        .into();
        let asp = AstPrinter;
        let res = asp.print(&expression);
        assert_eq!("(* (- 123) (group 45.67))", res);
    }
}
