use clap::Parser;
use rlox::{cli, lox::Lox, prompt};

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
