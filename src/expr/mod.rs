use std::fmt::Display;

use crate::tokens::{Nil, Token};

pub trait Visitor<R> {
    fn visit_assign_expr(&self, expr: &Assign) -> R;
    fn visit_binary_expr(&self, expr: &Binary) -> R;
    fn visit_call_expr(&self, expr: &Call) -> R;
    fn visit_get_expr(&self, expr: &Get) -> R;
    fn visit_grouping_expr(&self, expr: &Grouping) -> R;
    fn visit_literal_expr(&self, expr: &Literal) -> R;
    fn visit_logical_expr(&self, expr: &Logical) -> R;
    fn visit_set_expr(&self, expr: &Set) -> R;
    fn visit_super_expr(&self, expr: &Super) -> R;
    fn visit_this_expr(&self, expr: &This) -> R;
    fn visit_unary_expr(&self, expr: &Unary) -> R;
    fn visit_variable_expr(&self, expr: &Variable) -> R;
}

pub trait Expr {
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R;
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Exprs {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}
impl Expr for Exprs {
    #[inline]
    fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        #[expect(clippy::enum_glob_use, reason = "happy")]
        use Exprs::*;
        match self {
            Assign(inner) => inner.accept(visitor),
            Binary(inner) => inner.accept(visitor),
            Call(inner) => inner.accept(visitor),
            Get(inner) => inner.accept(visitor),
            Grouping(inner) => inner.accept(visitor),
            Literal(inner) => inner.accept(visitor),
            Logical(inner) => inner.accept(visitor),
            Set(inner) => inner.accept(visitor),
            Super(inner) => inner.accept(visitor),
            This(inner) => inner.accept(visitor),
            Unary(inner) => inner.accept(visitor),
            Variable(inner) => inner.accept(visitor),
        }
    }
}

macro_rules! impl_expr {
    ($($expr:ident), *) => {
        $(
            impl Expr for $expr {
                fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R
                {
                    paste::paste! {
                        visitor.[<visit_ $expr:lower _expr>](self)
                    }
                }
            }
        )*
    };
}

impl_expr!(
    Assign, Binary, Call, Get, Grouping, Literal, Logical, Set, Super, This, Unary, Variable
);

#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub enum LiteralType {
    String(String),
    Number(f64),
    Bool(bool),
    Nil(Nil), // CallAble(Callable),
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #![expect(clippy::enum_glob_use, reason = "happy")]
        use LiteralType::*;
        match self {
            String(s) => f.write_str(s),
            Number(n) => f.write_fmt(format_args!("{n}")),
            Bool(b) => f.write_fmt(format_args!("{b}")),
            Nil(n) => f.write_fmt(format_args!("{n}")),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Literal {
    pub value: LiteralType,
}

#[derive(Debug)]
#[derive(Clone)]
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
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub const fn new(name: Token) -> Self {
        Self { name }
    }
}
