#[cfg(test)]
mod tests;

use std::{fmt::Write, string::String};

use crate::{
    expr::{Expr, ExprVisitor, Exprs},
    stmt::{StmtVisitor, Stmts, *},
    token::Token,
};

// #[derive(Debug)]
pub struct AstPrinter;

pub enum Obj<'o> {
    Expr(&'o Exprs),
    Stmt(&'o Stmts),
    Token(&'o Token),
    List(Vec<Self>),
    Str(&'o str),
}

impl AstPrinter {
    pub fn print(&mut self, expr: &[Stmts]) -> String {
        let mut it = vec![];
        for ele in expr {
            it.push(Obj::Stmt(ele));
        }
        let mut builder = String::new();
        self.transform(&mut builder, it.iter());
        builder
        // expr.accept(self)
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

    pub fn parenthesize2<'o, I>(&mut self, name: &str, parts: I) -> String
    where
        I: IntoIterator<Item = &'o Obj<'o>>,
    {
        let mut builder = format!("({name}");
        self.transform(&mut builder, parts);
        builder.push(')');
        builder
    }

    fn transform<'o, I>(&mut self, builder: &mut String, parts: I)
    where
        I: IntoIterator<Item = &'o Obj<'o>>,
    {
        for ele in parts {
            match ele {
                Obj::Expr(exprs) => builder.push_str(&exprs.accept(self)),
                Obj::Stmt(stmts) => builder.push_str(&stmts.accept(self)),
                Obj::Token(token) => builder.push_str(token.lexeme()),
                Obj::List(vec) => self.transform(builder, vec),
                Obj::Str(s) => builder.push_str(s),
            }
        }
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> String {
        self.parenthesize2(
            "=",
            vec![&Obj::Str(expr.name().lexeme()), &Obj::Expr(expr.value())],
        )
    }

    fn visit_binary_expr(&mut self, expr: &crate::expr::Binary) -> String {
        let exprs = [expr.left(), expr.right()];
        self.parenthesize(expr.operator().lexeme(), exprs)
    }

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> String {
        self.parenthesize2(
            "call",
            vec![
                &Obj::Expr(expr.callee()),
                &Obj::List(expr.arguments().iter().map(Obj::Expr).collect()),
            ],
        )
    }

    fn visit_get_expr(&mut self, expr: &crate::expr::Get) -> String {
        self.parenthesize2(
            ".",
            vec![&Obj::Expr(expr.object()), &Obj::Str(expr.name().lexeme())],
        )
    }

    fn visit_grouping_expr(&mut self, expr: &crate::expr::Grouping) -> String {
        self.parenthesize("group", [expr.expression()])
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::Literal) -> String {
        expr.value().to_string()
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::Logical) -> String {
        self.parenthesize(expr.operator().lexeme(), [expr.left(), expr.right()])
    }

    fn visit_set_expr(&mut self, expr: &crate::expr::Set) -> String {
        self.parenthesize2(
            "=",
            [
                &Obj::Expr(expr.object()),
                &Obj::Str(expr.name().lexeme()),
                &Obj::Expr(expr.value()),
            ],
        )
    }

    fn visit_super_expr(&mut self, expr: &crate::expr::Super) -> String {
        self.parenthesize2("super", [&Obj::Token(expr.method())])
    }

    fn visit_this_expr(&mut self, expr: &crate::expr::This) -> String {
        expr.keyword().lexeme().to_owned()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> String {
        self.parenthesize(expr.operator().lexeme(), [expr.right()])
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> String {
        expr.name_str().to_owned()
    }
}

impl StmtVisitor<String> for AstPrinter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> String {
        self.parenthesize(";", [stmt.expr()])
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> String {
        self.parenthesize("print", [stmt.expr()])
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> String {
        if let Some(i) = stmt.initializer() {
            self.parenthesize2(
                "var",
                [&Obj::Token(stmt.name()), &Obj::Str("="), &Obj::Expr(i)],
            )
        }
        else {
            self.parenthesize2("var", [&Obj::Token(stmt.name())])
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> String {
        let mut builder = String::new();
        builder.push_str("(block ");
        for ele in stmt.statements() {
            builder.push_str(&ele.accept(self));
        }
        builder.push(')');
        builder
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> String {
        if let Some(eb) = stmt.else_branch() {
            self.parenthesize2(
                "if-else",
                [
                    &Obj::Expr(stmt.condition()),
                    &Obj::Stmt(stmt.then_branch()),
                    &Obj::Stmt(eb),
                ],
            )
        }
        else {
            self.parenthesize2(
                "if",
                [&Obj::Expr(stmt.condition()), &Obj::Stmt(stmt.then_branch())],
            )
        }
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> String {
        self.parenthesize2(
            "while",
            [&Obj::Expr(stmt.condition()), &Obj::Stmt(stmt.body())],
        )
    }

    fn visit_break_stmt(&mut self, stmt: &Break) -> String {
        stmt.token().lexeme().to_owned()
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> String {
        let mut builder = format!("(fun {} (", stmt.name.lexeme());
        for ele in &stmt.params {
            if ele != &stmt.params[0] {
                builder.push(' ');
            }
            builder.push_str(ele.lexeme());
        }
        builder.push(')');
        builder
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> String {
        stmt.value().map_or_else(
            || "(return)".to_owned(),
            |v| self.parenthesize("return", [v]),
        )
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> String {
        let mut builder = format!("(class {}", stmt.name().lexeme());
        if let Some(superclass) = stmt.superclass() {
            write!(&mut builder, " < {}", superclass.accept(self)).unwrap();
        }
        for ele in stmt.methods() {
            write!(&mut builder, " {}", ele.accept(self)).unwrap();
        }
        builder.push(')');
        builder
    }
}
