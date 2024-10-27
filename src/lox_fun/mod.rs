use std::rc::Rc;

use crate::{
    env::Environment,
    expr::LiteralType,
    interpreter::{InterError, Interpreter},
    lox_callable::LoxCallable,
    stmt::Function,
};

type Result<T> = std::result::Result<T, InterError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct LoxFunction {
    pub declaration: Function,
}

impl LoxFunction {
    pub fn new(declaration: Function) -> Self {
        Self { declaration }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> Result<LiteralType> {
        let mut env = Environment::with_enclosing(Rc::clone(&inter.globals));
        for (tk, val) in self.declaration.params.iter().zip(args.iter()) {
            env.define(tk.inner().lexeme().to_owned(), val.clone());
        }

        inter.execute_block(&self.declaration.body, env)?;

        Ok(LiteralType::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("<fn {} >", self.declaration.name.inner().lexeme()).fmt(f)
    }
}
