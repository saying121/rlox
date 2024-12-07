use std::{collections::HashMap, fmt::Display};

use crate::{expr::LiteralType, lox_class::LoxClass, tokens::Token};

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
        // NOTE: is it need?
        // for ele in &self.fields {
        //     ele.hash(state);
        // }
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
        self.fields.get(name.inner().lexeme()).cloned()
    }
}
