#[cfg(test)]
mod tests;

use std::fmt::Display;

use crate::{
    expr::LiteralType,
    interpreter::Interpreter,
    lox_callable::{CallResult, Callables, LoxCallable},
    lox_instance::LoxInstance,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LoxClass {
    name: String,
}

impl LoxCallable for LoxClass {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> CallResult<LiteralType> {
        let instance = LoxInstance::new(self.clone());
        Ok(LiteralType::Callable(Callables::Instance(instance)))
    }

    fn arity(&self) -> usize {
        0
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl LoxClass {
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}
