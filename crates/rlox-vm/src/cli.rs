use clap::Parser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, value_name = "PATH")]
    pub(crate) file_path: Option<String>,
    #[arg(short, long)]
    pub(crate) repl: bool,
}

impl Cli {
    pub const fn repl(&self) -> bool {
        self.repl
    }

    pub const fn file_path(&self) -> Option<&String> {
        self.file_path.as_ref()
    }
}
