#[cfg(test)]
mod test;

use std::{cell::RefCell, rc::Rc, time::SystemTimeError};

use crate::{
    env::Environment,
    expr::{Expr, ExprVisitor, Exprs, LiteralType},
    lox_callable::{Callables, LoxCallable},
    lox_fun::{ClockFunction, LoxFunction},
    r#return::FnReturn,
    stmt::{
        Block, Break, Expression, Function, If, Print, Return, Stmt, StmtVisitor, Stmts, Var, While,
    },
    tokens::{Token, TokenInner},
};

#[derive(Clone)]
#[derive(Debug)]
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
    #[error("Should not use `break` out of loop: {0}")]
    NeedBreak(Token),
    #[error("{0}")]
    Message(String),
    #[error("Can not call: {0}")]
    NotCallable(Token),
    #[error("Args arity not match: {tk}, expected: {expect}, but got {actual}")]
    ArgsArity {
        tk: Token,
        expect: usize,
        actual: usize,
    },
    #[error("Get time failed: {0}")]
    Time(#[from] SystemTimeError),
    // TODO: maybe use Result<Resturn, Error>
    #[error("Fn return value: {0}")]
    Return(crate::r#return::FnReturn),
}

pub type Result<T> = core::result::Result<T, InterError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        globals.define(
            "clock".to_owned(),
            LiteralType::Callable(Callables::Clock(ClockFunction)),
        );
        let globals = Rc::new(RefCell::new(globals));
        Self {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
        }
    }

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

    pub fn execute_block(&mut self, statements: &[Stmts], env: Environment) -> Result<()> {
        let previous = Rc::clone(&self.environment);
        self.environment = Rc::new(RefCell::new(env));

        let res: Result<()> = try {
            for stmt in statements {
                self.execute(stmt)?;
            }
        };

        self.environment = previous;

        res
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
        self.environment
            .borrow_mut()
            .define(stmt.var_name().to_owned(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<()> {
        self.execute_block(
            stmt.statements(),
            Environment::with_enclosing(Rc::clone(&self.environment)),
        )?;
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
        let res: Result<()> = try {
            while Self::is_truthy(&self.evaluate(stmt.condition())?) {
                self.execute(stmt.body())?;
            }
        };
        match res {
            r @ Ok(_) => r,
            Err(InterError::NeedBreak(_)) => Ok(()),
            e @ Err(_) => e,
        }
    }

    fn visit_break_stmt(&mut self, stmt: &Break) -> Result<()> {
        Err(InterError::NeedBreak(stmt.lexeme().clone()))
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<()> {
        let fun = LoxFunction::new(stmt.clone());
        self.environment.borrow_mut().define(
            stmt.name.inner().lexeme().to_owned(),
            LiteralType::Callable(Callables::Fun(fun)),
        );

        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<()> {
        if let Some(v) = &stmt.value {
            let value = self.evaluate(v)?;
            return Err(InterError::Return(FnReturn::new(value)));
        }

        Ok(())
    }
}

impl ExprVisitor<Result<LiteralType>> for Interpreter {
    fn visit_assign_expr(&mut self, expr: &crate::expr::Assign) -> Result<LiteralType> {
        let value = self.evaluate(&expr.value)?;
        self.environment
            .borrow_mut()
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
        let callee = self.evaluate(&expr.callee)?;
        let LiteralType::Callable(callee) = callee
        else {
            return Err(InterError::NotCallable(expr.name.clone()));
        };
        let mut args = Vec::with_capacity(expr.arguments.len());
        for arg in &expr.arguments {
            args.push(self.evaluate(arg)?);
        }
        let res = match callee {
            Callables::Fun(fun) => {
                if args.len() != fun.arity() {
                    return Err(InterError::ArgsArity {
                        tk: expr.name.clone(),
                        expect: fun.arity(),
                        actual: args.len(),
                    });
                }
                fun.call(self, args)?
            },
            Callables::Clock(clock_function) => clock_function.call(self, vec![])?,
        };
        Ok(res)
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
        if let Some(v) = self.environment.borrow().get(&expr.name) {
            return Ok(v);
        }

        self.globals
            .borrow_mut()
            .get(&expr.name)
            .ok_or_else(|| InterError::NoVar(expr.name.clone()))
    }
}
