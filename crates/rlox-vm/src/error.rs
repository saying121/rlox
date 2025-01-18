use snafu::{Location, Snafu};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum LoxError {
    #[snafu(display(""))]
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
}

pub type Result<T> = std::result::Result<T, LoxError>;
