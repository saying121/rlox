use crate::{expr::*, interpreter::Interpreter, parser::Result, stmt::*};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct Resolver {
    pub interpreter: Interpreter,
}

impl crate::expr::ExprVisitor<Result<()>> for Resolver {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<()> {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<()> {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &Call) -> Result<()> {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &Get) -> Result<()> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<()> {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<()> {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<()> {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &Set) -> Result<()> {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &Super) -> Result<()> {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &This) -> Result<()> {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<()> {
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<()> {
        todo!()
    }
}
impl crate::stmt::StmtVisitor<Result<()>> for Resolver {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<()> {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<()> {
        todo!()
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<()> {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<()> {
        todo!()
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<()> {
        todo!()
    }

    fn visit_break_stmt(&mut self, stmt: &Break) -> Result<()> {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<()> {
        todo!()
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<()> {
        todo!()
    }
}
