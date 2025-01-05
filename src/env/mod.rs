use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::{expr::LiteralType, tokens::Token};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd)]
#[derive(thiserror::Error)]
pub enum EnvError {
    #[error("Not define: {0}")]
    UndefinedVar(Token),
    #[error("No var: `{name}`, distance: {distance}")]
    NoVar { distance: usize, name: String },
    #[error("Distance not enough depth: {0}")]
    Distance(usize),
}

pub type Result<T> = core::result::Result<T, EnvError>;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    /// NOTE: Use [`std::rc::Rc`] pointers to avoid expensive cloning of [`Environment::ancestor`]
    values: Rc<RefCell<HashMap<String, LiteralType>>>,
}

impl Hash for Environment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if let Some(encl) = &self.enclosing {
            encl.borrow().hash(state);
        }
        for ele in &*self.values.borrow() {
            ele.hash(state);
        }
    }
}

impl Environment {
    /// global scope
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    /// local scope
    pub fn with_enclosing(enclosing: Rc<RefCell<Self>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, name: &Token) -> Option<LiteralType> {
        if let v @ Some(_) = self.values.borrow().get(name.inner().lexeme()).cloned() {
            return v;
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        None
    }

    pub fn define(&self, name: String, value: LiteralType) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn assign(&self, name: &Token, value: LiteralType) -> Result<()> {
        let k = name.inner().lexeme();
        if self.values.borrow().contains_key(k) {
            self.values.borrow_mut().insert(k.to_owned(), value);
            return Ok(());
        }
        if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(EnvError::UndefinedVar(name.clone()))
    }
    pub fn get_at(&self, distance: usize, name: &str) -> Result<LiteralType> {
        self.ancestor(distance)?
            .borrow()
            .values
            .borrow()
            .get(name)
            .cloned()
            .ok_or_else(|| EnvError::NoVar {
                distance,
                name: name.to_owned(),
            })
    }

    fn ancestor(&self, distance: usize) -> Result<Rc<RefCell<Self>>> {
        let mut env = Rc::new(RefCell::new(self.clone()));

        for _ in 0..distance {
            let enc = Rc::clone(
                env.borrow()
                    .enclosing
                    .as_ref()
                    .ok_or(EnvError::Distance(distance))?,
            );

            env = enc;
        }

        Ok(env)
    }
    pub fn assign_at(&self, distance: usize, name: &Token, value: LiteralType) -> Result<()> {
        self.ancestor(distance)?
            .borrow()
            .values
            .borrow_mut()
            .insert(name.inner().lexeme().to_owned(), value);
        Ok(())
    }
}
