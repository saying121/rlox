use std::hint::unreachable_unchecked;

use itertools::PeekNth;
use rlox::token::Token;

use crate::{
    chunk::{Chunk, OpCode},
    error,
    error::Result,
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
            if !matches!(self.current, None | Some(Token::Invalid { .. })) {
                break;
            }
            self.error_at_current();
            // match &self.current {
            //     Some(Token::Invalid { inner }) => {
            //         if self.panic_mode {
            //             return;
            //         }
            //         self.panic_mode = true;
            //         self.had_error = true;
            //         tracing::error!("error: {}", inner);
            //     },
            //     None => {
            //         if self.panic_mode {
            //             return;
            //         }
            //         self.panic_mode = true;
            //         self.had_error = true;
            //         tracing::error!(" at end");
            //     },
            //     _ => break,
            // }
        }
    }

    // TODO
    fn consume(&mut self, msg: &str) {
        if matches!(self.current, Some(Token::LeftParen { .. })) {
            self.advance();
            return;
        }
    }

    fn emit_byte(&self, byte: u8, cur_chunk: &mut Chunk) {
        let (row, col) = unsafe { self.previous.as_ref().unwrap_unchecked().inner().get_xy() };
        cur_chunk.write(byte, row);
    }

    fn end_compiler(&self, cur_chunk: &mut Chunk) {
        self.emit_return(cur_chunk);
    }

    fn emit_return(&self, cur_chunk: &mut Chunk) {
        self.emit_byte(OpCode::OpReturn.into(), cur_chunk);
    }

    fn emit_bytes(&self, cur_chunk: &mut Chunk, byte1: u8, byte2: u8) {
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
