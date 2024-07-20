use std::any::Any;

use enum_dispatch::enum_dispatch;

use crate::tokens::Token;

trait Visitor<R> {
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
trait Expr {
    fn greet(&self) {
        println!("Hello");
    }
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
            impl Expr for $expr {}

            impl $expr {
                fn accept<R, V>(&self, visitor: V) -> R
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

// #[derive(Debug)]
pub struct Binary {
    pub left:     Box<Exprs>,
    pub operator: Token,
    pub right:    Box<Exprs>,
}

// #[derive(Debug)]
pub struct Call {
    pub callee:    Box<Exprs>,
    pub paren:     Token,
    pub arguments: Vec<Exprs>,
}

// #[derive(Debug)]
pub struct Get {
    pub object: Box<Exprs>,
    pub name:   Token,
}

// #[derive(Debug)]
pub struct Grouping {
    pub expression: Box<Exprs>,
}

// #[derive(Debug)]
pub struct Literal {
    pub value: Box<dyn Any>,
}

// #[derive(Debug)]
pub struct Logical {
    pub left:     Box<Exprs>,
    pub operator: Token,
    pub right:    Box<Exprs>,
}

// #[derive(Debug)]
pub struct Set {
    pub object: Box<Exprs>,
    pub name:   Token,
    pub value:  Box<Exprs>,
}

// #[derive(Debug)]
pub struct Super {
    pub keyword: Token,
    pub method:  Token,
}

// #[derive(Debug)]
pub struct This {
    pub keyword: Token,
}

// #[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right:    Box<Exprs>,
}

// #[derive(Debug)]
pub struct Variable {
    pub name: Token,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::TokenInner;

    #[test]
    fn test_name() {
        let _var = "ab".to_owned();
        let lit: Exprs = Literal { value: Box::new(1) }.into();
        lit.greet();
    }
}
