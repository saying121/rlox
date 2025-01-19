use crate::{
    chunk::{Chunk, OpCode},
    complier,
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
        complier::compile(source, &chunk)?;
        // self.run(chunk, chunk.code())
        self.run(&chunk, chunk.code())
    }

    pub fn run(&mut self, chunk: &Chunk, ip: &[u8]) -> Result<()> {
        macro_rules! binary_op {
            ($stack:expr, $op:tt) => {
                {
                    let b = $stack.pop().unwrap().0;
                    let a = $stack.pop().unwrap().0;
                    self.stack.push(Value(a $op b));
                }
            };
        }

        let mut ip_iter = ip.iter().enumerate();
        while let Some((offset, &ele)) = ip_iter.next() {
            #[cfg(debug_assertions)]
            {
                for ele in &self.stack {
                    print!("[{}]", ele.0);
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
                    println!("{}", v.0);
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
                    value.0 = -value.0;
                },
                OpCode::OpAdd => binary_op!(self.stack, +),
                OpCode::OpSubtract => binary_op!(self.stack, -),
                OpCode::OpMultiply => binary_op!(self.stack, *),
                OpCode::OpDivide => binary_op!(self.stack, /),
            }
        }
        Ok(())
    }
}
