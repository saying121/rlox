use rlox::scan::scanner::Scanner;

use crate::{
    chunk::{Chunk, OpCode},
    compiler::Parser,
    error::{self, Result},
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct Vm {
    pub stack: Vec<Value>,
}

impl Vm {
    pub const fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn interpret(&mut self, source: &str) -> Result<()> {
        let chunk = Chunk::new();

        let mut scanner = Scanner::new(source);
        let p = Parser::new(scanner.scan_tokens());
        let chunk = p.compile(chunk)?;
        self.run(&chunk, chunk.code())
    }

    pub fn run(&mut self, chunk: &Chunk, ip: &[u8]) -> Result<()> {
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

        let mut ip_iter = ip.iter().enumerate();
        while let Some((offset, &ele)) = ip_iter.next() {
            #[cfg(debug_assertions)]
            {
                for ele in &self.stack {
                    print!("[{}]", ele);
                }
                println!();
                Chunk::disassemble_instruction(chunk, offset);
            };

            match ele.into() {
                OpCode::OpReturn => {
                    let Some(v) = self.stack.pop()
                    else {
                        return error::ReturnEmptyStackSnafu.fail();
                    };
                    println!("{}", v);
                    return Ok(());
                },
                OpCode::OpConstant => {
                    // Safety: OpConstant next must be index
                    let next = unsafe { ip_iter.next().unwrap_unchecked() };
                    let next = *next.1 as usize;
                    let constant = chunk.constants()[next].clone();
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
                            let line = chunk.get_line(offset);
                            return error::NegateNotNumSnafu { line }.fail();
                        },
                    }
                },
                OpCode::OpNil => self.stack.push(Value::Nil),
                OpCode::OpTrue => self.stack.push(Value::Bool(true)),
                OpCode::OpFalse => self.stack.push(Value::Bool(false)),
                OpCode::OpAdd => binary_op!(+, offset, Number),
                OpCode::OpSubtract => binary_op!(-, offset, Number),
                OpCode::OpMultiply => binary_op!(*, offset, Number),
                OpCode::OpDivide => binary_op!(/, offset, Number),
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
                OpCode::OpGreater => binary_op!(>, offset, Bool),
                OpCode::OpLess => binary_op!(<, offset, Bool),
            }
        }
        Ok(())
    }

    fn is_falsey(value: &Value) -> bool {
        match value {
            Value::Bool(b) => !b,
            Value::Nil => true,
            Value::Str(_) | Value::Number(_) => false,
        }
    }
}
