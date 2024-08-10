use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::lox::Lox;

pub fn run_prompt() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                Lox::run(line);
                // run(line)?;
                // println!("Line: {}", line);
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
