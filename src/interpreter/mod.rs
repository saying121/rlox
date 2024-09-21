#[cfg(test)]
mod test;

use crate::{
    env::Environment,
    expr::{Expr, ExprVisitor, Exprs, LiteralType},
    stmt::{Block, Expression, If, Print, Stmt, StmtVisitor, Stmts, Var, While},
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
    #[error("{0}")]
    Message(String),
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

    pub fn evaluate(&mut self, expr: &Exprs) -> Result<LiteralType> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmts) -> Result<()> {
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

    fn execute_block(&mut self, statements: &[Stmts]) -> Result<()> {
        self.environment.enter_block();

        for stmt in statements {
            self.execute(stmt)?;
        }

        unsafe { self.environment.out_block() };

        Ok(())
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

    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<()> {
        self.execute_block(stmt.statements())?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<()> {
        let cond = self.evaluate(stmt.condition())?;
        let cond = Self::is_truthy(&cond);
        if cond {
            self.execute(stmt.then_branch())?;
        }
        else if let Some(else_branch) = stmt.else_branch() {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<()> {
        while Self::is_truthy(&self.evaluate(stmt.condition())?) {
            self.execute(stmt.body())?;
        }
        Ok(())
    }
}

impl ExprVisitor<Result<LiteralType>> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> Result<LiteralType> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .assign(&expr.name, value.clone())
            .map_err(|v| InterError::Message(v.to_string()))?;
        Ok(value)
    }

    fn visit_binary_expr(&mut self, expr: &crate::expr::Binary) -> Result<LiteralType> {
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

    fn visit_call_expr(&mut self, expr: &crate::expr::Call) -> Result<LiteralType> {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &crate::expr::Get) -> Result<LiteralType> {
        todo!()
    }
    fn visit_grouping_expr(&mut self, expr: &crate::expr::Grouping) -> Result<LiteralType> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &crate::expr::Literal) -> Result<LiteralType> {
        Ok(expr.value.clone())
    }

    fn visit_logical_expr(&mut self, expr: &crate::expr::Logical) -> Result<LiteralType> {
        let left = self.evaluate(&expr.left)?;

        match &expr.operator {
            Token::Or { .. } => {
                if Self::is_truthy(&left) {
                    return Ok(left);
                }
            },
            _ => {
                if !Self::is_truthy(&left) {
                    return Ok(left);
                }
            },
        }

        self.evaluate(&expr.right)
    }

    fn visit_set_expr(&mut self, expr: &crate::expr::Set) -> Result<LiteralType> {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &crate::expr::Super) -> Result<LiteralType> {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &crate::expr::This) -> Result<LiteralType> {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &crate::expr::Unary) -> Result<LiteralType> {
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

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> Result<LiteralType> {
        self.environment
            .get(&expr.name)
            .cloned()
            .ok_or(InterError::NoVar(expr.name.clone()))
    }
}
