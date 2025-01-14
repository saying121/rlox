use rlox_vm::{
    chunk::{Chunk, OpCode},
    value::Value,
};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(Value(1.2));
    chunk.write(OpCode::OpConstant.into());
    chunk.write(constant as u8);
    chunk.write(OpCode::OpReturn.into());
    chunk.disassemble("test chunk");
}
