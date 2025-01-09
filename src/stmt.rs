use crate::{
    expr::{Exprs, Variable},
    token::Token,
};

pub trait Stmt {
    fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R;
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct Class {
    name: Token,
    superclass: Option<Variable>,
    methods: Vec<Function>, // Stmts::Function
}

impl Class {
    pub fn new<S: Into<Option<Variable>>>(
        name: Token,
        superclass: S,
        methods: Vec<Function>,
    ) -> Self {
        Self {
            name,
            superclass: superclass.into(),
            methods,
        }
    }

    pub const fn name(&self) -> &Token {
        &self.name
    }

    pub fn methods(&self) -> &[Function] {
        &self.methods
    }

    pub const fn superclass(&self) -> Option<&Variable> {
        self.superclass.as_ref()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct Return {
    keyword: Token,
    value: Option<Exprs>,
}

impl Return {
    pub fn new<V: Into<Option<Exprs>>>(keyword: Token, value: V) -> Self {
        Self {
            keyword,
            value: value.into(),
        }
    }

    pub const fn value(&self) -> Option<&Exprs> {
        self.value.as_ref()
    }

    pub const fn keyword(&self) -> &Token {
        &self.keyword
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmts>,
}

impl Function {
    pub const fn new(name: Token, params: Vec<Token>, body: Vec<Stmts>) -> Self {
        Self { name, params, body }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
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
#[derive(PartialEq, Eq, Hash)]
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
#[derive(PartialEq, Eq, Hash)]
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

    pub const fn name(&self) -> &Token {
        &self.name
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, Hash)]
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

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct If {
    condition: Exprs,
    then_branch: Box<Stmts>,
    else_branch: Option<Box<Stmts>>,
}

impl If {
    pub fn new<E>(condition: Exprs, then_branch: Box<Stmts>, else_branch: E) -> Self
    where
        E: Into<Option<Box<Stmts>>>,
    {
        Self {
            condition,
            then_branch,
            else_branch: else_branch.into(),
        }
    }

    pub const fn condition(&self) -> &Exprs {
        &self.condition
    }

    pub const fn then_branch(&self) -> &Stmts {
        &self.then_branch
    }

    pub fn else_branch(&self) -> Option<&Stmts> {
        self.else_branch.as_deref()
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
pub struct While {
    condition: Exprs,
    body: Box<Stmts>,
}

impl While {
    pub const fn new(condition: Exprs, body: Box<Stmts>) -> Self {
        Self { condition, body }
    }

    pub const fn condition(&self) -> &Exprs {
        &self.condition
    }

    pub const fn body(&self) -> &Stmts {
        &self.body
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, Hash, PartialOrd)]
pub struct Break {
    lexeme: Token,
}

impl Break {
    pub const fn new(lexeme: Token) -> Self {
        Self { lexeme }
    }

    pub const fn lexeme(&self) -> &Token {
        &self.lexeme
    }
}

macro_rules! statement_gen {
    ($($stm:ident), *,) => {
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
#[derive(PartialEq, Eq, Hash)]
pub enum Stmts {
$(
    $stm($stm),
)*
}

impl Stmt for Stmts {
    #[inline]
    fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
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

statement_gen!(Expression, Print, Var, Block, If, While, Break, Function, Return, Class,);

impl From<Stmts> for Option<Box<Stmts>> {
    fn from(val: Stmts) -> Self {
        Some(Box::new(val))
    }
}

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
