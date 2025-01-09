use clap::Parser;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, value_name = "PATH")]
    pub file_path: Option<String>,
    #[arg(short, long)]
    pub(crate) prompt: bool,
}

impl Cli {
    pub const fn prompt(&self) -> bool {
        self.prompt
    }
}
