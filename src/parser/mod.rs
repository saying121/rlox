#[cfg(test)]
mod tests;

use itertools::PeekNth;
use thiserror::Error;

use crate::{
    expr::{Binary, Exprs, Grouping, Literal, LiteralType, Unary},
    tokens::Token,
};

#[derive(Clone)]
#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Missing ')' after expression: {0}")]
    RightParen(Token),
    #[error("End of source code, no next token")]
    Eof,
    #[error("Invalid Primary: {0}")]
    Primary(Token),
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
    pub fn parse(&mut self) -> Result<Exprs> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Exprs> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Exprs> {
        let mut expr = self.comparison()?;

        while let Some(op) = self.peeks.next()
            && matches!(op, Token::BangEqual { .. } | Token::EqualEqual { .. })
        {
            let right = self.comparison()?;
            expr = Exprs::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
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

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
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

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
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

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
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

    #[expect(dead_code, reason = "todo")]
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
