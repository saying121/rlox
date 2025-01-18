use rlox_vm::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::Vm,
};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(Value(1.2));
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);

    let constant = chunk.add_constant(Value(3.4));
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpAdd, 123);

    let constant = chunk.add_constant(Value(5.6));
    chunk.write(OpCode::OpConstant, 123);
    chunk.write(constant as u8, 123);

    chunk.write(OpCode::OpDivide, 123);

    chunk.write(OpCode::OpNegate, 123);
    chunk.write(OpCode::OpReturn, 123);

    chunk.disassemble("test chunk");

    let mut vm = Vm::new(&chunk);
    vm.run();
}
