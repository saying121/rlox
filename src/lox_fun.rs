use std::{cell::RefCell, fmt::Display, rc::Rc, time::SystemTime};

use crate::{
    env::Environment,
    expr::LiteralType,
    interpreter::{InterError, Interpreter},
    lox_callable::{Callables, LoxCallable},
    lox_instance::LoxInstance,
    stmt::Function,
};

type Result<T> = std::result::Result<T, InterError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct LoxFunction {
    pub declaration: Function,
    pub closure: Rc<RefCell<Environment>>,
    is_init: bool,
}

impl std::hash::Hash for LoxFunction {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.declaration.hash(state);
        self.closure.borrow().hash(state);
    }
}

impl LoxFunction {
    pub const fn new(
        declaration: Function,
        closure: Rc<RefCell<Environment>>,
        is_init: bool,
    ) -> Self {
        Self {
            declaration,
            closure,
            is_init,
        }
    }

    pub fn bind(&self, arg: Rc<RefCell<LoxInstance>>) -> Self {
        let env = Environment::with_enclosing(Rc::clone(&self.closure));
        env.define(
            "this".to_owned(),
            LiteralType::Callable(Callables::Instance(arg)),
        );
        Self {
            declaration: self.declaration.clone(),
            closure: Rc::new(RefCell::new(env)),
            is_init: self.is_init,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> Result<LiteralType> {
        let env = Environment::with_enclosing(Rc::clone(&self.closure));
        for (tk, val) in self.declaration.params.iter().zip(args.iter()) {
            env.define(tk.lexeme().to_owned(), val.clone());
        }
        match inter.execute_block(&self.declaration.body, env) {
            Ok(()) => {},
            Err(InterError::Return(fn_return)) => {
                if self.is_init {
                    return Ok(self.closure.borrow().get_at(0, "this")?);
                }
                return Ok(fn_return.value);
            },
            Err(e) => return Err(e),
        }

        if self.is_init {
            let get_at = self.closure.borrow().get_at(0, "this")?;
            return Ok(get_at);
        }

        Ok(LiteralType::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl std::fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("<fn {}>", self.declaration.name.lexeme()).fmt(f)
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
