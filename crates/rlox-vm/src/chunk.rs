use std::mem;

use crate::{
    object::Obj,
    value::{Value, ValueArray},
};

#[derive(strum::Display)]
#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OpCode {
    OpConstant,
    OpNil,
    OpTrue,
    OpFalse,
    OpPop,
    OpGetLocal,
    OpGetGlobal,
    OpSetLocal,
    OpSetGlobal,
    OpDefaineGlobal,
    OpEqual,
    OpGreater,
    OpLess,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNot,
    OpNegate,
    OpPrint,
    // OpConstantLong,
    OpReturn,
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

    pub fn write<V: Into<u8>>(&mut self, value: V, line: usize) {
        self.code.push(value.into());
        let last = self.lines.last_mut();
        match last {
            Some((count, line_)) if *line_ == line => *count += 1,
            _ => self.lines.push((1, line)),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write(value);
        self.constants.len() - 1
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

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub const fn constants(&self) -> &ValueArray {
        &self.constants
    }

    pub fn get_ident_string(&self, idx: usize) -> String {
        let objs = self.constants[idx].clone();
        let Value::Obj(Obj::String(name)) = objs
        else {
            unreachable!("Expect string")
        };
        name
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

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:0>4} ", offset);
        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {
            print!("   | ");
        }
        else {
            print!("{:0>4} ", self.get_line(offset));
        }

        match self.code[offset].into() {
            v @ (OpCode::OpDefaineGlobal
            | OpCode::OpConstant
            | OpCode::OpSetGlobal
            | OpCode::OpGetGlobal) => {
                self.constant_instruction(v, offset)
            },
            v => Self::simple_instruction(v, offset),
        }
    }

    fn constant_instruction(&self, name: OpCode, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:<16} {:>4} '", name, constant);
        println!("{}", self.constants[constant as usize]);
        offset + 2
    }

    fn simple_instruction(v: OpCode, offset: usize) -> usize {
        println!("{v}");
        offset + 1
    }
}
