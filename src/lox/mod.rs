use std::path::Path;

use anyhow::Result;

use crate::scan::scanner::Scanner;

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Lox {
    had_err: bool,
}

impl Lox {
    pub fn run_file<T: AsRef<Path>>(path: T) -> Result<()> {
        let content = std::fs::read_to_string(path)?;
        Self::run(content)?;

        Ok(())
    }

    pub fn run(source: String) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        unimplemented!();

        Ok(())
    }

    pub fn error(line: usize, message: String) {
        Self::report(line, "".to_owned(), message);
    }

    fn report(line: usize, where_: String, message: String) {
        eprintln!("[line {line}] {where_}: {message}");
    }
}
