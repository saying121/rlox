use std::{cell::RefCell, fmt::Display, rc::Rc, time::SystemTime};

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
#[derive(PartialEq, Eq)]
pub struct LoxFunction {
    pub declaration: Function,
    pub closure: Rc<RefCell<Environment>>,
}

impl std::hash::Hash for LoxFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.declaration.hash(state);
        self.closure.borrow().hash(state);
    }
}

impl LoxFunction {
    pub const fn new(declaration: Function, closure: Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> Result<LiteralType> {
        let env = Environment::with_enclosing(Rc::clone(&self.closure));
        for (tk, val) in self.declaration.params.iter().zip(args.iter()) {
            env.define(tk.inner().lexeme().to_owned(), val.clone());
        }
        match inter.execute_block(&self.declaration.body, env) {
            Ok(()) => {},
            Err(InterError::Return(fn_return)) => {
                return Ok(fn_return.value);
            },
            Err(e) => return Err(e),
        }

        Ok(LiteralType::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("<fn {}>", self.declaration.name.inner().lexeme()).fmt(f)
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ClockFunction;

impl Display for ClockFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<fn clock>(inner)")
    }
}

impl LoxCallable for ClockFunction {
    fn call(&self, _inter: &mut Interpreter, _args: Vec<LiteralType>) -> Result<LiteralType> {
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(v) => v,
            Err(e) => {
                return Err(e.into());
            },
        };

        Ok(LiteralType::Number(now.as_millis_f64()))
    }

    fn arity(&self) -> usize {
        0
    }
}
