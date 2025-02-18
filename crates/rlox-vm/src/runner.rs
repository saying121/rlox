use std::{path::Path, process::exit};

use rustyline::{DefaultEditor, error::ReadlineError};
use snafu::ResultExt;

use crate::{
    error::{LoxError, ReadFileSnafu, ReplSnafu, Result},
    vm::Vm,
};

pub fn run_prompt(vm: &mut Vm) -> Result<()> {
    let mut rl = DefaultEditor::new().with_context(|_| ReplSnafu)?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .with_context(|_| ReplSnafu)?;
                vm.interpret(&line)?;
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }
    Ok(())
}

pub fn run_file<P: AsRef<Path>>(vm: &mut Vm, path: P) -> Result<()> {
    let source = std::fs::read_to_string(&path).with_context(|_| ReadFileSnafu {
        path: path.as_ref().display().to_string(),
    })?;
    let result = vm.interpret(&source);
    match result {
        Err(LoxError::CompileError { .. }) => exit(65),
        Err(LoxError::RuntimeError { .. }) => exit(70),
        v => v,
    }
}

#[test]
fn feature() {
    let mut vm = Vm::new();

    vm.interpret("print !(5 - 4 > 3 * 2 == !nil);").unwrap();
    vm.interpret("print 1;").unwrap();
}
