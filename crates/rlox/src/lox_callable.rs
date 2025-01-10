use std::{
    fmt::{self, Display},
    hash::Hash,
};

use crate::{
    expr::LiteralType,
    interpreter::{InterError, Interpreter},
    lox_class::LoxClass,
    lox_fun::{ClockFunction, LoxFunction},
};

pub type CallResult<T> = std::result::Result<T, InterError>;

pub trait LoxCallable {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> CallResult<LiteralType>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Callables {
    Fun(LoxFunction),
    Clock(ClockFunction),
    Class(LoxClass),
}

impl Hash for Callables {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Self::Fun(f0) => f0.hash(state),
            Self::Clock(f0) => f0.hash(state),
            Self::Class(f0) => f0.hash(state),
        }
    }
}

impl Display for Callables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fun(lox_function) => lox_function.fmt(f),
            Self::Clock(clock_function) => clock_function.fmt(f),
            Self::Class(lox_class) => lox_class.fmt(f),
        }
    }
}
