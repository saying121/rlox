use clap::Parser;
use rlox::{cli, lox::Lox, prompt};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .with_test_writer()
        .init();

    let cli = cli::Cli::parse();

    if cli.prompt() {
        prompt::run_prompt()?;
    }
    else if let Some(fp) = cli.file_path {
        let lox = Lox::default();
        if let Err(e) = lox.run_file(&fp) {
            tracing::error!("{}", e);
        }
    }
    else if let Some(fp) = cli.ast {
        let lox = Lox::default();
        if let Err(e) = lox.ast_file(&fp) {
            tracing::error!("{}", e);
        }
    }

    Ok(())
}
