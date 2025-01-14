use crate::value::{Value, ValueArray};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum OpCode {
    OpReturn,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct Chunk {
    // `count`, `capacity`, rust direct use `Vec`
    code: Vec<OpCode>,
    constants: ValueArray,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            code: vec![],
            constants: ValueArray::new(),
        }
    }

    pub fn write(&mut self, value: OpCode) {
        self.code.push(value);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.0.len() - 1
    }

    pub fn count(&self) -> usize {
        self.code.len()
    }
}

// debug
impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        for (offset, &instruction) in self.code.iter().enumerate() {
            Self::disassemble_instruction(offset, instruction);
        }
    }

    fn disassemble_instruction(offset: usize, instruction: OpCode) {
        println!("{:0>4}", offset);

        match instruction {
            OpCode::OpReturn => Self::simple_instruction("OP_RETURN"),
        }
    }

    fn simple_instruction(name: &str) {
        println!("{}", name);
    }
}
