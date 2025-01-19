use rlox::scan::scanner::Scanner;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    unimplemented!()
}
