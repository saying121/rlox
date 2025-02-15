use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Str(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(d) => d.fmt(f),
            Self::Bool(b) => b.fmt(f),
            Self::Nil => "nil".fmt(f),
            Self::Str(s) => s.fmt(f),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct ValueArray(pub Vec<Value>);

impl DerefMut for ValueArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for ValueArray {
    // type Target = Vec<Value>;
    type Target = [Value];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ValueArray {
    pub const fn new() -> Self {
        Self(vec![])
    }

    pub fn write(&mut self, v: Value) {
        self.0.push(v);
    }
}
