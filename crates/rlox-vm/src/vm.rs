use std::collections::HashMap;

use rlox::scan::scanner::Scanner;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Parser,
    error::{self, Result},
    object::{Obj, ObjFunction},
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
// #[derive(PartialEq, PartialOrd)]
pub struct Vm {
    pub frames: Vec<CallFrame>,
    pub frame_count: usize,
    pub stack: Vec<Value>,
    pub globals: HashMap<String, Value>,
    pub ip: usize,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct CallFrame {
    function: ObjFunction,
    ip: usize,
    ip_code: Vec<u8>,
    slots: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            // frames: vec![CallFrame::default(); 256],
            frames: vec![],
            frame_count: 0,
            stack: vec![],
            globals: HashMap::new(),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let p = Parser::new(scanner.scan_tokens());
        let function = p.compile()?;
        self.stack.push(Value::Obj(Obj::Fun(function.clone())));
        self.frames.push(CallFrame {
            function: function.clone(),
            ip: 0,
            slots: self.stack.clone(),
            ip_code: function.chunk.code,
        });
        self.run()
    }

    #[expect(clippy::unwrap_in_result, reason = "lazy")]
    pub fn run(&mut self) -> Result<()> {
        #[expect(clippy::unwrap_used, reason = "lazy")]
        let mut frame = self.frames.last().unwrap().clone();

        macro_rules! binary_op {
            ($op:tt, $offset:expr, $type:ident) => {
                {
                    let last: Option<&[Value; 2]> = self.stack.last_chunk();
                    match last {
                        Some([Value::Number(a), Value::Number(b)]) => {
                            self.stack.push(Value::$type(a $op b));
                        },
                        _ => return error::BinaryNotNumSnafu {
                                line: frame.function.chunk.get_line(frame.ip),
                        }
                        .fail(),
                    }
                }
            };
        }

        let code_len = frame.ip_code.len();
        while frame.ip < code_len {
            // #[cfg(debug_assertions)]
            {
                for ele in &self.stack {
                    print!("[{}]", ele);
                }
                println!();
                Chunk::disassemble_instruction(
                    &frame.function.chunk,
                    frame.ip - frame.function.chunk.code.len(),
                );
            };

            match frame.read_byte().into() {
                OpCode::OpReturn => return Ok(()),
                OpCode::OpConstant => {
                    let constant = frame.read_const();
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
                            let line = frame.function.chunk.get_line(frame.ip);
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
                    let name = frame.read_string();
                    let Some(val) = self.globals.get(&name)
                    else {
                        return error::UndefindVarSnafu { name }.fail();
                    };

                    self.stack.push(val.to_owned());
                },
                OpCode::OpSetGlobal => {
                    let name = frame.read_string();
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
                    let name = frame.read_string();
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
                                line: frame.function.chunk.get_line(frame.ip),
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
                    let slot = frame.read_byte();
                    self.stack.push(frame.slots[slot as usize].clone());
                },
                OpCode::OpSetLocal => {
                    let slot = frame.read_byte();
                    let value = unsafe { self.stack.last().unwrap_unchecked() }.clone();
                    frame.slots[slot as usize] = value;
                },
                OpCode::OpJumpIfFalse => {
                    let offset = frame.read_short();
                    if Self::is_falsey(unsafe { self.stack.last().unwrap_unchecked() }) {
                        frame.ip += offset as usize;
                    }
                },
                OpCode::OpJump => {
                    let offset = frame.read_short();
                    frame.ip += offset as usize;
                },
                OpCode::OpLoop => {
                    let offset = frame.read_short();
                    frame.ip -= offset as usize;
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
}

impl CallFrame {
    fn read_byte(&mut self) -> u8 {
        let ip = self.ip;
        self.ip += 1;
        self.ip_code[ip]
    }

    fn read_short(&mut self) -> u16 {
        let offset = self.ip;
        self.ip += 2;
        u16::from_be_bytes([self.ip_code[offset + 1], self.ip_code[offset + 2]])
    }

    fn read_string(&mut self) -> String {
        let ip = self.ip;
        let next = self.ip_code[ip];
        self.ip += 1;
        self.function.chunk.get_ident_string(next as usize)
    }

    fn read_const(&mut self) -> Value {
        let next = self.read_byte();
        let next = next as usize;
        self.function.chunk.constants()[next].clone()
    }
}
