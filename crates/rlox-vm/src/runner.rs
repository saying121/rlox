use std::{path::Path, process::exit};

use rustyline::{DefaultEditor, error::ReadlineError};
use snafu::ResultExt;

use crate::{
    error::{ReadFileSnafu, ReplSnafu, Result},
    vm::{InterpretResult, Vm},
};

pub fn run_prompt(vm: &mut Vm) -> crate::error::Result<()> {
    let mut rl = DefaultEditor::new().with_context(|_| ReplSnafu)?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())
                    .with_context(|_| ReplSnafu)?;
                vm.interpret(line.as_bytes());
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
    let source = std::fs::read(&path).with_context(|_| ReadFileSnafu {
        path: path.as_ref().display().to_string(),
    })?;
    let result = vm.interpret(&source);
    match result {
        InterpretResult::Ok => (),
        InterpretResult::CompileError => exit(65),
        InterpretResult::RuntimeError => exit(70),
    }
    Ok(())
}
