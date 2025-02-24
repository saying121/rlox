use std::fmt::Display;

use crate::chunk::Chunk;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub enum Obj {
    String(String),
    Fun(ObjFunction),
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => s.fmt(f),
            Self::Fun(fun) => write!(f, "<fn {}>", fun.name),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct ObjFunction {
    pub arity: usize,
    pub chunk: Chunk,
    pub name: String,
}

impl Display for ObjFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.name.is_empty() {
            write!(f, "<script>")?;
        }
        write!(f, "<fn {}>", self.name)
    }
}

impl ObjFunction {
    pub const fn new() -> Self {
        Self {
            arity: 0,
            chunk: Chunk::new(),
            name: String::new(),
        }
    }
}

impl Default for ObjFunction {
    fn default() -> Self {
        Self::new()
    }
}
