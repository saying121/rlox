#[test]
fn run_scripts() {
    let read_dir = std::fs::read_dir("./test-resource").unwrap();
    for ele in read_dir {
        let path = ele.unwrap();
        let path = path.path();
        let lox = rlox::lox::Lox::new();
        lox.run_file(path).unwrap();
    }
}
