use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Vm {
    // pub chunk: &'v Chunk,
    // pub ip: &'v [u8],
    pub stack: Vec<Value>,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl Vm {
    pub const fn new() -> Self {
        Self {
            // chunk,
            // ip: chunk.code(),
            stack: vec![],
        }
    }

    pub fn interpret(&mut self, chunk: &[u8]) -> InterpretResult {
        // self.run(chunk, chunk.code())
        self.run(todo!(), todo!())
    }

    pub fn run(&mut self, chunk: &Chunk, ip: &[u8]) -> InterpretResult {
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
                    let v = self.stack.pop().unwrap();
                    println!("{}", v.0);
                    return InterpretResult::Ok;
                },
                OpCode::OpConstant => {
                    // Safety: OpConstant next must be index
                    let next = unsafe { ip_iter.next().unwrap_unchecked() };
                    let next = *next.1 as usize;
                    let constant = chunk.constants()[next];
                    self.stack.push(constant);
                },
                OpCode::OpNegate => {
                    let value = self.stack.last_mut().unwrap();
                    value.0 = -value.0;
                },
                OpCode::OpAdd => binary_op!(self.stack, +),
                OpCode::OpSubtract => binary_op!(self.stack, -),
                OpCode::OpMultiply => binary_op!(self.stack, *),
                OpCode::OpDivide => binary_op!(self.stack, /),
            }
        }
        InterpretResult::Ok
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
