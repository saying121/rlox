use std::path::Path;

use anyhow::Result;

use crate::ast_printer::AstPrinter;
use crate::parser::Parser;
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
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut parser = Parser::new(tokens);
        let expression = parser.parse()?;

        let ast = AstPrinter.print(&expression);
        println!("{ast}");

        Ok(())
    }

    pub fn error(line: usize, message: &str) {
        Self::report(line, "", message);
    }

    fn report(line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] {where_}: {message}");
    }
}
