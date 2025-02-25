pub mod rule;

use std::{cell::RefCell, convert::Into, hint::unreachable_unchecked};

use itertools::PeekNth;
use rlox::token::{Token, TokenInner};

use self::rule::{ParseRule, get_rule};
use crate::{
    chunk::{Chunk, OpCode},
    error::{self, Result},
    object::{Obj, ObjFunction},
    value::Value,
};

thread_local! {
    pub static CUR_CHUNK: RefCell<Chunk> = const { RefCell::new(Chunk::new()) };
}

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
    cur_compiler: Compiler,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CurFunType {
    Fun,
    #[default]
    Script,
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
#[derive(PartialEq)]
pub struct Compiler {
    function: ObjFunction,
    cur_fn_typ: CurFunType,
    locals: Vec<Local>,
    scope_depth: usize,
}

impl Compiler {
    pub fn new(cur_fn_typ: CurFunType) -> Self {
        Self {
            locals: vec![Local {
                name: Token::Invalid {
                    inner: TokenInner::default(),
                },
                depth: 0,
            }],
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
            function: ObjFunction::new(),
            cur_fn_typ,
        }
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn grouping(&mut self, _: bool) -> Result<()> {
        // self.consume_left_paren();
        self.expression()?;
        self.consume_right_paren()
    }

    fn end_compiler(self) -> ObjFunction {
        self.emit_return();
        // #[cfg(debug_assertions)]
        if !self.had_error {
            CUR_CHUNK.with_borrow_mut(|v| {
                v.disassemble(
                    if self.cur_compiler.function.name.is_empty() {
                        "<script>"
                    }
                    else {
                        &self.cur_compiler.function.name
                    },
                );
            });
        }
        self.cur_compiler.function
    }

    fn emit_return(&self) {
        self.emit_byte(OpCode::OpReturn);
    }

    fn emit_bytes<B1, B2>(&self, byte1: B1, byte2: B2)
    where
        B1: Into<u8>,
        B2: Into<u8>,
    {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_byte<B: Into<u8>>(&self, byte: B) {
        let (row, _col) = self
            .previous
            .as_ref()
            .expect("Missing previous token")
            .inner()
            .get_xy();
        CUR_CHUNK.with_borrow_mut(|v| {
            v.write(byte, row);
        });
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
        let Some(name) = self.previous.clone()
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
            Self::make_constant(Value::Obj(Obj::String(name.lexeme().to_owned())))?
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
        let rule: ParseRule<I> = get_rule(&op_type);
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

    fn emit_constant(&self, value: Value) -> Result<()> {
        let byte2 = Self::make_constant(value)?;
        self.emit_bytes(OpCode::OpConstant, byte2);
        Ok(())
    }

    fn make_constant(value: Value) -> Result<u8> {
        let constant = CUR_CHUNK.with_borrow_mut(|v| v.add_constant(value));
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
                self.advance();
                self.begin_scope();
                self.block()?;
                self.end_scope();
            },
            Token::If { .. } => {
                self.advance();
                self.if_statement()?;
            },
            Token::While { .. } => {
                self.advance();
                self.while_statement()?;
            },
            Token::For { .. } => {
                self.advance();
                self.for_statement()?;
            },
            _ => self.expression_statement()?,
        }
        Ok(())
    }

    fn for_statement(&mut self) -> Result<()> {
        self.begin_scope();

        self.consume_left_paren()?;
        let Some(cur_tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };

        match cur_tk {
            Token::Semicolon { .. } => {
                self.advance();
            },
            Token::Var { .. } => {
                self.advance();
                self.var_declaration()?;
            },
            _ => {
                self.expression_statement()?;
            },
        }

        let mut loop_start = CUR_CHUNK.with_borrow(Chunk::count);
        let mut exit_jump = None;
        let Some(cur_tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        if !matches!(cur_tk, Token::Semicolon { .. }) {
            self.expression()?;
            self.consume_semicolon()?;

            exit_jump = Some(self.emit_jump(OpCode::OpJumpIfFalse));
            self.emit_byte(OpCode::OpPop);
        }

        let Some(cur_tk) = &self.current
        else {
            return error::MissingCurSnafu.fail();
        };
        if !matches!(cur_tk, Token::RightParen { .. }) {
            let body_jump = self.emit_jump(OpCode::OpJump);
            let increment_start = CUR_CHUNK.with_borrow(Chunk::count);
            self.expression()?;
            self.emit_byte(OpCode::OpPop);
            self.consume_right_paren()?;

            self.emit_loop(loop_start)?;
            loop_start = increment_start;
            Self::patch_jump(body_jump)?;
        }

        self.statement()?;

        self.emit_loop(loop_start)?;

        if let Some(exit_jump) = exit_jump {
            Self::patch_jump(exit_jump)?;
            self.emit_byte(OpCode::OpPop);
        }
        self.end_scope();

        Ok(())
    }

    fn while_statement(&mut self) -> Result<()> {
        let loop_start = CUR_CHUNK.with_borrow(Chunk::count);

        self.consume_left_paren()?;
        self.expression()?;
        self.consume_right_paren()?;

        let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse);
        self.emit_byte(OpCode::OpPop);
        self.statement()?;

        self.emit_loop(loop_start)?;

        Self::patch_jump(exit_jump)?;
        self.emit_byte(OpCode::OpPop);

        Ok(())
    }

    fn emit_loop(&self, loop_start: usize) -> Result<()> {
        self.emit_byte(OpCode::OpLoop);

        let offset = CUR_CHUNK.with_borrow(Chunk::count) - loop_start + 2;
        if offset > u16::MAX.into() {
            return error::LoopLargeSnafu.fail();
        }
        self.emit_byte((offset >> 8) as u8);
        self.emit_byte(offset as u8);

        Ok(())
    }

    fn if_statement(&mut self) -> Result<()> {
        self.consume_left_paren()?;
        self.expression()?;
        self.consume_right_paren()?;

        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse);
        self.emit_byte(OpCode::OpPop);
        self.statement()?;
        let else_jump = self.emit_jump(OpCode::OpJump);
        Self::patch_jump(then_jump)?;
        self.emit_byte(OpCode::OpPop);

        if matches!(self.current, Some(Token::Else { .. })) {
            self.advance();
            self.statement()?;
        }
        Self::patch_jump(else_jump)?;

        Ok(())
    }

    fn emit_jump(&self, instruct: impl Into<u8>) -> usize {
        self.emit_byte(instruct);
        self.emit_byte(0xFF);
        self.emit_byte(0xFF);
        CUR_CHUNK.with_borrow(|v| v.count() - 2)
    }

    fn patch_jump(offset: usize) -> Result<()> {
        let jump = CUR_CHUNK.with_borrow(|v| v.count() - offset - 2);
        if jump > u16::MAX.into() {
            return error::TooMuchJumpSnafu.fail();
        }
        CUR_CHUNK.with_borrow_mut(|v| {
            v.code[offset] = ((jump >> 8) & 0xFF) as u8;
        });
        CUR_CHUNK.with_borrow_mut(|v| {
            v.code[offset + 1] = ((jump >> 8) & 0xFF) as u8;
        });
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
        Self::make_constant(Value::Obj(Obj::String(ident.lexeme().to_owned())))
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

    fn and(&mut self, _: bool) -> Result<()> {
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse);

        self.emit_byte(OpCode::OpPop);
        self.parse_precedence(Precedence::And)?;
        Self::patch_jump(end_jump)
    }

    fn or(&mut self, _: bool) -> Result<()> {
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse);
        let end_jump = self.emit_jump(OpCode::OpJump);

        Self::patch_jump(else_jump)?;
        self.emit_byte(OpCode::OpPop);

        self.parse_precedence(Precedence::Or)?;
        Self::patch_jump(end_jump)
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
            cur_compiler: Compiler::new(CurFunType::Script),
        }
    }

    pub fn compile(self) -> Result<ObjFunction> {
        let mut var = self;
        var.advance();
        while var.current.is_some() {
            var.declaration()?;
        }
        if var.had_error {
            return error::CompileSnafu.fail();
        }
        Ok(var.end_compiler())
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
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
