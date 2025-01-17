use crate::chunk::{Chunk, OpCode};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub struct Vm<'v> {
    pub chunk: &'v Chunk,
    pub ip: &'v [u8],
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
        }
    }

    // pub fn interpret() -> InterpretResult {
    //     unimplemented!()
    // }

    pub fn run(&self) -> InterpretResult {
        let mut ip_iter = self.ip.iter().enumerate();
        while let Some((offset, &ele)) = ip_iter.next() {
            #[cfg(debug_assertions)]
            Chunk::disassemble_instruction(self.chunk, offset);

            match ele.into() {
                OpCode::OpReturn => return InterpretResult::Ok,
                OpCode::OpConstant => {
                    let next = *ip_iter.next().unwrap().1 as usize;
                    let constant = self.chunk.constants()[next];
                    println!("{}", constant.0);
                    // break;
                },
            }
        }
        unimplemented!()
    }
}
