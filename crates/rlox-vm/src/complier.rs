use std::hint::unreachable_unchecked;

use itertools::PeekNth;
use rlox::token::{self, Token};

use crate::{
    chunk::{Chunk, OpCode},
    error::{self, Result},
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    peeks: PeekNth<I>,
    previous: Option<Token>,
    current: Option<Token>,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    None,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
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
            previous: None,
            current: None,
            had_error: false,
            panic_mode: false,
        }
    }
    pub fn compile(&self, cur_chunk: &mut Chunk) -> Result<()> {
        self.end_compiler(cur_chunk);
        if self.had_error {
            return error::CompileSnafu.fail();
        }
        Ok(())
    }

    fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            self.current = self.peeks.next();
            if !matches!(self.current, Some(Token::Invalid { .. })) {
                break;
            }
            self.error_at_current();
        }
    }

    fn expression(&self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn number(&self, cur_chunk: &mut Chunk) {
        let value: f64 = unsafe {
            self.previous
                .as_ref()
                .unwrap_unchecked()
                .inner()
                .lexeme()
                .parse()
                .unwrap_unchecked()
        };
        self.emit_constant(Value(value), cur_chunk);
    }

    fn unary(&mut self, cur_chunk: &mut Chunk) -> Result<()> {
        let operator_type = self.previous.as_ref().expect("missing previous");
        self.parse_precedence(Precedence::Unary);
        // self.expression();
        match operator_type {
            Token::Minus { .. } => {
                self.emit_byte(OpCode::OpNegate, cur_chunk);
            },
            _ => unreachable!(),
        }
        Ok(())
    }

    fn binary(&self, cur_chunk: &mut Chunk) {
        let op_type = unsafe { self.previous.as_ref().unwrap_unchecked() };
        // let rule  = get_rule();
        // self.parse_precedence(rule+1);
        match op_type {
            Token::Plus { .. } => self.emit_byte(OpCode::OpAdd, cur_chunk),
            Token::Minus { .. } => self.emit_byte(OpCode::OpSubtract, cur_chunk),
            Token::Star { .. } => self.emit_byte(OpCode::OpMultiply, cur_chunk),
            Token::Slash { .. } => self.emit_byte(OpCode::OpDivide, cur_chunk),
            _ => unsafe { unreachable_unchecked() },
        }
    }

    fn parse_precedence(&self, precedence: Precedence) {
        unimplemented!()
    }

    fn emit_constant(&self, value: Value, cur_chunk: &mut Chunk) -> Result<()> {
        let byte2 = Self::make_constant(value, cur_chunk)?;
        self.emit_bytes(cur_chunk, OpCode::OpConstant, byte2);
        Ok(())
    }

    fn make_constant(value: Value, cur_chunk: &mut Chunk) -> Result<u8> {
        let constant = cur_chunk.add_constant(value);
        if constant > u8::MAX.into() {
            return error::TooManyConstsSnafu.fail();
        }
        Ok(constant as u8)
    }

    fn emit_byte<B: Into<u8>>(&self, byte: B, cur_chunk: &mut Chunk) {
        let (row, _col) = unsafe { self.previous.as_ref().unwrap_unchecked().inner().get_xy() };
        cur_chunk.write(byte, row);
    }

    fn end_compiler(&self, cur_chunk: &mut Chunk) {
        self.emit_return(cur_chunk);
    }

    fn grouping(&mut self) -> Result<()> {
        // self.consume_left_paren();
        self.expression();
        self.consume_right_paren()
    }

    fn emit_return(&self, cur_chunk: &mut Chunk) {
        self.emit_byte(OpCode::OpReturn, cur_chunk);
    }

    fn emit_bytes<B1, B2>(&self, cur_chunk: &mut Chunk, byte1: B1, byte2: B2)
    where
        B1: Into<u8>,
        B2: Into<u8>,
    {
        self.emit_byte(byte1, cur_chunk);
        self.emit_byte(byte2, cur_chunk);
    }

    fn error_at_current(&mut self) {
        let token = self.current.clone();
        self.error_at(token);
    }

    fn error(&mut self) {
        let token = self.previous.clone();
        self.error_at(token);
    }

    fn error_at(&mut self, token: Option<Token>) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        match token {
            Some(Token::Invalid { inner }) => tracing::error!("Error at: {}", inner),
            Some(_) => unsafe { unreachable_unchecked() },
            None => tracing::error!(" at end"),
        }
        self.had_error = true;
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        self.peeks.next();
        while let Some(cur) = &self.current {
            if matches!(self.previous, Some(Token::Semicolon { .. })) {
                return;
            }
            match cur {
                Token::Class { .. }
                | Token::Fun { .. }
                | Token::Var { .. }
                | Token::For { .. }
                | Token::If { .. }
                | Token::While { .. }
                | Token::Print { .. }
                | Token::Return { .. } => return,
                _ => {},
            };
            self.advance();
        }
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn consume_left_paren(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::LeftParen { .. }) => Ok(()),
            Some(t) => error::NotMatchSnafu {
                msg: "Expect '(' after expression",
                token: Some(t),
            }
            .fail(),
            None => error::NotMatchSnafu {
                msg: "Expect ')' after expression",
                token: None,
            }
            .fail(),
        }
    }
    fn consume_right_paren(&mut self) -> Result<()> {
        match self.peeks.next() {
            Some(Token::RightParen { .. }) => Ok(()),
            Some(t) => error::NotMatchSnafu {
                msg: "Expect ')' after expression",
                token: Some(t),
            }
            .fail(),
            None => error::NotMatchSnafu {
                msg: "Expect ')' after expression",
                token: None,
            }
            .fail(),
        }
    }
}
