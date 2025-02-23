use std::collections::HashMap;

use rlox::scan::scanner::Scanner;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Parser,
    error::{self, Result},
    object::Obj,
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
// #[derive(PartialEq, PartialOrd)]
pub struct Vm {
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    pub ip: usize,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            globals: HashMap::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let chunk = Chunk::new();

        let mut scanner = Scanner::new(source);
        let p = Parser::new(scanner.scan_tokens());
        let chunk = p.compile(chunk)?;
        self.run(&chunk, chunk.code())
    }

    pub fn run(&mut self, chunk: &Chunk, code: &[u8]) -> Result<()> {
        macro_rules! binary_op {
            ($op:tt, $offset:expr, $type:ident) => {
                {
                    let last: Option<&[Value; 2]> = self.stack.last_chunk();
                    match last {
                        Some([Value::Number(a), Value::Number(b)]) => {
                            self.stack.push(Value::$type(a $op b));
                        },
                        _ => return error::BinaryNotNumSnafu {
                            line: chunk.get_line($offset),
                        }
                        .fail(),
                    }
                }
            };
        }

        let code_len = code.len();
        while self.ip < code_len {
            #[cfg(debug_assertions)]
            {
                for ele in &self.stack {
                    print!("[{}]", ele);
                }
                println!();
                Chunk::disassemble_instruction(chunk, self.ip);
            };

            match self.read_byte(code).into() {
                OpCode::OpReturn => return Ok(()),
                OpCode::OpConstant => {
                    let constant = self.read_const(chunk);
                    self.stack.push(constant);
                },
                OpCode::OpNot => {
                    let Some(value) = self.stack.pop()
                    else {
                        return error::NotEmptyStackSnafu.fail();
                    };
                    self.stack.push(Value::Bool(Self::is_falsey(&value)));
                },
                OpCode::OpNegate => {
                    let Some(value) = self.stack.last_mut()
                    else {
                        return error::NegateEmptyStackSnafu.fail();
                    };
                    match value {
                        Value::Number(d) => {
                            *d = -*d;
                        },
                        _ => {
                            let line = chunk.get_line(self.ip);
                            return error::NegateNotNumSnafu { line }.fail();
                        },
                    }
                },
                OpCode::OpNil => self.stack.push(Value::Nil),
                OpCode::OpTrue => self.stack.push(Value::Bool(true)),
                OpCode::OpFalse => self.stack.push(Value::Bool(false)),
                OpCode::OpPop => {
                    if self.stack.pop().is_none() {
                        return error::EmptyStackSnafu.fail();
                    }
                },
                OpCode::OpGetGlobal => {
                    let name = self.read_string(chunk);
                    let Some(val) = self.globals.get(&name)
                    else {
                        return error::UndefindVarSnafu { name }.fail();
                    };

                    self.stack.push(val.to_owned());
                },
                OpCode::OpSetGlobal => {
                    let name = self.read_string(chunk);
                    let Some(v) = self.stack.last()
                    else {
                        return error::EmptyStackSnafu.fail();
                    };
                    if self.globals.insert(name.clone(), v.clone()).is_none() {
                        self.globals.remove(&name);
                        return error::UndefindVarSnafu { name }.fail();
                    }
                },
                OpCode::OpDefaineGlobal => {
                    let name = self.read_string(chunk);
                    let Some(v) = self.stack.pop()
                    else {
                        return error::EmptyStackSnafu.fail();
                    };

                    self.globals.insert(name, v);
                },
                OpCode::OpAdd => {
                    let last: Option<&[Value; 2]> = self.stack.last_chunk();
                    match last {
                        Some([Value::Number(a), Value::Number(b)]) => {
                            self.stack.push(Value::Number(a + b));
                        },
                        Some([Value::Obj(Obj::String(a)), Value::Obj(Obj::String(b))]) => {
                            self.stack.push(Value::Obj(Obj::String(format!("{a}{b}"))));
                        },
                        _ => {
                            return error::BinaryNotNumSnafu {
                                line: chunk.get_line(self.ip),
                            }
                            .fail();
                        },
                    }
                },
                OpCode::OpSubtract => binary_op!(-, self.ip, Number),
                OpCode::OpMultiply => binary_op!(*, self.ip, Number),
                OpCode::OpDivide => binary_op!(/, self.ip, Number),
                OpCode::OpEqual => {
                    let Some(b) = self.stack.pop()
                    else {
                        return error::EmptyStackSnafu.fail();
                    };
                    let Some(a) = self.stack.pop()
                    else {
                        return error::EmptyStackSnafu.fail();
                    };
                    self.stack.push(Value::Bool(a == b));
                },
                OpCode::OpGreater => binary_op!(>, self.ip, Bool),
                OpCode::OpLess => binary_op!(<, self.ip, Bool),
                OpCode::OpPrint => {
                    let Some(var) = self.stack.pop()
                    else {
                        return error::EmptyStackSnafu.fail();
                    };
                    println!("{}", var);
                },
                OpCode::OpGetLocal => {
                    let slot = self.read_byte(code);
                    self.stack.push(self.stack[slot as usize].clone());
                },
                OpCode::OpSetLocal => {
                    let slot = self.read_byte(code);
                    let value = unsafe { self.stack.last().unwrap_unchecked() }.clone();
                    self.stack[slot as usize] = value;
                },
                OpCode::OpJumpIfFalse => {
                    let offset = self.read_short(code);
                    if Self::is_falsey(unsafe { self.stack.last().unwrap_unchecked() }) {
                        self.ip += offset as usize;
                    }
                },
                OpCode::OpJump => {
                    let offset = self.read_short(code);
                    self.ip += offset as usize;
                },
                OpCode::OpLoop => {
                    let offset = self.read_short(code);
                    self.ip -= offset as usize;
                },
            }
        }

        Ok(())
    }

    fn is_falsey(value: &Value) -> bool {
        match value {
            Value::Bool(b) => !b,
            Value::Nil => true,
            Value::Obj(_) | Value::Number(_) => false,
        }
    }

    fn read_byte(&mut self, code: &[u8]) -> u8 {
        let ip = self.ip;
        self.ip += 1;
        code[ip]
    }

    fn read_short(&mut self, code: &[u8]) -> u16 {
        let offset = self.ip;
        self.ip += 2;
        u16::from_be_bytes([code[offset + 1], code[offset + 2]])
    }

    fn read_string(&mut self, chunk: &Chunk) -> String {
        let ip = self.ip;
        let next = chunk.code[ip];
        self.ip += 1;
        chunk.get_ident_string(next as usize)
    }

    fn read_const(&mut self, chunk: &Chunk) -> Value {
        let next = self.read_byte(&chunk.code);
        let next = next as usize;
        chunk.constants()[next].clone()
    }
}
