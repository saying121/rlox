use clap::Parser;
use rlox::lox::Lox;
use rlox::{cli, prompt, run_file};

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    if cli.prompt() {
        prompt::run_prompt()?;
    }
    else if let Some(fp) = cli.file_path {
        Lox::run(fp)?;
    }

    Ok(())
}
