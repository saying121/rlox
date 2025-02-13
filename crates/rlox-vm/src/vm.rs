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
            ($stack:expr, $op:tt, $offset:expr) => {
                {
                    let last: Option<&[Value; 2]> = self.stack.last_chunk();
                    match last {
                        Some([Value::Double(a), Value::Double(b)]) => {
                            self.stack.push(Value::Double(a $op b));
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
                    let constant = chunk.constants()[next];
                    self.stack.push(constant);
                },
                OpCode::OpNegate => {
                    let Some(value) = self.stack.last_mut()
                    else {
                        return error::NegateEmptyStackSnafu.fail();
                    };
                    match value {
                        Value::Double(d) => {
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
                OpCode::OpAdd => binary_op!(self.stack, +, offset),
                OpCode::OpSubtract => binary_op!(self.stack, -, offset),
                OpCode::OpMultiply => binary_op!(self.stack, *, offset),
                OpCode::OpDivide => binary_op!(self.stack, /, offset),
            }
        }
        Ok(())
    }
}
