use std::fmt::Display;

use enum_dispatch::enum_dispatch;

use crate::tokens::Token;

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

#[enum_dispatch]
pub trait Expr {
    fn accept<R, V>(&self, visitor: &V) -> R
    where
        V: Visitor<R>;
}

// #[derive(Debug)]
#[enum_dispatch(Expr)]
pub enum Exprs {
    Assign,
    Binary,
    Call,
    Get,
    Grouping,
    Literal,
    Logical,
    Set,
    Super,
    This,
    Unary,
    Variable,
}

macro_rules! impl_expr {
    ($($expr:ident), *) => {
        $(
            impl Expr for $expr {
                fn accept<R, V>(&self, visitor: &V) -> R
                where
                    V: Visitor<R>,
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

// #[derive(Debug)]
pub struct Assign {
    pub name:  Token,
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

// #[derive(Debug)]
pub struct Binary {
    pub left:     Box<Exprs>,
    pub operator: Token,
    pub right:    Box<Exprs>,
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

// #[derive(Debug)]
pub struct Call {
    pub callee:    Box<Exprs>,
    pub paren:     Token,
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

// #[derive(Debug)]
pub struct Get {
    pub object: Box<Exprs>,
    pub name:   Token,
}

impl Get {
    pub fn new(object: Exprs, name: Token) -> Self {
        Self {
            object: Box::new(object),
            name,
        }
    }
}

// #[derive(Debug)]
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

// #[derive(Debug)]
pub struct Literal {
    pub value: Box<dyn Display>,
}

impl Literal {
    pub fn new<T>(value: T) -> Self
    where
        T: Display + 'static,
    {
        Self {
            value: Box::new(value),
        }
    }
}

// #[derive(Debug)]
pub struct Logical {
    pub left:     Box<Exprs>,
    pub operator: Token,
    pub right:    Box<Exprs>,
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

// #[derive(Debug)]
pub struct Set {
    pub object: Box<Exprs>,
    pub name:   Token,
    pub value:  Box<Exprs>,
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

// #[derive(Debug)]
pub struct Super {
    pub keyword: Token,
    pub method:  Token,
}

impl Super {
    pub const fn new(keyword: Token, method: Token) -> Self {
        Self { keyword, method }
    }
}

// #[derive(Debug)]
pub struct This {
    pub keyword: Token,
}

impl This {
    pub const fn new(keyword: Token) -> Self {
        Self { keyword }
    }
}

// #[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right:    Box<Exprs>,
}

impl Unary {
    pub fn new(operator: Token, right: Exprs) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }
}

// #[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub const fn new(name: Token) -> Self {
        Self { name }
    }
}
