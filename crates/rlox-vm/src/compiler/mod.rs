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
    cur_compiler: Compiler,
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

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Local {
    name: Token,
    depth: i32,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Compiler {
    locals: Vec<Local>,
    scope_depth: usize,
}

impl Compiler {
    pub const fn new() -> Self {
        Self {
            locals: vec![],
            // locals: vec![
            //     Local {
            //         name: Token::Eof {
            //             inner: TokenInner::default()
            //         },
            //         depth: 0
            //     };
            //     u8::MAX.into()
            // ],
            scope_depth: 0,
        }
    }
}

impl<I> Parser<I, Compiling>
where
    I: Iterator<Item = Token>,
{
    fn grouping(&mut self, _: bool) -> Result<()> {
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

    fn number(&mut self, _: bool) -> Result<()> {
        let Some(prev) = self.previous.clone()
        else {
            return error::MissingPrevSnafu.fail();
        };
        let Token::Number { double: num, .. } = prev
        else {
            unsafe { unreachable_unchecked() }
        };

        self.emit_constant(Value::Number(num))?;
        Ok(())
    }
    fn string(&mut self, _: bool) -> Result<()> {
        let Some(previous) = &self.previous
        else {
            return error::MissingPrevSnafu.fail();
        };
        self.emit_constant(Value::Obj(Obj::String(previous.lexeme().to_owned())))
    }

    fn variable(&mut self, can_assign: bool) -> Result<()> {
        let Some(name) = self.previous.take()
        else {
            return error::MissingPrevSnafu.fail();
        };
        self.named_variable(&name, can_assign)
    }

    fn named_variable(&mut self, name: &Token, can_assign: bool) -> Result<()> {
        let (get_op, set_op);
        let arg = self.resolve_local(name)?;
        let arg = if arg == -1 {
            get_op = OpCode::OpGetGlobal;
            set_op = OpCode::OpSetGlobal;
            self.make_constant(Value::Obj(Obj::String(name.lexeme().to_owned())))?
        }
        else {
            get_op = OpCode::OpGetLocal;
            set_op = OpCode::OpSetLocal;
            arg as u8
        };

        if can_assign && matches!(self.current, Some(Token::Equal { .. })) {
            self.advance();
            self.expression()?;
            self.emit_bytes(set_op, arg);
        }
        else {
            self.emit_bytes(get_op, arg);
        }
        Ok(())
    }
    fn resolve_local(&self, name: &Token) -> Result<i32> {
        for (i, ele) in self.cur_compiler.locals.iter().rev().enumerate() {
            if ele.name.lexeme() == name.lexeme() {
                if ele.depth == -1 {
                    return error::OwnInitSnafu {
                        name: ele.name.clone(),
                    }
                    .fail();
                }
                return Ok(i as i32);
            }
        }
        Ok(-1)
    }

    fn unary(&mut self, _: bool) -> Result<()> {
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

    fn binary(&mut self, _: bool) -> Result<()> {
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

    fn literal(&mut self, _: bool) -> Result<()> {
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
        let can_assign = precedence <= Precedence::Assignment;
        prefix_fule.map_or_else(|| error::NotExpressionSnafu.fail(), |t| t(self, can_assign))?;

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
            infix_rule(self, can_assign)?;
        }
        if can_assign && matches!(self.current, Some(Token::Equal { .. })) {
            self.advance();
            return error::InvalidAssignTargetSnafu {
                token: unsafe { self.previous.clone().unwrap_unchecked() },
            }
            .fail();
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
        let Some(cur) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };

        match cur {
            Token::Var { .. } => {
                self.advance();
                self.var_declaration()?;
            },
            _ => {
                self.statement()?;
            },
        }

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
            Token::LeftBrace { .. } => {
                self.begin_scope();
                self.block()?;
                self.end_scope();
            },
            Token::If { .. } => self.if_statement()?,
            _ => self.expression_statement()?,
        }
        Ok(())
    }
    fn if_statement(&mut self) -> Result<()> {
        self.consume_left_paren()?;
        self.expression()?;
        self.consume_right_paren()?;

        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse);
        self.statement()?;
        self.patch_jump(then_jump)
    }

    fn emit_jump(&mut self, instruct: impl Into<u8>) -> usize {
        self.emit_byte(instruct);
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        self.cur_chunk.count() - 2
    }

    fn patch_jump(&mut self, offset: usize) -> Result<()> {
        let jump = self.cur_chunk.count() - offset - 2;
        if jump > u16::MAX.into() {
            return error::TooMuchJumpSnafu.fail();
        }
        self.cur_chunk.code[offset] = ((jump >> 8) & 0xFF) as u8;
        self.cur_chunk.code[offset + 1] = ((jump >> 8) & 0xFF) as u8;
        Ok(())
    }

    const fn begin_scope(&mut self) {
        self.cur_compiler.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.cur_compiler.scope_depth -= 1;
        while let Some(local) = self.cur_compiler.locals.last()
            && local.depth as usize > self.cur_compiler.scope_depth
        {
            self.cur_compiler.locals.pop();
            self.emit_byte(OpCode::OpPop);
        }
    }

    fn block(&mut self) -> Result<()> {
        while let Some(cur_tk) = &self.current
            && !matches!(cur_tk, Token::RightBrace { .. })
        {
            self.declaration()?;
        }
        self.consume_right_brace()
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

    fn var_declaration(&mut self) -> Result<()> {
        let global = self.parse_variable()?;
        let Some(cur) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };

        match cur {
            Token::Equal { .. } => {
                self.advance();
                self.expression()?;
            },
            _ => self.emit_byte(OpCode::OpNil),
        }
        self.consume_semicolon()?;

        self.define_var_global(global);
        Ok(())
    }

    fn parse_variable(&mut self) -> Result<u8> {
        self.consume_ident()?;
        self.declare_var()?;
        if self.cur_compiler.scope_depth > 0 {
            return Ok(0);
        }

        let ident = unsafe { self.previous.as_ref().unwrap_unchecked() };
        self.make_constant(Value::Obj(Obj::String(ident.lexeme().to_owned())))
    }

    fn define_var_global(&mut self, global: u8) {
        if self.cur_compiler.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_bytes(OpCode::OpDefaineGlobal, global);
    }

    fn declare_var(&mut self) -> Result<()> {
        if self.cur_compiler.scope_depth == 0 {
            return Ok(());
        }
        let Some(name) = self.previous.clone()
        else {
            return error::MissingPrevSnafu.fail();
        };
        for local in self.cur_compiler.locals.iter().rev() {
            if local.depth != -1 && (local.depth as usize) < self.cur_compiler.scope_depth {
                break;
            }

            if name.lexeme() == local.name.lexeme() {
                return error::DuplicateVarNameSnafu { name }.fail();
            }
        }
        self.add_local(name)?;

        Ok(())
    }

    fn add_local(&mut self, name: Token) -> Result<()> {
        if self.cur_compiler.locals.len() == u8::MAX.into() {
            return error::TooManyLocalVarSnafu.fail();
        }
        self.cur_compiler.locals.push(Local { name, depth: -1 });

        Ok(())
    }

    fn mark_initialized(&mut self) {
        unsafe { self.cur_compiler.locals.last_mut().unwrap_unchecked() }.depth =
            self.cur_compiler.scope_depth as i32;
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
            cur_compiler: Compiler::new(),
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
            cur_compiler: self.cur_compiler,
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

    fn consume_left_paren(&mut self) -> Result<()> {
        let Some(tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        match tk {
            Token::LeftParen { .. } => {
                self.advance();
                Ok(())
            },
            t => error::NotMatchSnafu {
                msg: "Expect '('",
                token: Some(t.clone()),
            }
            .fail(),
        }
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

    fn consume_ident(&mut self) -> Result<()> {
        let Some(tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        match tk {
            Token::Identifier { .. } => {
                self.advance();
                Ok(())
            },
            t => error::NotMatchSnafu {
                msg: "Expect variable name",
                token: Some(t.clone()),
            }
            .fail(),
        }
    }

    fn consume_right_brace(&mut self) -> Result<()> {
        let Some(tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        match tk {
            Token::RightBrace { .. } => {
                self.advance();
                Ok(())
            },
            t => error::NotMatchSnafu {
                msg: "Expect '}' after block",
                token: Some(t.clone()),
            }
            .fail(),
        }
    }
}
