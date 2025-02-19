use rlox::token::Token;
use snafu::{Location, Snafu};

#[derive(Debug)]
#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum LoxError {
    #[snafu(display("Empty stack"))]
    EmptyStack {
        #[snafu(implicit)]
        localtion: Location,
    },
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
    #[snafu(display("Not op but stack is empty"))]
    NotEmptyStack {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Too many consts"))]
    TooManyConsts {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("{msg}"))]
    NotMatch {
        msg: &'static str,
        token: Option<Token>,
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Missing previous"))]
    MissingPrev {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Missing current"))]
    MissingCur {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Missing infix parse fn"))]
    MissingInfix {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Expect expression"))]
    NotExpression {
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Negate operand must be a number: line: {line}"))]
    NegateNotNum {
        line: usize,
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("+, -, *, / operand must be a number: line: {line}"))]
    BinaryNotNum {
        line: usize,
        #[snafu(implicit)]
        localtion: Location,
    },
    #[snafu(display("Undefined variable {}",name))]
    UndefindVar {
        name: String,
        #[snafu(implicit)]
        localtion: Location,
    },
}

pub type Result<T> = std::result::Result<T, LoxError>;
