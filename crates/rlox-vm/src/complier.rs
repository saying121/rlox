use std::{convert::Into, hint::unreachable_unchecked};

use itertools::PeekNth;
use rlox::token::Token;

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
    cur_chunk: Option<Chunk>,
    had_error: bool,
    panic_mode: bool,
}

type ParseFn<I> = for<'a> fn(&'a mut Parser<I>) -> Result<()>;

pub struct ParseRule<I>
where
    I: Iterator<Item = Token>,
{
    prefix: Option<ParseFn<I>>,
    infix: Option<ParseFn<I>>,
    precedence: Precedence,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
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

#[expect(clippy::match_same_arms, reason = "align")]
pub fn get_rule<I>(typ: &Token) -> ParseRule<I>
where
    I: Iterator<Item = Token>,
{
    match typ {
        Token::LeftParen { .. } => ParseRule {
            prefix: Some(Parser::grouping),
            infix: None,
            precedence: Precedence::None,
        },
        Token::RightParen { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::LeftBrace { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::RightBrace { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Comma { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Dot { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Minus { .. } => ParseRule {
            prefix: Some(Parser::unary),
            infix: Some(Parser::binary),
            precedence: Precedence::Term,
        },
        Token::Plus { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Term,
        },
        Token::Semicolon { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Slash { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Factor,
        },
        Token::Star { .. } => ParseRule {
            prefix: None,
            infix: Some(Parser::binary),
            precedence: Precedence::Factor,
        },
        Token::Bang { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::BangEqual { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Equal { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::EqualEqual { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Greater { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::GreaterEqual { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Less { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::LessEqual { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Identifier { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::String { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Number { .. } => ParseRule {
            prefix: Some(Parser::number),
            infix: None,
            precedence: Precedence::None,
        },
        Token::And { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Class { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Else { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Fun { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::For { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::If { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Nil { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Or { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Print { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Return { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Super { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::This { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::True { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::False { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Var { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::While { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Eof { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Comment { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::BlockComment { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Break { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
        Token::Invalid { .. } => ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        },
    }
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
            cur_chunk: None,
        }
    }
    pub fn compile(&mut self, cur_chunk: Chunk) -> Result<Chunk> {
        self.cur_chunk = Some(cur_chunk);
        self.end_compiler();
        if self.had_error {
            return error::CompileSnafu.fail();
        }
        unsafe { Ok(self.cur_chunk.take().unwrap_unchecked()) }
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

    fn previous(&self) -> Result<&Token> {
        let Some(prev) = &self.previous
        else {
            return error::MissingPrevSnafu.fail();
        };
        Ok(prev)
    }

    fn current(&self) -> Result<&Token> {
        let Some(prev) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        Ok(prev)
    }

    fn expression(&mut self) -> Result<()> {
        self.parse_precedence(Precedence::Assignment)
    }

    fn number(&mut self) -> Result<()> {
        let prev = self.previous()?;

        let value: f64 = unsafe { prev.inner().lexeme().parse().unwrap_unchecked() };
        self.emit_constant(Value(value))?;
        Ok(())
    }

    fn unary(&mut self) -> Result<()> {
        let operator_type = self.previous()?.clone();
        self.parse_precedence(Precedence::Unary)?;
        // self.expression();
        match operator_type {
            Token::Minus { .. } => {
                self.emit_byte(OpCode::OpNegate);
            },
            _ => unreachable!(),
        }
        Ok(())
    }

    fn binary(&mut self) -> Result<()> {
        let op_type = self.previous()?.clone();
        let rule: ParseRule<I> = get_rule(&op_type);
        self.parse_precedence((Into::<u8>::into(rule.precedence) + 1_u8).into())?;
        match op_type {
            Token::Plus { .. } => self.emit_byte(OpCode::OpAdd),
            Token::Minus { .. } => self.emit_byte(OpCode::OpSubtract),
            Token::Star { .. } => self.emit_byte(OpCode::OpMultiply),
            Token::Slash { .. } => self.emit_byte(OpCode::OpDivide),
            _ => unsafe { unreachable_unchecked() },
        }
        Ok(())
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Result<()> {
        self.advance();
        let prefix_fule = get_rule(self.previous()?).prefix;
        prefix_fule.map_or_else(|| error::NotExpressionSnafu.fail(), |p| p(self))?;

        while precedence <= get_rule::<I>(self.current()?).precedence {
            self.advance();
            let Some(infix_rule) = get_rule(self.previous()?).infix
            else {
                return error::MissingInfixSnafu.fail();
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
        let constant = self.cur_chunk().add_constant(value);
        if constant > u8::MAX.into() {
            return error::TooManyConstsSnafu.fail();
        }
        Ok(constant as u8)
    }

    fn emit_byte<B: Into<u8>>(&mut self, byte: B) {
        let (row, _col) = unsafe { self.previous.as_ref().unwrap_unchecked().inner().get_xy() };
        self.cur_chunk().write(byte, row);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn grouping(&mut self) -> Result<()> {
        // self.consume_left_paren();
        self.expression()?;
        self.consume_right_paren()
    }

    fn cur_chunk(&mut self) -> &mut Chunk {
        self.cur_chunk.as_mut().expect("Not input chunk")
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
