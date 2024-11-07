use std::fmt::Display;

use crate::expr::LiteralType;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub struct FnReturn {
    pub value: LiteralType,
}

impl FnReturn {
    pub const fn new(value: LiteralType) -> Self {
        Self { value }
    }
}

impl Display for FnReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
