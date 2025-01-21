use snafu::{Location, Snafu};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum LoxError {
    #[snafu(display("Repl: {source}"))]
    Repl {
        #[snafu(source)]
        source: rustyline::error::ReadlineError,
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Failed to read: {}", path))]
    ReadFile {
        #[snafu(source)]
        source: std::io::Error,
        path: String,
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Compile error"))]
    CompileError {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Compile error"))]
    RuntimeError {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Return but stack is empty"))]
    ReturnEmptyStack {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Negate op but stack is empty"))]
    NegateEmptyStack {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Too many consts"))]
    TooManyConsts {
        #[snafu(implicit)]
        localtion: Location,
    },
}

pub type Result<T> = std::result::Result<T, LoxError>;
