#![allow(unfulfilled_lint_expectations, reason = "allow it")]

use std::fmt::Display;

use crate::tokens::Token;

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
#[derive(PartialEq, PartialOrd)]
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

expr_gen!(Assign, Binary, Call, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable);

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Exprs>,
}

impl Assign {
    pub fn new(name: Token, value: Exprs) -> Self {
        Self {
            name,
            value: Box::new(value),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Binary {
    pub left: Box<Exprs>,
    pub operator: Token,
    pub right: Box<Exprs>,
}

impl Binary {
    pub fn new(left: Exprs, operator: Token, right: Exprs) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Call {
    pub callee: Box<Exprs>,
    pub paren: Token,
    pub arguments: Vec<Exprs>,
}

impl Call {
    pub fn new(callee: Exprs, paren: Token, arguments: Vec<Exprs>) -> Self {
        Self {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
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
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Grouping {
    pub expression: Box<Exprs>,
}

impl Grouping {
    pub fn new(expression: Exprs) -> Self {
        Self {
            expression: Box::new(expression),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct Literal {
    pub value: LiteralType,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Bool(bool),
    #[default]
    Nil, // CallAble(Callable),
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #![expect(clippy::enum_glob_use, reason = "happy")]
        use LiteralType::*;
        match self {
            String(s) => f.write_str(s),
            Number(n) => f.write_fmt(format_args!("{n}")),
            Bool(b) => f.write_fmt(format_args!("{b}")),
            Nil => f.write_fmt(format_args!("nil")),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Logical {
    pub left: Box<Exprs>,
    pub operator: Token,
    pub right: Box<Exprs>,
}

impl Logical {
    pub fn new(left: Exprs, operator: Token, right: Exprs) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Set {
    pub object: Box<Exprs>,
    pub name: Token,
    pub value: Box<Exprs>,
}

impl Set {
    pub fn new(object: Exprs, name: Token, value: Exprs) -> Self {
        Self {
            object: Box::new(object),
            name,
            value: Box::new(value),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

impl Super {
    pub const fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct This {
    pub keyword: Token,
}

impl This {
    pub const fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Exprs>,
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
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub const fn new(name: Token) -> Self {
        Self { name }
    }
}
