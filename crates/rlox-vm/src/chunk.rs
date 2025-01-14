use std::{fmt::Display, mem};

use crate::value::{Value, ValueArray};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum OpCode {
    OpConstant,
    // OpConstantLong,
    OpReturn,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpConstant => "OP_CONSTANT",
            // Self::OpConstantLong => "OP_CONSTANT_LONG",
            Self::OpReturn => "OP_RETURN",
        }
        .fmt(f)
    }
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        val as Self
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        unsafe { mem::transmute::<u8, Self>(value) }
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, PartialOrd)]
pub struct Chunk {
    // `count`, `capacity`, rust direct use `Vec`
    code: Vec<u8>,
    constants: ValueArray,
    // (count, line)
    lines: Vec<(usize, usize)>,
}

impl Chunk {
    pub const fn new() -> Self {
        Self {
            code: vec![],
            constants: ValueArray::new(),
            lines: vec![],
        }
    }

    pub fn write(&mut self, value: u8, line: usize) {
        self.code.push(value);
        let last = self.lines.last_mut();
        match last {
            Some((count, line_)) if *line_ == line => *count += 1,
            _ => self.lines.push((1, line)),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.0.len() - 1
    }

    // TODO
    pub fn write_constant(&self, _value: Value, _line: usize) {
        unimplemented!()
    }

    pub fn count(&self) -> usize {
        self.code.len()
    }

    pub fn get_line(&self, offset: usize) -> usize {
        let mut cur = 0;
        for &(count, cur_line) in &self.lines {
            cur += count;
            if offset < cur {
                return cur_line;
            }
        }
        0
    }
}

// debug
impl Chunk {
    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        }
        else {
            print!("{:0>4} ", self.get_line(offset));
        }

        match self.code[offset].into() {
            v @ OpCode::OpConstant => self.constant_instruction(v, offset),
            v @ OpCode::OpReturn => {
                println!("{v}");
                offset + 1
            },
        }
    }

    fn constant_instruction(&self, name: OpCode, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:<16} {:>4} '", name, constant);
        println!("{}", self.constants.0[constant as usize]);
        offset + 2
    }
}
