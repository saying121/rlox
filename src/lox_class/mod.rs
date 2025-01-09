#[cfg(test)]
mod tests;

use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{
    expr::LiteralType,
    interpreter::Interpreter,
    lox_callable::{CallResult, LoxCallable},
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
    superclass: Option<Box<LoxClass>>,
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
        let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        if let Some(initializer) = self.find_method("init") {
            initializer.bind(Rc::clone(&instance)).call(inter, args)?;
        }
        Ok(LiteralType::LoxInstance(Rc::clone(&instance)))
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
    pub const fn new(
        name: String,
        superclass: Option<Box<Self>>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        Self {
            name,
            methods,
            superclass,
        }
    }

    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let m @ Some(_) = self.methods.get(name) {
            return m.cloned();
        }

        self.superclass
            .as_ref()
            .and_then(|sup| sup.find_method(name))
    }
}
