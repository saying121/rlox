#![allow(unfulfilled_lint_expectations, reason = "allow it")]
use crate::{expr::Exprs, tokens::Token};

pub trait Stmt {
    fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R;
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Expression {
    expr: Exprs,
}

impl Expression {
    pub const fn new(expr: Exprs) -> Self {
        Self { expr }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Print {
    expr: Exprs,
}

impl Print {
    pub const fn new(expr: Exprs) -> Self {
        Self { expr }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Var {
    name: Token,
    expr: Exprs,
}

impl Var {
    pub const fn new(name: Token, expr: Exprs) -> Self {
        Self { name, expr }
    }

    pub const fn initializer(&self) -> &Exprs {
        &self.expr
    }

    pub fn var_name(&self) -> &str {
        self.name.inner().lexeme()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct Block {
    statements: Vec<Stmts>,
}

impl Block {
    pub const fn new(statements: Vec<Stmts>) -> Self {
        Self { statements }
    }

    pub fn statements(&self) -> &[Stmts] {
        &self.statements
    }
}

macro_rules! statement_gen {
    ($($stm:ident), *) => {
paste::paste! {

pub trait StmtVisitor<R> {
$(
    fn [<visit_ $stm:lower _stmt>](&mut self, stmt: &$stm) -> R;
)*
}

$(
    impl Stmt for $stm {
        fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R
        {
                visitor.[<visit_ $stm:lower _stmt>](self)
        }
    }
)*

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub enum Stmts {
$(
    $stm($stm),
)*
}

impl Stmt for Stmts {
    #[inline]
    fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        #[expect(clippy::enum_glob_use, reason = "happy")]
        use Stmts::*;
        match self {
        $(
            $stm(inner) => inner.accept(visitor),
        )*
        }
    }
}

}
    };
}

statement_gen!(Expression, Print, Var, Block);

macro_rules! statement_expr {
    ($($stm:ident), *) => {
paste::paste! {

$(
    impl $stm {
        pub const fn expr(&self) -> &Exprs {
            &self.expr
        }
    }
)*

}
    };
}

statement_expr!(Expression, Print, Var);
