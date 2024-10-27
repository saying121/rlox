use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    enclosing: Option<Rc<RefCell<Environment>>>,
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
    pub fn with_enclosing(enclosing: Rc<RefCell<Self>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Option<LiteralType> {
        if let v @ Some(_) = self.values.get(name.inner().lexeme()).cloned() {
            return v;
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
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
        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(EnvError::UndefinedVar(name.clone()))
    }
}
