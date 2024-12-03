use std::fmt::Display;

use crate::lox_class::LoxClass;

#[derive(Hash)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LoxInstance {
    klass: LoxClass,
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{} instance", self.klass).fmt(f)
    }
}

impl LoxInstance {
    pub const fn new(klass: LoxClass) -> Self {
        Self { klass }
    }
}
