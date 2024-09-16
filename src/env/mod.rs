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
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, LiteralType>,
}

impl Environment {
    /// global scope
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    /// local scope
    pub fn with_enclosing(enclosing: Self) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Option<&LiteralType> {
        if let v @ Some(_) = self.values.get(name.inner().lexeme()) {
            return v;
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }
        None
    }

    pub fn define(&mut self, name: String, value: LiteralType) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: &Token, value: LiteralType) -> Result<(), EnvError> {
        let k = name.inner().lexeme();
        if self.values.contains_key(k) {
            self.values.insert(k.to_owned(), value);
            return Ok(());
        }
        if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value)?;
            return Ok(());
        }

        Err(EnvError::UndefinedVar(name.clone()))
    }
}
