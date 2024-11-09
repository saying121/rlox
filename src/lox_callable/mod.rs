use std::fmt::{self, Display};

use crate::{
    expr::LiteralType,
    interpreter::{InterError, Interpreter},
    lox_fun::{ClockFunction, LoxFunction},
};

type Result<T> = std::result::Result<T, InterError>;

pub trait LoxCallable {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> Result<LiteralType>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Callables {
    Fun(LoxFunction),
    Clock(ClockFunction),
}

impl Display for Callables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fun(lox_function) => lox_function.fmt(f),
            Self::Clock(clock_function) => clock_function.fmt(f),
        }
    }
}