use std::collections::HashMap;

use crate::{
    expr::*,
    interpreter::Interpreter,
    parser::{ParserError, Result},
    stmt::*,
};

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Resolver<'i> {
    pub interpreter: &'i mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
    current_fun: FunctionType,
    current_class: ClassType,
    had_err: bool,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum FunctionType {
    #[default]
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ClassType {
    #[default]
    None,
    Class,
    SubClass,
}

impl<'i> Resolver<'i> {
    pub fn new(interpreter: &'i mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_fun: FunctionType::None,
            current_class: ClassType::Class,
            had_err: false,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn end_scope(&mut self) {
        // TODO: judge it empty?
        self.scopes.pop();
    }

    pub fn resolve(&mut self, statements: &[Stmts]) -> bool {
        for stmt in statements {
            if let Err(e) = self.resolve_stmt(stmt) {
                tracing::error!("{e}");
                self.had_err = true;
            }
        }
        self.had_err
    }

    fn resolve_stmt(&mut self, stmt: &Stmts) -> Result<()> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Exprs) -> Result<()> {
        expr.accept(self)
    }

    fn resolve_expr_variable(&mut self, expr: &Variable) -> Result<()> {
        expr.accept(self)
    }

    fn declare(&mut self, name: &crate::token::Token) -> Result<()> {
        let Some(last) = self.scopes.last_mut()
        else {
            // Without local scope it will look global
            return Ok(());
        };
        if last.contains_key(name.lexeme()) {
            return Err(ParserError::DoubleVar(name.clone()));
        }
        last.insert(name.lexeme().to_owned(), false);
        Ok(())
    }

    fn define(&mut self, name: &crate::token::Token) {
        if let Some(last) = self.scopes.last_mut() {
            last.insert(name.lexeme().to_owned(), true);
        }
    }

    fn resolve_local(&mut self, expr: &Exprs, name: &crate::token::Token) {
        for (i, ele) in self.scopes.iter().enumerate() {
            if ele.contains_key(name.lexeme()) {
                self.interpreter.resolve(expr, self.scopes.len() - 1 - i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, stmt: &Function, ft: FunctionType) -> Result<()> {
        let enclosing_fun = self.current_fun;
        self.current_fun = ft;
        self.begin_scope();

        for ele in &stmt.params {
            self.declare(ele)?;
            self.define(ele);
        }
        self.resolve(&stmt.body);
        self.end_scope();
        self.current_fun = enclosing_fun;
        Ok(())
    }
}

impl crate::expr::ExprVisitor<Result<()>> for Resolver<'_> {
    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<()> {
        self.resolve_expr(expr.value())?;
        // PERF: avoid `.clone()` add `self.resolve_local_assign` method
        self.resolve_local(&Exprs::Assign(expr.clone()), expr.name());
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<()> {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())
    }

    fn visit_call_expr(&mut self, expr: &Call) -> Result<()> {
        self.resolve_expr(expr.callee())?;
        for ele in expr.arguments() {
            self.resolve_expr(ele)?;
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, expr: &Get) -> Result<()> {
        self.resolve_expr(expr.object())
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<()> {
        self.resolve_expr(expr.expression())
    }

    fn visit_literal_expr(&mut self, _expr: &Literal) -> Result<()> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<()> {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())
    }

    fn visit_set_expr(&mut self, expr: &Set) -> Result<()> {
        self.resolve_expr(expr.value())?;
        self.resolve_expr(expr.object())
    }

    fn visit_super_expr(&mut self, expr: &Super) -> Result<()> {
        match self.current_class {
            ClassType::None => return Err(ParserError::NotInClassSuper(expr.keyword().clone())),
            ClassType::Class => return Err(ParserError::ClassNoSuper(expr.keyword().clone())),
            ClassType::SubClass => {},
        }
        self.resolve_local(&Exprs::Super(expr.clone()), expr.keyword());
        Ok(())
    }

    fn visit_this_expr(&mut self, expr: &This) -> Result<()> {
        if matches!(self.current_class, ClassType::None) {
            return Err(ParserError::NotInClassThis(expr.keyword().clone()));
        }
        self.resolve_local(&Exprs::This(expr.clone()), expr.keyword());
        Ok(())
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<()> {
        self.resolve_expr(expr.right())
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<()> {
        if let Some(last) = self.scopes.last()
            && last.get(expr.name_str()) == Some(&false)
        {
            return Err(ParserError::Initialization(expr.name().clone()));
        }

        self.resolve_local(&Exprs::Variable(expr.clone()), expr.name());

        Ok(())
    }
}

impl crate::stmt::StmtVisitor<Result<()>> for Resolver<'_> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<()> {
        self.resolve_expr(stmt.expr())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<()> {
        self.resolve_expr(stmt.expr())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<()> {
        self.declare(stmt.name())?;

        if let Some(v) = stmt.initializer() {
            self.resolve_expr(v)?;
        }

        self.define(stmt.name());

        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<()> {
        self.begin_scope();
        self.resolve(stmt.statements());
        self.end_scope();

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<()> {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.then_branch())?;
        if let Some(else_b) = stmt.else_branch() {
            self.resolve_stmt(else_b)?;
        }
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<()> {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.body())
    }

    fn visit_break_stmt(&mut self, _stmt: &Break) -> Result<()> {
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<()> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name);

        self.resolve_function(stmt, FunctionType::Function)?;
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<()> {
        if matches!(self.current_fun, FunctionType::None) {
            return Err(ParserError::NotInFn(stmt.keyword().clone()));
        }
        if let Some(v) = stmt.value() {
            if matches!(self.current_fun, FunctionType::Initializer) {
                return Err(ParserError::RtValInit(stmt.keyword().clone()));
            }
            self.resolve_expr(v)?;
        }
        Ok(())
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> Result<()> {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(stmt.name())?;
        self.define(stmt.name());

        if let Some(superclass) = stmt.superclass() {
            if stmt.name().lexeme() == superclass.name().lexeme() {
                return Err(ParserError::RecurseClass(superclass.name().clone()));
            }

            self.current_class = ClassType::SubClass;

            self.resolve_expr_variable(superclass)?;

            // env for superclass
            self.begin_scope();

            unsafe {
                self.scopes
                    .last_mut()
                    .unwrap_unchecked()
                    .insert("super".to_owned(), true)
            };
        }

        self.begin_scope();

        unsafe {
            self.scopes
                .last_mut()
                .unwrap_unchecked()
                .insert("this".to_owned(), true)
        };

        for method in stmt.methods() {
            let declaration = if method.name.lexeme().eq("init") {
                FunctionType::Initializer
            }
            else {
                FunctionType::Method
            };
            self.resolve_function(method, declaration)?;
        }

        self.end_scope();

        if stmt.superclass().is_some() {
            self.end_scope();
        }

        self.current_class = enclosing_class;

        Ok(())
    }
}
