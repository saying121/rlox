#![allow(unfulfilled_lint_expectations, reason = "allow it")]

use std::{cell::RefCell, fmt::Display, hash::Hash, rc::Rc};

use crate::{lox_callable::Callables, lox_instance::LoxInstance, token::Token};

pub trait Expr {
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R;
}

macro_rules! expr_gen {
    ($($variant:ident), *) => {
paste::paste! {

pub trait ExprVisitor<R> {
$(
    fn [<visit_ $variant:lower _expr>](&mut self, expr: &$variant) -> R;
)*
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub enum Exprs {
$(
    $variant($variant),
)*
}

impl Expr for Exprs {
    #[inline]
    fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        #[expect(clippy::enum_glob_use, reason = "happy")]
        use Exprs::*;
        match self {
        $(
            $variant(inner) => inner.accept(visitor),
        )*
        }
    }
}

$(
    impl Expr for $variant {
        fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R
        {
                visitor.[<visit_ $variant:lower _expr>](self)
        }
    }
)*

}
    };
}

expr_gen!(
    Assign, Binary, Call, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable
);

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Assign {
    name: Token,
    value: Box<Exprs>,
}

impl Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl Assign {
    pub fn new(name: Token, value: Exprs) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }

    pub const fn value(&self) -> &Exprs {
        &self.value
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Binary {
    left: Box<Exprs>,
    operator: Token,
    right: Box<Exprs>,
}

impl Binary {
    pub fn new(left: Exprs, operator: Token, right: Exprs) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub const fn operator(&self) -> &Token {
        &self.operator
    }

    pub const fn left(&self) -> &Exprs {
        &self.left
    }

    pub const fn right(&self) -> &Exprs {
        &self.right
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Call {
    callee: Box<Exprs>,
    name: Token,
    arguments: Vec<Exprs>,
}

impl Call {
    pub fn new(callee: Exprs, paren: Token, arguments: Vec<Exprs>) -> Self {
        Self {
            callee: Box::new(callee),
            name: paren,
            arguments,
        }
    }

    pub const fn callee(&self) -> &Exprs {
        &self.callee
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }

    pub fn arguments(&self) -> &[Exprs] {
        &self.arguments
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Get {
    pub object: Box<Exprs>,
    pub name: Token,
}

impl Get {
    pub fn new(object: Exprs, name: Token) -> Self {
        Self {
            object: Box::new(object),
            name,
        }
    }

    pub const fn object(&self) -> &Exprs {
        &self.object
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Grouping {
    expression: Box<Exprs>,
}

impl Grouping {
    pub fn new(expression: Exprs) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }

    pub const fn expression(&self) -> &Exprs {
        &self.expression
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(PartialEq, Eq, Hash)]
pub struct Literal {
    value: LiteralType,
}

impl Literal {
    pub const fn new(value: LiteralType) -> Self {
        Self { value }
    }

    pub const fn value(&self) -> &LiteralType {
        &self.value
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Bool(bool),
    #[default]
    Nil,
    Callable(Callables),
    LoxInstance(Rc<RefCell<LoxInstance>>),
}

impl Hash for LiteralType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self {
            Self::String(s) => s.hash(state),
            Self::Number(n) => n.to_bits().hash(state),
            Self::Bool(b) => b.hash(state),
            Self::Nil => "nil".hash(state),
            Self::Callable(callables) => callables.hash(state),
            Self::LoxInstance(instance) => instance.borrow().hash(state),
        }
    }
}

impl Eq for LiteralType {}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #![expect(clippy::enum_glob_use, reason = "happy")]
        use LiteralType::*;
        match self {
            String(s) => f.write_str(s),
            Number(n) => f.write_fmt(format_args!("{n}")),
            Bool(b) => f.write_fmt(format_args!("{b}")),
            Nil => f.write_fmt(format_args!("nil")),
            Callable(v) => v.fmt(f),
            LoxInstance(instance) => instance.borrow().fmt(f),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Logical {
    left: Box<Exprs>,
    operator: Token,
    right: Box<Exprs>,
}

impl Logical {
    pub fn new(left: Exprs, operator: Token, right: Exprs) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub const fn left(&self) -> &Exprs {
        &self.left
    }

    pub const fn operator(&self) -> &Token {
        &self.operator
    }

    pub const fn right(&self) -> &Exprs {
        &self.right
    }
}

#[derive(PartialEq, Eq, Hash)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Set {
    object: Box<Exprs>,
    name: Token,
    value: Box<Exprs>,
}

impl Set {
    pub fn new(object: Exprs, name: Token, value: Exprs) -> Self {
        Self {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }

    pub const fn object(&self) -> &Exprs {
        &self.object
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }

    pub const fn value(&self) -> &Exprs {
        &self.value
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
pub struct Super {
    keyword: Token,
    method: Token,
}

impl Super {
    pub const fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }

    pub const fn keyword(&self) -> &Token {
        &self.keyword
    }

    pub const fn method(&self) -> &Token {
        &self.method
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
pub struct This {
    keyword: Token,
}

impl This {
    pub const fn new(keyword: Token) -> Self {
        Self { keyword }
    }

    pub const fn keyword(&self) -> &Token {
        &self.keyword
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
pub struct Unary {
    operator: Token,
    right: Box<Exprs>,
}

impl Unary {
    pub fn new(operator: Token, right: Exprs) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub const fn right(&self) -> &Exprs {
        &self.right
    }

    pub const fn operator(&self) -> &Token {
        &self.operator
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
pub struct Variable {
    name: Token,
}

impl Variable {
    pub const fn new(name: Token) -> Self {
        Self { name }
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }

    pub fn into_name(self) -> Token {
        self.name
    }

    pub fn name_str(&self) -> &str {
        self.name.lexeme()
    }
}
