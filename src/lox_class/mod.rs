#[cfg(test)]
mod tests;

use std::{collections::HashMap, fmt::Display};

use crate::{
    expr::LiteralType,
    interpreter::Interpreter,
    lox_callable::{CallResult, Callables, LoxCallable},
    lox_fun::LoxFunction,
    lox_instance::LoxInstance,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, LoxFunction>,
}

impl std::hash::Hash for LoxClass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        for ele in &self.methods {
            ele.hash(state);
        }
    }
}

impl LoxCallable for LoxClass {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> CallResult<LiteralType> {
        let instance = LoxInstance::new(self.clone());
        if let Some(initializer) = self.find_method("init") {
            initializer.bind(&instance).call(inter, args)?;
        }
        Ok(LiteralType::Callable(Callables::Instance(instance)))
    }

    fn arity(&self) -> usize {
        self.find_method("init").map_or(0, |init| init.arity())
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl LoxClass {
    pub const fn new(name: String, methods: HashMap<String, LoxFunction>) -> Self {
        Self { name, methods }
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.methods.get(name).cloned()
    }
}
