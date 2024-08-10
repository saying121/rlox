use std::vec::IntoIter;

use anyhow::{bail, Result};
use itertools::PeekNth;
use thiserror::Error;

use crate::{
    expr::{Binary, Exprs, Grouping, Literal, Unary},
    tokens::{Nil, Token},
};

#[derive(Clone)]
#[derive(Debug, Error)]
enum ParserError {
    #[error("missing ')' after expression: {0}")]
    RightParen(Token),
    #[error("at file end")]
    Eof,
    #[error("invalid Primary: {0}")]
    Primary(Token),
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    peeks: PeekNth<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let peeks = itertools::peek_nth(tokens.clone());
        Self { tokens, peeks }
    }
    pub fn parse(&mut self) -> Result<Exprs> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Exprs> {
        self.equality()
    }

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
    fn equality(&mut self) -> Result<Exprs> {
        let mut expr = self.comparison()?;

        while let Some(pk) = self.peeks.peek()
            && matches!(pk, Token::BangEqual { .. } | Token::EqualEqual { .. })
        {
            let operator = self.peeks.next().expect("Should not panic");
            let right = self.comparison()?;
            expr = Exprs::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
    fn comparison(&mut self) -> Result<Exprs> {
        let mut expr = self.term()?;

        while let Some(pk) = self.peeks.peek()
            && matches!(
                pk,
                Token::Greater { .. }
                    | Token::GreaterEqual { .. }
                    | Token::Less { .. }
                    | Token::LessEqual { .. }
            )
        {
            let operator = self.peeks.next().expect("Should not panic");
            let right = self.term()?;
            expr = Exprs::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
    fn term(&mut self) -> Result<Exprs> {
        let mut expr = self.factor()?;

        while let Some(pk) = self.peeks.peek()
            && matches!(pk, Token::Minus { .. } | Token::Plus { .. })
        {
            let operator = self.peeks.next().expect("Should not panic");
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
            let operator = self.peeks.next().expect("Should not panic");
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
            let operator = self.peeks.next().expect("Should not panic");
            let right = self.unary()?;
            return Ok(Exprs::Unary(Unary::new(operator, right)));
        }

        self.primary()
    }

    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
    fn primary(&mut self) -> Result<Exprs> {
        match self.peeks.peek() {
            Some(pk) => match pk {
                Token::False { .. } => {
                    self.peeks.next().expect("Should not panic");
                    Ok(Exprs::Literal(Literal::new(false)))
                },
                Token::True { .. } => {
                    self.peeks.next().expect("Should not panic");
                    Ok(Exprs::Literal(Literal::new(true)))
                },
                Token::Nil { .. } => {
                    self.peeks.next().expect("Should not panic");
                    Ok(Exprs::Literal(Literal::new(Nil)))
                },
                Token::Number { .. } => {
                    let next = self.peeks.next().expect("Should not panic");
                    Ok(Exprs::Literal(Literal::new(
                        next.inner()
                            .lexeme()
                            .parse::<f64>()
                            .expect("parse number failed"),
                    )))
                },
                Token::String { .. } => {
                    let next = self.peeks.next().expect("Should not panic");
                    Ok(Exprs::Literal(Literal::new(
                        next.inner().lexeme().to_owned(),
                    )))
                },
                Token::LeftParen { .. } => {
                    let expr = self.expression()?;
                    self.consume_rignt_paren()?;
                    Ok(Exprs::Grouping(Grouping::new(expr)))
                },
                other => {
                    tracing::error!("{}", other);
                    bail!(ParserError::Primary(other.clone()))
                },
            },
            None => {
                tracing::error!("End of file, no next token.");
                bail!(ParserError::Eof)
            },
        }
    }

    /// Expect `Token::RightParen`
    #[expect(clippy::unwrap_in_result, reason = "had checked previous")]
    fn consume_rignt_paren(&mut self) -> Result<Token> {
        match self.peeks.peek() {
            Some(pk) => match pk {
                Token::RightParen { .. } => Ok(self.peeks.next().expect("Should not panic")),
                other => {
                    tracing::error!("{}", other);
                    bail!(ParserError::RightParen(other.clone()))
                },
            },
            None => {
                tracing::error!("End of file, no next token.");
                bail!(ParserError::Eof)
            },
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
                _ => {
                    self.peeks.next();
                },
            }
        }
    }
}
