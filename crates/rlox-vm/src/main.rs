use std::process::exit;

use clap::Parser;
use rlox_vm::{cli, error::Result, vm::Vm};

fn main() -> Result<()> {
    let mut vm = Vm::new();

    let cli = cli::Cli::parse();
    if cli.repl() {
        rlox_vm::runner::run_prompt(&mut vm)?;
    }
    else if let Some(f) = cli.file_path() {
        rlox_vm::runner::run_file(&mut vm, f)?;
    }
    else {
        exit(64)
    }
    Ok(())

    // let mut chunk = Chunk::new();
    // let constant = chunk.add_constant(Value(1.2));
    // chunk.write(OpCode::OpConstant, 123);
    // chunk.write(constant as u8, 123);
    //
    // let constant = chunk.add_constant(Value(3.4));
    // chunk.write(OpCode::OpConstant, 123);
    // chunk.write(constant as u8, 123);
    //
    // chunk.write(OpCode::OpAdd, 123);
    //
    // let constant = chunk.add_constant(Value(5.6));
    // chunk.write(OpCode::OpConstant, 123);
    // chunk.write(constant as u8, 123);
    //
    // chunk.write(OpCode::OpDivide, 123);
    //
    // chunk.write(OpCode::OpNegate, 123);
    // chunk.write(OpCode::OpReturn, 123);
    //
    // chunk.disassemble("test chunk");
    //
    // vm.interpret(&chunk);
}
