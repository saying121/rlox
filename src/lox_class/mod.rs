use std::fmt::Display;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Hash)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct LoxClass {
    name: String,
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

impl LoxClass {
    pub const fn new(name: String) -> Self {
        Self { name }
    }
}
