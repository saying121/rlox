#[cfg(test)]
mod tests;

use std::fmt::Display;

use itertools::PeekNth;
use thiserror::Error;

use crate::{
    expr::*,
    stmt::{Block, Break, Class, Expression, Function, If, Print, Return, Stmts, Var, While},
    token::Token,
};

#[derive(Clone)]
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Missing '(' after expression: {0}")]
    LeftParen(Token),
    #[error("Missing '{{' after expression: {0}")]
    LeftBrace(Token),
    #[error("Missing ')' after expression: {0}")]
    RightParen(Token),
    #[error("Missing '}}' after expression: {0}")]
    RightBrace(Token),
    #[error("End of source code, no next token: {0}")]
    Eof(String),
    #[error("Invalid Primary: {0}")]
    Primary(Token),
    #[error("Expect `;` at stmt end: {0}")]
    PrintStmt(Token),
    #[error("{0}")]
    DeclarationStmt(String),
    #[error("Expect var name: {0}")]
    VarDeclaration(Token),
    #[error("Expect `;` after variable declaration: {0}")]
    Semicolon(Token),
    #[error("Invalid assignment target: {0}")]
    Assign(Token),
    #[error("Must be inside a loop to use `break`")]
    NotInLoop(Token),
    #[error("Can't have more than 255 arguments: {0}")]
    TooManyArgs(Token),
    #[error("{0}")]
    ErrMessage(String),
    #[error("Expect {kind} name: {tk}")]
    CallDecl { tk: Token, kind: FunctionKind },
    #[error("Expect parameters name: {0}")]
    Parameters(Token),
    #[error("Expect Class name: {0}")]
    Class(Token),
    #[error("Expect superclass name: {0}")]
    Superclass(Token),
    #[error("A Class can't inherit from itself: {0}")]
    RecurseClass(Token),
    #[error("Can't read local variable in its own initializer: {0}")]
    Initialization(Token),
    #[error("There is no scope")]
    NotInScope,
    #[error("Already variable with this name in this scope: {0}")]
    DoubleVar(Token),
    #[error("Use `return` outer function: {0}")]
    NotInFn(Token),
    #[error("Use `this` outer class: {0}")]
    NotInClassThis(Token),
    #[error("Use `super` outer class: {0}")]
    NotInClassSuper(Token),
    #[error("Use `super` in a class with no superclass: {0}")]
    ClassNoSuper(Token),
    #[error("Can't return a value from an initializer: {0}")]
    RtValInit(Token),
}

pub type Result<T, E = ParserError> = core::result::Result<T, E>;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum FunctionKind {
    Function,
    Method,
}

