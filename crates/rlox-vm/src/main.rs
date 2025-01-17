use rlox_vm::{
    chunk::{Chunk, OpCode},
    value::Value,
    vm::Vm,
};

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(Value(1.2));
    chunk.write(OpCode::OpConstant.into(), 123);
    chunk.write(constant as u8, 123);
    chunk.write(OpCode::OpReturn.into(), 123);
    chunk.disassemble("test chunk");

    let vm = Vm::new(&chunk);
    vm.run();
}
