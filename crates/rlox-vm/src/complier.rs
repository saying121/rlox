use rlox::scan::scanner::Scanner;

use crate::{chunk::Chunk, error::Result};

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Compiler {
    // field: (),
}

impl Compiler {
    pub fn compile(source: &str, chunk: &Chunk) -> Result<()> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let peek = itertools::peek_nth(tokens);
        for token in peek {}
        Ok(())
    }
}
