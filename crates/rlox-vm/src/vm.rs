use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Vm<'v> {
    pub chunk: &'v Chunk,
    pub ip: &'v [u8],
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

impl<'v> Vm<'v> {
    pub fn new(chunk: &'v Chunk) -> Self {
        Self {
            chunk,
            ip: chunk.code(),
            stack: vec![],
        }
    }

    // pub fn interpret() -> InterpretResult {
    //     unimplemented!()
    // }

    pub fn run(&mut self) -> InterpretResult {
        macro_rules! binary_op {
            ($stack:expr, $op:tt) => {
                {
                    let b = $stack.pop().unwrap().0;
                    let a = $stack.pop().unwrap().0;
                    self.stack.push(Value(a $op b));
                }
            };
        }

        let mut ip_iter = self.ip.iter().enumerate();
        while let Some((offset, &ele)) = ip_iter.next() {
            #[cfg(debug_assertions)]
            {
                for ele in &self.stack {
                    print!("[{}]", ele.0);
                }
                println!();
                Chunk::disassemble_instruction(self.chunk, offset);
            };

            match ele.into() {
                OpCode::OpReturn => {
                    let v = self.stack.pop().unwrap();
                    println!("{}", v.0);
                    return InterpretResult::Ok;
                },
                OpCode::OpConstant => {
                    let next = *ip_iter.next().unwrap().1 as usize;
                    let constant = self.chunk.constants()[next];
                    self.stack.push(constant);
                },
                OpCode::OpNegate => {
                    let mut value = self.stack.pop().unwrap();
                    value.0 = -value.0;
                    self.stack.push(value);
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