impl Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Function => f.write_str("function"),
            Self::Method => f.write_str("method"),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    peeks: PeekNth<I>,
    loop_depth: usize,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new<V>(tokens: V) -> Self
    where
        V: IntoIterator<IntoIter = I>,
    {
        let peeks = itertools::peek_nth(tokens);
        Self {
            peeks,
            loop_depth: 0,
        }
    }

    pub fn parse(&mut self) -> (Vec<Stmts>, bool) {
        let mut had_err = false;
        let mut stmts = Vec::new();
        while self.peeks.peek().is_some() {
            match self.declaration() {
                Ok(v) => stmts.push(v),
                Err(e) => {
                    had_err = true;
                    tracing::error!("{e}");
                },
            }
        }

        (stmts, had_err)
    }

    fn declaration(&mut self) -> Result<Stmts> {
        match self.peeks.peek() {
            Some(Token::Class { .. }) => self.class_declaration(),
            Some(Token::Fun { .. }) => self.function(FunctionKind::Function),
            Some(Token::Var { .. }) => self.var_declaration(),
            _ => match self.statement() {
                stmt @ Ok(_) => stmt,
                Err(e) => {
                    self.synchronize();
                    Err(ParserError::DeclarationStmt(e.to_string()))
                },
            },
        }
    }

    fn var_declaration(&mut self) -> Result<Stmts> {
        let var = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(var, Token::Var { .. }));

        let Some(ident) = self.peeks.next()
        else {
            return Err(ParserError::Eof("Expect a ident after `var`".to_owned()));
        };
        if !matches!(ident, Token::Identifier { .. }) {
            return Err(ParserError::VarDeclaration(ident));
        }

        let Some(next_token) = self.peeks.peek()
        else {
            return Err(ParserError::Eof(format!(
                "Expect `=` or literal value after {ident}"
            )));
        };
        let init_val = match next_token {
            Token::Equal { .. } => {
                self.peeks.next();
                self.expression()?
            },
            _ => Exprs::Literal(Literal::default()),
        };
        match self.peeks.next() {
            Some(Token::Semicolon { .. }) => {},
            Some(v) => return Err(ParserError::Semicolon(v)),
            None => return Err(ParserError::Eof("Expect `;` at end".to_owned())),
        }

        Ok(Stmts::Var(Var::new(ident, Some(init_val))))
    }

    fn statement(&mut self) -> Result<Stmts> {
        let Some(next) = self.peeks.peek()
        else {
            return Err(ParserError::Eof("Expect a statement.".to_owned()));
        };

        match next {
            Token::For { .. } => {
                let stmt = self.for_statement()?;
                Ok(stmt)
            },
            Token::If { .. } => {
                let stmt = self.if_statement()?;
                Ok(stmt)
            },
            Token::Print { .. } => {
                let stmt = self.print_statement()?;
                Ok(stmt)
            },
            Token::Return { .. } => {
                let stmt = self.return_statement()?;
                Ok(stmt)
            },
            Token::While { .. } => {
                let stmt = self.while_statement()?;
                Ok(stmt)
            },
            Token::LeftBrace { .. } => {
                let blk_stmt = self.block()?;
                let stmt = Stmts::Block(Block::new(blk_stmt));
                Ok(stmt)
            },
            Token::Break { .. } => {
                let stmt = self.break_statement()?;
                Ok(stmt)
            },
            _ => self.expression_stmt(),
        }
    }

    fn break_statement(&mut self) -> Result<Stmts> {
        let break_ = unsafe { self.peeks.next().unwrap_unchecked() };
        if self.loop_depth == 0 {
            return Err(ParserError::NotInLoop(break_));
        }
        self.consume_semicolon_paren()?;
        Ok(Stmts::Break(Break::new(break_)))
    }

    fn while_statement(&mut self) -> Result<Stmts> {
        let while_ = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(while_, Token::While { .. }));

        self.consume_left_paren()?;

        let cond = self.expression()?;

        self.consume_rignt_paren()?;
        let res: Result<Stmts> = try {
            self.loop_depth += 1;
            let body = self.statement()?;

            Stmts::While(While::new(cond, body.into()))
        };
        match res {
            r @ Ok(_) => r,
            r @ Err(_) => {
                self.loop_depth -= 1;
                r
            },
        }
    }

    fn if_statement(&mut self) -> Result<Stmts> {
        let if_ = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(if_, Token::If { .. }));

        self.consume_left_paren()?;

        let condition = self.expression()?;

        self.consume_rignt_paren()?;

        let then_branch = self.statement()?;

        let stmts = match self.peeks.peek() {
            Some(Token::Else { .. }) => {
                self.peeks.next();
                let else_branch = self.statement()?;
                Stmts::If(If::new(condition, then_branch.into(), else_branch))
            },
            _ => Stmts::If(If::new(condition, Box::new(then_branch), None)),
        };
        Ok(stmts)
    }

    fn expression_stmt(&mut self) -> Result<Stmts> {
        let expr = self.expression()?;
        match self.peeks.next() {
            Some(Token::Semicolon { .. }) => Ok(Stmts::Expression(Expression::new(expr))),
            Some(tk) => Err(ParserError::Semicolon(tk)),
            None => Err(ParserError::ErrMessage("Not end of `}`".to_owned())),
        }
    }

    fn block(&mut self) -> Result<Vec<Stmts>> {
        let left_brace = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(left_brace, Token::LeftBrace { .. }));

        let mut statements = Vec::new();
        while let Some(tk) = self.peeks.peek()
            && !matches!(tk, Token::RightBrace { .. })
        {
            statements.push(self.declaration()?);
        }
        match self.peeks.next() {
            Some(Token::RightBrace { .. }) => Ok(statements),
            Some(v) => Err(ParserError::RightBrace(v)),
            None => Err(ParserError::ErrMessage("Not end of `}`".to_owned())),
        }
    }

    fn print_statement(&mut self) -> Result<Stmts> {
        let print_ = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(print_, Token::Print { .. }));

        let expr = self.expression()?;
        match self.peeks.next() {
            Some(Token::Semicolon { .. }) => Ok(Stmts::Print(Print::new(expr))),
            Some(v) => Err(ParserError::PrintStmt(v)),
            None => Err(ParserError::ErrMessage("Missing `;` at end".to_owned())),
        }
    }

    fn expression(&mut self) -> Result<Exprs> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Exprs> {
        let expr = self.or()?;
        if !matches!(self.peeks.peek(), Some(Token::Equal { .. })) {
            return Ok(expr);
        }
        let equals = unsafe { self.peeks.next().unwrap_unchecked() };
        let value = self.assignment()?;
        match expr {
            Exprs::Variable(v) => {
                let name = v.into_name();
                Ok(Exprs::Assign(Assign::new(name, value)))
            },
            Exprs::Get(get) => {
                let set = Exprs::Set(Set::new(*get.object, get.name, value));
                Ok(set)
            },
            _ => Err(ParserError::Assign(equals)),
        }
    }
    fn or(&mut self) -> Result<Exprs> {
        let mut expr = self.and()?;

        while let Some(Token::Or { .. }) = self.peeks.peek() {
            let operator = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.and()?;
            expr = Exprs::Logical(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }
    fn and(&mut self) -> Result<Exprs> {
        let mut expr = self.equality()?;

        while let Some(Token::And { .. }) = self.peeks.peek() {
            let operator = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.and()?;
            expr = Exprs::Logical(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Exprs> {
        let mut expr = self.comparison()?;

        while let Some(Token::BangEqual { .. } | Token::EqualEqual { .. }) = self.peeks.peek() {
            let op = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.comparison()?;
            expr = Exprs::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Exprs> {
        let mut expr = self.term()?;

        while let Some(
            Token::Greater { .. }
            | Token::GreaterEqual { .. }
            | Token::Less { .. }
            | Token::LessEqual { .. },
        ) = self.peeks.peek()
        {
            let op = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.term()?;
            expr = Exprs::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Exprs> {
        let mut expr = self.factor()?;

        while let Some(Token::Minus { .. } | Token::Plus { .. }) = self.peeks.peek() {
            let operator = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.factor()?;
            expr = Exprs::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Exprs> {
        let mut expr = self.unary()?;

        while let Some(Token::Slash { .. } | Token::Star { .. }) = self.peeks.peek() {
            let operator = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.unary()?;
            expr = Exprs::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Exprs> {
        if let Some(pk) = self.peeks.peek()
            && matches!(pk, Token::Bang { .. } | Token::Minus { .. })
        {
            let operator = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.unary()?;
            return Ok(Exprs::Unary(Unary::new(operator, right)));
        }

        self.call()
    }

    fn call(&mut self) -> Result<Exprs> {
        let mut expr = self.primary()?;
        while let Some(token) = self.peeks.peek() {
            if matches!(token, Token::LeftParen { .. }) {
                expr = self.finish_call(expr)?;
            }
            else if matches!(token, Token::Dot { .. }) {
                // consume `Dot`
                self.peeks.next();
                let name = self.consume_identifier()?;
                expr = Exprs::Get(Get::new(expr, name));
            }
            else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Exprs) -> Result<Exprs> {
        let left_paren = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(left_paren, Token::LeftParen { .. }));
        let mut args = Vec::new();
        while let Some(tk) = self.peeks.peek()
            && !matches!(tk, Token::RightParen { .. })
        {
            loop {
                if args.len() >= 255 {
                    // return Err(ParserError::TooManyArgs(left_paren));
                    tracing::error!("Can't have more than 255 arguments: {left_paren}");
                }
                args.push(self.expression()?);
                let flag = matches!(self.peeks.peek(), Some(Token::Comma { .. }));
                if !flag {
                    break;
                }
                self.peeks.next();
            }
        }
        self.consume_rignt_paren()?;

        Ok(Exprs::Call(Call::new(callee, left_paren, args)))
    }

    fn primary(&mut self) -> Result<Exprs> {
        match self.peeks.next() {
            Some(pk) => match pk {
                Token::False { .. } => Ok(Exprs::Literal(Literal::new(LiteralType::Bool(false)))),
                Token::True { .. } => Ok(Exprs::Literal(Literal::new(LiteralType::Bool(true)))),
                Token::Nil { .. } => Ok(Exprs::Literal(Literal::new(LiteralType::Nil))),
                Token::Number { double, .. } => {
                    Ok(Exprs::Literal(Literal::new(LiteralType::Number(double))))
                },
                Token::String { mut inner } => Ok(Exprs::Literal(Literal::new(
                    LiteralType::String(inner.lexeme_take()),
                ))),
                sup @ Token::Super { .. } => {
                    let keyword = sup;
                    self.consume_dot()?;
                    let method = self.consume_identifier()?;
                    Ok(Exprs::Super(Super::new(keyword, method)))
                },
                this @ Token::This { .. } => Ok(Exprs::This(This::new(this))),
                tk @ Token::Identifier { .. } => Ok(Exprs::Variable(Variable::new(tk))),
                Token::LeftParen { .. } => {
                    let expr = self.expression()?;
                    self.consume_rignt_paren()?;
                    Ok(Exprs::Grouping(Grouping::new(expr)))
                },
                other => Err(ParserError::Primary(other)),
            },
            None => Err(ParserError::Eof("Expect a primary".to_owned())),
        }
    }

    fn for_statement(&mut self) -> Result<Stmts> {
        let for_ = unsafe { self.peeks.next().unwrap_unchecked() };
        assert!(matches!(for_, Token::For { .. }));

        self.consume_left_paren()?;

        let Some(tk) = self.peeks.peek()
        else {
            return Err(ParserError::Eof(
                "Expect a varDecl or expr or `;`".to_owned(),
            ));
        };

        let initializer = match tk {
            Token::Semicolon { .. } => None,
            Token::Var { .. } => Some(self.var_declaration()?),
            _ => Some(self.expression_stmt()?),
        };

        let Some(tk) = self.peeks.peek()
        else {
            return Err(ParserError::Eof("Expect a condition expr".to_owned()));
        };

        let condition = match tk {
            Token::Semicolon { .. } => None,
            _ => Some(self.expression()?),
        };
        self.consume_semicolon_paren()?;

        let Some(tk) = self.peeks.peek()
        else {
            return Err(ParserError::Eof("Expect a increment expr".to_owned()));
        };

        let increment = match tk {
            Token::RightParen { .. } => None,
            _ => Some(self.expression()?),
        };
        self.consume_rignt_paren()?;

        let res: Result<Stmts> = try {
            self.loop_depth += 1;
            let mut body = self.statement()?;

            if let Some(increment) = increment {
                body = Stmts::Block(Block::new(vec![
                    body,
                    Stmts::Expression(Expression::new(increment)),
                ]));
            }

            match condition {
                Some(cond) => {
                    body = Stmts::While(While::new(cond, body.into()));
                },
                None => {
                    body = Stmts::While(While::new(
                        Exprs::Literal(Literal::new(LiteralType::Bool(true))),
                        body.into(),
                    ));
                },
            }

            if let Some(initializer) = initializer {
                body = Stmts::Block(Block::new(vec![initializer, body]));
            }

            body
        };
        match res {
            r @ Ok(_) => r,
            e @ Err(_) => {
                self.loop_depth -= 1;
                e
            },
        }
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Stmts> {
        let fun = self.peeks.next();
        assert!(matches!(fun, Some(Token::Fun { .. })));
        let name = match self.peeks.next() {
            Some(tk @ Token::Identifier { .. }) => tk,
            Some(other) => return Err(ParserError::CallDecl { tk: other, kind }),
            None => return Err(ParserError::Eof(format!("Expect `{kind}` name"))),
        };
        self.consume_left_paren()?;
        let mut parameters = Vec::new();
        if let Some(pk) = self.peeks.peek()
            && !matches!(pk, Token::RightParen { .. })
        {
            loop {
                if parameters.len() >= 255 {
                    return Err(ParserError::TooManyArgs(name));
                }

                let value = match self.peeks.next() {
                    Some(value @ Token::Identifier { .. }) => value,
                    Some(v) => return Err(ParserError::Parameters(v)),
                    None => return Err(ParserError::Eof("Expect parameters".to_owned())),
                };
                parameters.push(value);

                match self.peeks.peek() {
                    Some(Token::Comma { .. }) => {
                        self.peeks.next();
                    },
                    _ => break,
                }
            }
        }
        self.consume_rignt_paren()?;
        // self.consume_left_brace()?;
        let body = self.block()?;

        Ok(Stmts::Function(Function::new(name, parameters, body)))
    }

    fn return_statement(&mut self) -> Result<Stmts> {
        let keyword = unsafe { self.peeks.next().unwrap_unchecked() };
        let value = if matches!(self.peeks.peek(), Some(Token::Semicolon { .. })) {
            None
        }
        else {
            Some(self.expression()?)
        };
        self.consume_semicolon_paren()?;

        Ok(Stmts::Return(Return::new(keyword, value)))
    }

    fn class_declaration(&mut self) -> Result<Stmts> {
        let class = self.peeks.next();
        assert!(matches!(class, Some(Token::Class { .. })));
        let name = match self.peeks.next() {
            Some(tk @ Token::Identifier { .. }) => tk,
            Some(other) => return Err(ParserError::Class(other)),
            None => return Err(ParserError::Eof("Expect class name".to_owned())),
        };

        let mut superclass = None;

        if let Some(Token::Less { .. }) = self.peeks.peek() {
            self.peeks.next();
            // TODO: use `consume_identifier`
            let tk = match self.peeks.next() {
                Some(tk @ Token::Identifier { .. }) => tk,
                Some(other) => return Err(ParserError::Superclass(other)),
                None => return Err(ParserError::Eof("Expect superclass name".to_owned())),
            };
            superclass = Some(Variable::new(tk));
        }

        self.consume_left_brace()?;

        let mut methods = Vec::new();

        while let Some(next) = self.peeks.peek()
            && !matches!(next, Token::RightBrace { .. })
        {
            let value = self.function(FunctionKind::Method)?;
            match value {
                Stmts::Function(function) => {
                    methods.push(function);
                },
                _ => unreachable!("parser function method return not Function variant"),
            }
        }

        self.consume_rignt_brace()?;

        Ok(Stmts::Class(Class::new(name, superclass, methods)))
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    /// Consume Identifier and get it's name
    fn consume_identifier(&mut self) -> Result<Token> {
        match self.peeks.next() {
            Some(t @ Token::Identifier { .. }) => Ok(t),
            Some(other) => Err(ParserError::LeftParen(other)),
            None => Err(ParserError::Eof("Expect `;`".to_owned())),
        }
    }
    fn consume_dot(&mut self) -> Result<Token> {
        match self.peeks.next() {
            Some(t @ Token::Dot { .. }) => Ok(t),
            Some(other) => Err(ParserError::LeftParen(other)),
            None => Err(ParserError::Eof("Expect `;`".to_owned())),
        }
    }
    fn consume_semicolon_paren(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::Semicolon { .. }) => Ok(()),
            Some(other) => Err(ParserError::LeftParen(other)),
            None => Err(ParserError::Eof("Expect `;`".to_owned())),
        }
    }
    /// Expect `Token::LeftParen`, (
    fn consume_left_paren(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::LeftParen { .. }) => Ok(()),
            Some(other) => Err(ParserError::LeftParen(other)),
            None => Err(ParserError::Eof("Expect `(`".to_owned())),
        }
    }
    /// Expect `Token::LeftBrace`, {
    fn consume_left_brace(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::LeftBrace { .. }) => Ok(()),
            Some(other) => Err(ParserError::LeftBrace(other)),
            None => Err(ParserError::Eof("Expect `{`".to_owned())),
        }
    }

    /// Expect `Token::RightParen`, )
    fn consume_rignt_paren(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::RightParen { .. }) => Ok(()),
            Some(other) => Err(ParserError::RightParen(other)),
            None => Err(ParserError::Eof("Expect `)`".to_owned())),
        }
    }
    /// Expect `Token::RightBrace`, }
    fn consume_rignt_brace(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::RightBrace { .. }) => Ok(()),
            Some(other) => Err(ParserError::RightBrace(other)),
            None => Err(ParserError::Eof("Expect `}`".to_owned())),
        }
    }

    /// when want discard tokens until we're right at the beginning of the next statment
    fn synchronize(&mut self) {
        self.peeks.next();
        while let Some(pk) = self.peeks.peek() {
            match pk {
                Token::Class { .. }
                | Token::Fun { .. }
                | Token::Var { .. }
                | Token::For { .. }
                | Token::If { .. }
                | Token::While { .. }
                | Token::Print { .. }
                | Token::Return { .. } => return,
                _ => self.peeks.next(),
            };
        }
    }
}
