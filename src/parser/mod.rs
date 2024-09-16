#[cfg(test)]
mod tests;

use itertools::PeekNth;
use thiserror::Error;

use crate::{
    expr::{Assign, Binary, Exprs, Grouping, Literal, LiteralType, Unary, Variable},
    stmt::{Block, Expression, Print, Stmts, Var},
    tokens::Token,
};

#[derive(Clone)]
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Missing ')' after expression: {0}")]
    RightParen(Token),
    #[error("Missing '}}' after expression: {0}")]
    RightBrace(Token),
    #[error("End of source code, no next token")]
    Eof,
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

    pub fn parse(&mut self) -> Vec<Stmts> {
        let mut stmts = Vec::new();
        while self.peeks.peek().is_some() {
            // stmts.push(self.statement()?);
            match self.declaration() {
                Ok(stmt) => {
                    stmts.push(stmt);
                },
                Err(e) => tracing::error!("{e}"),
            }
        }

        stmts
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
            return Err(ParserError::Eof);
        };
        if !matches!(ident, Token::Identifier { .. }) {
            return Err(ParserError::VarDeclaration(ident));
        }

        let Some(next_token) = self.peeks.peek()
        else {
            return Err(ParserError::Eof);
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
            None => return Err(ParserError::Eof),
        }

        Ok(Stmts::Var(Var::new(ident, init_val)))
    }

    fn statement(&mut self) -> Result<Stmts> {
        let Some(next) = self.peeks.peek()
        else {
            return Err(ParserError::Eof);
        };

        match next {
            Token::Print { .. } => {
                self.peeks.next();
                let stmt = self.print_statement()?;
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
        let expr = self.equality()?;
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

    fn equality(&mut self) -> Result<Exprs> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.peeks.peek()
            && matches!(op, Token::BangEqual { .. } | Token::EqualEqual { .. })
        {
            let op = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.comparison()?;
            expr = Exprs::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Exprs> {
        let mut expr = self.term()?;

        while let Some(op) = self.peeks.peek()
            && matches!(
                op,
                Token::Greater { .. }
                    | Token::GreaterEqual { .. }
                    | Token::Less { .. }
                    | Token::LessEqual { .. }
            )
        {
            let op = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.term()?;
            expr = Exprs::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Exprs> {
        let mut expr = self.factor()?;

        while let Some(pk) = self.peeks.peek()
            && matches!(pk, Token::Minus { .. } | Token::Plus { .. })
        {
            let operator = unsafe { self.peeks.next().unwrap_unchecked() };
            let right = self.factor()?;
            expr = Exprs::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Exprs> {
        let mut expr = self.unary()?;

        while let Some(pk) = self.peeks.peek()
            && matches!(pk, Token::Slash { .. } | Token::Star { .. })
        {
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
            None => Err(ParserError::Eof),
        }
    }

    /// Expect `Token::RightParen`
    fn consume_rignt_paren(&mut self) -> Result<Token> {
        self.peeks.next().map_or_else(
            || Err(ParserError::Eof),
            |pk| match pk {
                Token::RightParen { inner } => Ok(Token::RightParen { inner }),
                other => Err(ParserError::RightParen(other)),
            },
        )
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
