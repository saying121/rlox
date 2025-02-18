pub mod rule;
pub mod state;

use std::{convert::Into, hint::unreachable_unchecked};

use itertools::PeekNth;
use rlox::token::Token;

use self::{
    rule::{ParseRule, get_rule},
    state::{CompileState, Compiling, Init},
};
use crate::{
    chunk::{Chunk, OpCode},
    error::{self, Result},
    object::Obj,
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
pub struct Parser<I, S>
where
    I: Iterator<Item = Token>,
    S: CompileState,
{
    peeks: PeekNth<I>,
    previous: Option<Token>,
    current: Option<Token>,
    cur_chunk: S::Data,
    had_error: bool,
    panic_mode: bool,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}
impl From<u8> for Precedence {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute::<u8, Self>(value) }
    }
}

impl From<Precedence> for u8 {
    fn from(val: Precedence) -> Self {
        val as Self
    }
}
impl<I> Parser<I, Compiling>
where
    I: Iterator<Item = Token>,
{
    fn grouping(&mut self) -> Result<()> {
        // self.consume_left_paren();
        self.expression()?;
        self.consume_right_paren()
    }

    fn end_compiler(&mut self) {
        self.emit_return();

        #[cfg(debug_assertions)]
        if !self.had_error {
            self.cur_chunk.disassemble("code");
        }
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn emit_bytes<B1, B2>(&mut self, byte1: B1, byte2: B2)
    where
        B1: Into<u8>,
        B2: Into<u8>,
    {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_byte<B: Into<u8>>(&mut self, byte: B) {
        let (row, _col) = self
            .previous
            .as_ref()
            .expect("Missing previous token")
            .inner()
            .get_xy();
        self.cur_chunk.write(byte, row);
    }

    fn number(&mut self) -> Result<()> {
        let Some(prev) = self.previous.clone()
        else {
            return error::MissingPrevSnafu.fail();
        };
        let num = match prev {
            Token::Number { double, .. } => double,
            _ => unsafe { unreachable_unchecked() },
        };

        self.emit_constant(Value::Number(num))?;
        Ok(())
    }
    fn string(&mut self) -> Result<()> {
        let Some(previous) = &self.previous
        else {
            return error::MissingPrevSnafu.fail();
        };
        self.emit_constant(Value::Obj(Obj::String(previous.lexeme().to_owned())))
    }

    fn unary(&mut self) -> Result<()> {
        let Some(operator_type) = self.previous.clone()
        else {
            return error::MissingPrevSnafu.fail();
        };
        self.parse_precedence(Precedence::Unary)?;
        // self.expression();
        match operator_type {
            Token::Minus { .. } => self.emit_byte(OpCode::OpNegate),
            Token::Bang { .. } => self.emit_byte(OpCode::OpNot),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn binary(&mut self) -> Result<()> {
        let Some(op_type) = self.previous.clone()
        else {
            return error::MissingPrevSnafu.fail();
        };
        let rule: ParseRule<I, Compiling> = get_rule(&op_type);
        self.parse_precedence((Into::<u8>::into(rule.precedence) + 1_u8).into())?;
        match op_type {
            Token::BangEqual { .. } => self.emit_bytes(OpCode::OpEqual, OpCode::OpNot),
            Token::EqualEqual { .. } => self.emit_byte(OpCode::OpEqual),
            Token::Greater { .. } => self.emit_byte(OpCode::OpGreater),
            Token::GreaterEqual { .. } => self.emit_bytes(OpCode::OpLess, OpCode::OpNot),
            Token::Less { .. } => self.emit_byte(OpCode::OpLess),
            Token::LessEqual { .. } => self.emit_bytes(OpCode::OpGreater, OpCode::OpNot),
            Token::Plus { .. } => self.emit_byte(OpCode::OpAdd),
            Token::Minus { .. } => self.emit_byte(OpCode::OpSubtract),
            Token::Star { .. } => self.emit_byte(OpCode::OpMultiply),
            Token::Slash { .. } => self.emit_byte(OpCode::OpDivide),
            _ => unsafe { unreachable_unchecked() },
        }
        Ok(())
    }

    fn literal(&mut self) -> Result<()> {
        let Some(token) = &self.previous
        else {
            return error::MissingPrevSnafu.fail();
        };
        match token {
            Token::False { .. } => self.emit_byte(OpCode::OpFalse),
            Token::Nil { .. } => self.emit_byte(OpCode::OpNil),
            Token::True { .. } => self.emit_byte(OpCode::OpTrue),
            _ => {},
        }
        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<()> {
        self.advance();
        let Some(typ) = &self.previous
        else {
            return error::MissingPrevSnafu.fail();
        };
        let prefix_fule = get_rule(typ).prefix;
        prefix_fule.map_or_else(|| error::NotExpressionSnafu.fail(), |p| p(self))?;

        'l: while precedence
            <= get_rule::<I>(match &self.current {
                Some(t) => t,
                None => break 'l,
            })
            .precedence
        {
            self.advance();
            let Some(infix_rule) = get_rule(match &self.previous {
                Some(p) => p,
                None => return error::MissingPrevSnafu.fail(),
            })
            .infix
            else {
                break;
                // return error::MissingInfixSnafu.fail();
            };
            infix_rule(self)?;
        }
        Ok(())
    }

    fn emit_constant(&mut self, value: Value) -> Result<()> {
        let byte2 = self.make_constant(value)?;
        self.emit_bytes(OpCode::OpConstant, byte2);
        Ok(())
    }

    fn make_constant(&mut self, value: Value) -> Result<u8> {
        let constant = self.cur_chunk.add_constant(value);
        if constant > u8::MAX.into() {
            return error::TooManyConstsSnafu.fail();
        }
        Ok(constant as u8)
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn declaration(&mut self) -> Result<()> {
        self.statement()?;
        if self.panic_mode {
            self.synchronize();
        }
        Ok(())
    }

    fn statement(&mut self) -> Result<()> {
        let Some(cur) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };

        match cur {
            Token::Print { .. } => {
                self.advance();
                self.print_statement()?;
            },
            _ => {
                self.expression_statement()?;
            },
        }
        Ok(())
    }

    fn print_statement(&mut self) -> Result<()> {
        self.expression()?;
        self.consume_semicolon()?;
        self.emit_byte(OpCode::OpPrint);
        Ok(())
    }

    fn expression_statement(&mut self) -> Result<()> {
        self.expression()?;
        self.consume_semicolon()?;
        self.emit_byte(OpCode::OpPop);
        Ok(())
    }
}

impl<I> Parser<I, Init>
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
            cur_chunk: (),
        }
    }

    pub fn compile(self, cur_chunk: Chunk) -> Result<Chunk> {
        let mut var: Parser<I, Compiling> = Parser {
            peeks: self.peeks,
            previous: self.previous,
            current: self.current,
            cur_chunk,
            had_error: self.had_error,
            panic_mode: self.panic_mode,
        };
        var.advance();
        while var.current.is_some() {
            var.declaration()?;
        }
        var.end_compiler();
        if var.had_error {
            return error::CompileSnafu.fail();
        }
        Ok(var.cur_chunk)
    }
}

impl<I, S> Parser<I, S>
where
    I: Iterator<Item = Token>,
    S: CompileState,
{
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

    fn error(&mut self) {
        let token = self.previous.clone();
        self.error_at(token);
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while let Some(cur) = &self.current {
            if matches!(self.current, Some(Token::Semicolon { .. })) {
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

    fn error_at_current(&mut self) {
        let token = self.current.clone();
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

    fn consume_right_paren(&mut self) -> Result<()> {
        let Some(tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        match tk {
            Token::RightParen { .. } => {
                self.advance();
                Ok(())
            },
            t => error::NotMatchSnafu {
                msg: "Expect ')' after expression",
                token: Some(t.clone()),
            }
            .fail(),
        }
    }

    fn consume_semicolon(&mut self) -> Result<()> {
        let Some(tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        match tk {
            Token::Semicolon { .. } => {
                self.advance();
                Ok(())
            },
            t => error::NotMatchSnafu {
                msg: "Expect ';' after value",
                token: Some(t.clone()),
            }
            .fail(),
        }
    }
}
