use test_generator::test_resources;

mod ok {
    use super::*;

    #[test_resources("./test-resource/ok/*")]
    fn run(resource: &str) {
        let lox = rlox::lox::Lox::new();
        let r = lox.run_file(resource);
        assert!(r.is_ok());
    }
}

mod err {
    use super::*;
    #[test_resources("./test-resource/err/*")]
    fn run(resource: &str) {
        let lox = rlox::lox::Lox::new();
        let r = lox.run_file(resource);
        assert!(r.is_err());
    }
}
