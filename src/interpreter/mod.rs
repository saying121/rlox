#[cfg(test)]
mod test;

use crate::{
    expr::{Expr, Exprs, LiteralType, Visitor},
    tokens::{Token, TokenInner},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(thiserror::Error)]
enum InterError {
    #[error("{0}\nhelp: Operand must be numbers.")]
    Number(TokenInner),
    #[error("{0}\nhelp: Operand must be a number.")]
    UnaryNumber(TokenInner),
    #[error("{0}\nhelp: Operand must be two number or two strings.")]
    Plus(TokenInner),
    #[error("{0}")]
    NotMatch(String),
}

pub type Result<T, E = InterError> = core::result::Result<T, E>;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Interpreter;

impl Interpreter {
    pub fn interpret(self, expr: &Exprs) {
        match self.evaluate(expr) {
            Ok(i) => {
                println!("{}", i);
            },
            Err(e) => tracing::error!("{e}"),
        }
    }
    fn evaluate(&self, expr: &Exprs) -> Result<LiteralType> {
        expr.accept(self)
    }
    fn is_truthy(literal: &LiteralType) -> bool {
        !matches!(literal, LiteralType::Nil(_))
    }
    fn is_equal(a: LiteralType, b: LiteralType) -> bool {
        a == b
    }
}

impl Visitor<Result<LiteralType>> for Interpreter {
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
                let b = !Self::is_equal(left, right);
                return Ok(LiteralType::Bool(b));
            },
            Token::EqualEqual { .. } => {
                let b = Self::is_equal(left, right);
                return Ok(LiteralType::Bool(b));
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
        todo!()
    }
}
