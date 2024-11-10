use std::collections::HashMap;

use crate::{
    expr::*,
    interpreter::Interpreter,
    parser::{ParserError, Result},
    stmt::*,
};

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Resolver<'i> {
    pub interpreter: &'i mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl<'i> Resolver<'i> {
    pub fn new(interpreter: &'i mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        // TODO: judge it empty?
        self.scopes.pop();
    }

    pub fn resolve(&mut self, statements: &[Stmts]) -> Result<()> {
        for stmt in statements {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmts) -> Result<()> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Exprs) -> Result<()> {
        expr.accept(self)
    }

    fn declare(&mut self, name: &crate::tokens::Token) {
        let Some(last) = self.scopes.last_mut()
        else {
            return;
        };
        last.insert(name.inner().lexeme().to_owned(), false);
    }

    fn define(&mut self, name: &crate::tokens::Token) {
        if let Some(last) = self.scopes.last_mut() {
            last.insert(name.inner().lexeme().to_owned(), true);
        }
    }

    fn resolve_local(&mut self, expr: &Exprs, name: &crate::tokens::Token) {
        for (i, ele) in self.scopes.iter().enumerate() {
            if ele.contains_key(name.inner().lexeme()) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, stmt: &Function) -> Result<()> {
        self.begin_scope();

        for ele in &stmt.params {
            self.declare(ele);
            self.define(ele);
        }
        self.resolve(&stmt.body)?;
        self.end_scope();
        Ok(())
    }
}

impl crate::expr::ExprVisitor<Result<()>> for Resolver<'_> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<()> {
        self.resolve_expr(expr.value())?;
        self.resolve_local(&Exprs::Assign(expr.clone()), expr.name());
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<()> {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())
    }

    fn visit_call_expr(&mut self, expr: &Call) -> Result<()> {
        self.resolve_expr(expr.callee())?;
        for ele in expr.arguments() {
            self.resolve_expr(ele)?;
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, expr: &Get) -> Result<()> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<()> {
        self.resolve_expr(expr.expression())
    }

    fn visit_literal_expr(&mut self, _expr: &Literal) -> Result<()> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<()> {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())
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
        self.resolve_expr(expr.right())
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<()> {
        if let Some(last) = self.scopes.last() {
            if last.get(expr.name_str()) == Some(&false) {
                return Err(ParserError::Initialization(expr.name().clone()));
            }
        };

        self.resolve_local(&Exprs::Variable(expr.clone()), expr.name());

        Ok(())
    }
}

impl crate::stmt::StmtVisitor<Result<()>> for Resolver<'_> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<()> {
        self.resolve_expr(stmt.expr())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        self.resolve_expr(stmt.expr())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<()> {
        self.declare(stmt.name());

        self.resolve_expr(stmt.initializer())?;

        self.define(stmt.name());

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<()> {
        self.begin_scope();
        self.resolve(stmt.statements())?;
        self.end_scope();

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<()> {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.then_branch())?;
        if let Some(else_b) = stmt.else_branch() {
            self.resolve_stmt(else_b)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<()> {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.body())
    }

    fn visit_break_stmt(&mut self, stmt: &Break) -> Result<()> {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<()> {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(stmt)?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<()> {
        if let Some(v) = stmt.value() {
            self.resolve_expr(v)?;
        }
        Ok(())
    }
}
