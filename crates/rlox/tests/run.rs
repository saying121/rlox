#![expect(clippy::unwrap_used, reason = "test")]

use std::{path::PathBuf, str::FromStr};

use test_generator::test_resources;

mod ok {
    use super::*;

    #[test_resources("./crates/rlox/test-resource/ok/*")]
    fn run(resource: &str) {
        let Ok(p) = PathBuf::from_str(resource);
        let p = p.strip_prefix("crates/rlox/").unwrap();

        let lox = rlox::lox::Lox::new();
        let r = lox.run_file(p);
        assert!(r.is_ok());
    }
}

mod err {
    use super::*;
    #[test_resources("./crates/rlox/test-resource/err/*")]
    fn run(resource: &str) {
        let Ok(p) = PathBuf::from_str(resource);

        let p = p.strip_prefix("crates/rlox/").unwrap();

        let lox = rlox::lox::Lox::new();
        let r = lox.run_file(p);
        assert!(r.is_err());
    }
}
