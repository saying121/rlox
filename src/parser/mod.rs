#[cfg(test)]
mod tests;

use itertools::PeekNth;
use thiserror::Error;

use crate::{
    expr::{Assign, Binary, Exprs, Grouping, Literal, LiteralType, Logical, Unary, Variable},
    stmt::{Block, Expression, If, Print, Stmts, Var, While},
    tokens::Token,
};

#[derive(Clone)]
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Missing '(' after expression: {0}")]
    LeftParen(Token),
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
    #[error("{0}")]
    ErrMessage(String),
}
pub type Result<T, E = ParserError> = core::result::Result<T, E>;

#[derive(Clone)]
#[derive(Debug)]
pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    peeks: PeekNth<I>,
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
        Self { peeks }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmts>> {
        let mut stmts = Vec::new();
        while self.peeks.peek().is_some() {
            stmts.push(self.declaration()?);
        }

        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmts> {
        match self.peeks.peek() {
            Some(Token::Var { .. }) => {
                self.peeks.next();
                self.var_declaration()
            },
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

        Ok(Stmts::Var(Var::new(ident, init_val)))
    }

    fn statement(&mut self) -> Result<Stmts> {
        let Some(next) = self.peeks.peek()
        else {
            return Err(ParserError::Eof("Expect a statement.".to_owned()));
        };

        match next {
            Token::For { .. } => {
                self.peeks.next();
                let stmt = self.for_statement()?;
                Ok(stmt)
            },
            Token::If { .. } => {
                self.peeks.next();
                let stmt = self.if_statement()?;
                Ok(stmt)
            },
            Token::Print { .. } => {
                self.peeks.next();
                let stmt = self.print_statement()?;
                Ok(stmt)
            },
            Token::While { .. } => {
                self.peeks.next();
                let stmt = self.while_statement()?;
                Ok(stmt)
            },
            Token::LeftBrace { .. } => {
                self.peeks.next();
                let stmt = Stmts::Block(Block::new(self.block()?));
                Ok(stmt)
            },
            _ => self.expression_stmt(),
        }
    }

    fn while_statement(&mut self) -> Result<Stmts> {
        self.consume_left_paren()?;

        let cond = self.expression()?;

        self.consume_rignt_paren()?;
        let body = self.statement()?;

        Ok(Stmts::While(While::new(cond, body.into())))
    }

    fn if_statement(&mut self) -> Result<Stmts> {
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
        match self.peeks.peek() {
            Some(Token::Equal { .. }) => {
                let equals = unsafe { self.peeks.next().unwrap_unchecked() };
                let value = self.assignment()?;
                match expr {
                    Exprs::Variable(v) => {
                        let name = v.name;
                        Ok(Exprs::Assign(Assign::new(name, value)))
                    },
                    _ => Err(ParserError::Assign(equals)),
                }
            },
            _ => Ok(expr),
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

        self.primary()
    }

    fn primary(&mut self) -> Result<Exprs> {
        match self.peeks.next() {
            Some(pk) => match pk {
                Token::False { .. } => Ok(Exprs::Literal(Literal {
                    value: LiteralType::Bool(false),
                })),
                Token::True { .. } => Ok(Exprs::Literal(Literal {
                    value: LiteralType::Bool(true),
                })),
                Token::Nil { .. } => Ok(Exprs::Literal(Literal {
                    value: LiteralType::Nil,
                })),
                Token::Number { double, .. } => Ok(Exprs::Literal(Literal {
                    value: LiteralType::Number(double),
                })),
                Token::String { mut inner } => Ok(Exprs::Literal(Literal {
                    value: LiteralType::String(inner.lexeme_take()),
                })),
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
        self.consume_left_paren()?;

        let Some(tk) = self.peeks.peek()
        else {
            return Err(ParserError::Eof(
                "Expect a varDecl or expr or `;`".to_owned(),
            ));
        };

        let initializer = match tk {
            Token::Semicolon { .. } => None,
            Token::Var { .. } => {
                self.peeks.next();
                Some(self.var_declaration()?)
            },
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
                    Exprs::Literal(Literal {
                        value: LiteralType::Bool(true),
                    }),
                    body.into(),
                ));
            },
        }

        if let Some(initializer) = initializer {
            body = Stmts::Block(Block::new(vec![initializer, body]));
        }

        Ok(body)
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
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

    /// Expect `Token::RightParen`, )
    fn consume_rignt_paren(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::RightParen { .. }) => Ok(()),
            Some(other) => Err(ParserError::RightParen(other)),
            None => Err(ParserError::Eof("Expect `)`".to_owned())),
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
