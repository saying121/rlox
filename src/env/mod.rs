use std::collections::HashMap;

use crate::{expr::LiteralType, tokens::Token};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct Environment {
    values: HashMap<String, LiteralType>,
}

impl Environment {
    pub fn get(&self, name: &Token) -> Option<&LiteralType> {
        let name = name.inner().lexeme();
        self.values.get(name)
    }

    pub fn define(&mut self, name: String, value: LiteralType) {
        self.values.insert(name, value);
    }
}
