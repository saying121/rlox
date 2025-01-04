use std::path::Path;

use anyhow::{bail, Result};

use crate::{interpreter::Interpreter, parser::Parser, resolver::Resolver, scan::scanner::Scanner};

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq)]
pub struct Lox {
    had_err: bool,
    had_runtime_error: bool,
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_err: false,
            had_runtime_error: false,
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file<T: AsRef<Path>>(mut self, path: T) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        self.run(&content)
    }

    pub fn run(&mut self, source: &str) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let (mut expression, had_err) = parser.parse();
        if had_err {
            bail!("parse err")
        }
        let mut r = Resolver::new(&mut self.interpreter);
        let had_err = r.resolve(&expression);
        if had_err {
            bail!("resolver err")
        }

        // let ast = AstPrinter.print(&expression);
        // println!("{ast}");

        match self.interpreter.interpret(&mut expression) {
            Ok(_) => Ok(()),
            Err(e) => {
                bail!(e.to_string())
            },
        }
    }

    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    fn report(line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] {where_}: {message}");
    }
}
