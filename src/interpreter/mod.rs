#![allow(unfulfilled_lint_expectations, reason = "allow it")]

#[cfg(test)]
mod test;

use crate::{
    env::Environment,
    expr::{Expr, ExprVisitor, Exprs, LiteralType},
    stmt::{Expression, Print, Stmt, StmtVisitor, Stmts, Var},
    tokens::{Token, TokenInner},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(thiserror::Error)]
pub enum InterError {
    #[error("{0}\nhelp: Operand must be numbers")]
    Number(TokenInner),
    #[error("{0}\nhelp: Operand must be a number")]
    UnaryNumber(TokenInner),
    #[error("{0}\nhelp: Operand must be two number or two strings")]
    Plus(TokenInner),
    #[error("{0}")]
    NotMatch(String),
    #[error("Not exist variable: {0}")]
    NoVar(Token),
}

pub type Result<T, E = InterError> = core::result::Result<T, E>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn interpret(&mut self, exprs: &mut [Stmts]) -> Result<()> {
        for ele in exprs {
            self.execute(ele)?;
        }
        Ok(())
    }

    pub fn evaluate(&self, expr: &Exprs) -> Result<LiteralType> {
        expr.accept(self)
    }

    #[expect(clippy::trivially_copy_pass_by_ref, reason = "method")]
    fn execute(&mut self, stmt: &mut Stmts) -> Result<()> {
        stmt.accept(self)
    }

    const fn is_truthy(literal: &LiteralType) -> bool {
        match literal {
            LiteralType::Nil => false,
            LiteralType::Bool(v) => *v,
            _ => true,
        }
    }

    fn is_equal(a: &LiteralType, b: &LiteralType) -> bool {
        a == b
    }
}

impl StmtVisitor<Result<()>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<()> {
        self.evaluate(stmt.expr())?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        let v = self.evaluate(stmt.expr())?;
        println!("{v}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<()> {
        let value = self.evaluate(stmt.initializer())?;
        self.environment.define(stmt.var_name().to_owned(), value);
        Ok(())
    }
}

impl ExprVisitor<Result<LiteralType>> for Interpreter {
    fn visit_assign_expr(&self, expr: &crate::expr::Assign) -> Result<LiteralType> {
        todo!()
    }

    fn visit_binary_expr(&self, expr: &crate::expr::Binary) -> Result<LiteralType> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match &expr.operator {
            Token::Plus { inner } => match (left, right) {
                (LiteralType::Number(left), LiteralType::Number(right)) => {
                    let var_name = left + right;
                    Ok(LiteralType::Number(var_name))
                },
                (LiteralType::String(left), LiteralType::String(right)) => {
                    let var_name = left + &right;
                    Ok(LiteralType::String(var_name))
                },
                _ => Err(InterError::Plus(inner.clone())),
            },
            Token::Minus { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left - right;
                    return Ok(LiteralType::Number(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::Slash { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left / right;
                    return Ok(LiteralType::Number(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::Star { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left * right;
                    return Ok(LiteralType::Number(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::Greater { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left > right;
                    return Ok(LiteralType::Bool(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::GreaterEqual { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left >= right;
                    return Ok(LiteralType::Bool(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::Less { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left < right;
                    return Ok(LiteralType::Bool(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::LessEqual { inner } => {
                if let (LiteralType::Number(left), LiteralType::Number(right)) = (left, right) {
                    let var_name = left <= right;
                    return Ok(LiteralType::Bool(var_name));
                }
                Err(InterError::Number(inner.clone()))
            },
            Token::BangEqual { .. } => {
                let b = !Self::is_equal(&left, &right);
                Ok(LiteralType::Bool(b))
            },
            Token::EqualEqual { .. } => {
                let b = Self::is_equal(&left, &right);
                Ok(LiteralType::Bool(b))
            },
            _ => Err(InterError::NotMatch("unreachable binary expr".to_owned())),
        }
    }

    fn visit_call_expr(&self, expr: &crate::expr::Call) -> Result<LiteralType> {
        todo!()
    }

    fn visit_get_expr(&self, expr: &crate::expr::Get) -> Result<LiteralType> {
        todo!()
    }
    fn visit_grouping_expr(&self, expr: &crate::expr::Grouping) -> Result<LiteralType> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&self, expr: &crate::expr::Literal) -> Result<LiteralType> {
        Ok(expr.value.clone())
    }

    fn visit_logical_expr(&self, expr: &crate::expr::Logical) -> Result<LiteralType> {
        todo!()
    }

    fn visit_set_expr(&self, expr: &crate::expr::Set) -> Result<LiteralType> {
        todo!()
    }

    fn visit_super_expr(&self, expr: &crate::expr::Super) -> Result<LiteralType> {
        todo!()
    }

    fn visit_this_expr(&self, expr: &crate::expr::This) -> Result<LiteralType> {
        todo!()
    }

    fn visit_unary_expr(&self, expr: &crate::expr::Unary) -> Result<LiteralType> {
        let right = self.evaluate(&expr.right)?;

        match &expr.operator {
            Token::Minus { inner } => {
                if let LiteralType::Number(n) = right {
                    return Ok(LiteralType::Number(-n));
                }
                Err(InterError::UnaryNumber(inner.clone()))
            },
            Token::Bang { .. } => {
                let is_truthy = Self::is_truthy(&right);
                Ok(LiteralType::Bool(!is_truthy))
            },
            _ => Err(InterError::NotMatch("unexpect unary expr".to_owned())),
        }
    }

    fn visit_variable_expr(&self, expr: &crate::expr::Variable) -> Result<LiteralType> {
        self.environment
            .get(&expr.name)
            .cloned()
            .ok_or(InterError::NoVar(expr.name.clone()))
    }
}
