use rlox::scan::scanner::Scanner;

use crate::chunk::Chunk;
use crate::error::Result;

pub fn compile(source: &str, chunk: &Chunk) -> Result<()> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for token in tokens {

    }
    Ok(())
}
