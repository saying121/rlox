use std::fmt::{self, Display};

use crate::interpreter::InterError;
use crate::{expr::LiteralType, interpreter::Interpreter, lox_fun::LoxFunction};

type Result<T> = std::result::Result<T, InterError>;

pub trait LoxCallable {
    fn call(&self, inter: &mut Interpreter, args: Vec<LiteralType>) -> Result<LiteralType>;
    fn arity(&self) -> usize;
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub enum Callables {
    Fun(LoxFunction),
}

impl Display for Callables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// #[derive(Clone, Copy)]
// #[derive(Debug)]
// #[derive(Default)]
// #[derive(PartialEq, Eq, PartialOrd, Ord)]
// pub struct Name {
//     field: Box<dyn LoxCallable>,
// }
