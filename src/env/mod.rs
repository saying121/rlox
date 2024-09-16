use std::collections::HashMap;

use crate::{expr::LiteralType, tokens::Token};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
#[derive(thiserror::Error)]
pub enum EnvError {
    #[error("Not define: {0}")]
    UndefinedVar(Token),
}

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

    pub fn assign(&mut self, name: &Token, value: LiteralType) -> Result<(), EnvError> {
        let k = name.inner().lexeme();
        if self.values.contains_key(k) {
            self.values.insert(k.to_owned(), value);
            Ok(())
        }
        else {
            Err(EnvError::UndefinedVar(name.clone()))
        }
    }
}
