use std::{collections::HashMap, fmt::Display};

use crate::{expr::LiteralType, lox_class::LoxClass, lox_fun::LoxFunction, tokens::Token};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: HashMap<String, LiteralType>,
}

impl std::hash::Hash for LoxInstance {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.klass.hash(state);
        for ele in &self.fields {
            ele.hash(state);
        }
    }
}

impl Eq for LoxInstance {}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} instance", self.klass).fmt(f)
    }
}

impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        Self {
            klass,
            fields: HashMap::new(),
        }
    }
    pub fn get(&self, name: &Token) -> Option<LiteralType> {
        if let m @ Some(_) = self.fields.get(name.inner().lexeme()) {
            return m.cloned();
        }

        let method: Option<LoxFunction> = self.klass.find_method(name.inner().lexeme());

        method.map(|m| {
            let fun = m.bind(self);
            LiteralType::Callable(crate::lox_callable::Callables::Fun(fun))
        })
    }

    pub fn set(&mut self, name: Token, value: LiteralType) {
        self.fields.insert(name.into_inner().lexeme_take(), value);
    }
}
