use test_generator::test_resources;

#[test_resources("./test-resource/ok/*")]
fn run_ok(resource: &str) {
    let lox = rlox::lox::Lox::new();
    let r = lox.run_file(resource);
    assert!(r.is_ok());
}

#[test_resources("./test-resource/err/*")]
fn run_err(resource: &str) {
    let lox = rlox::lox::Lox::new();
    let r = lox.run_file(resource);
    assert!(r.is_err());
}
