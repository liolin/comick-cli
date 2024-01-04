use anyhow::Result;
use clap::{Parser, Subcommand};
use std::env;
use tracing::error;

mod api;
mod download;
mod archive;

use download::Download;
use archive::Archive;

#[derive(Debug, Parser)]
#[clap(author, version = version(), about)]
struct Cli {
    #[clap(flatten)]
    verbosity: Option<Verbosity>,

    #[clap(subcommand)]
    command: Command,
}

fn version() -> String {
    let package_version = env!("CARGO_PKG_VERSION");
    let git_commit_hash = env!("GIT_HASH");
    let build_date = env!("BUILD_DATE");

    if git_commit_hash.is_empty() {
        package_version.to_string()
    } else {
        format!("{} ({} {})", package_version, git_commit_hash, build_date)
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    Archive(Archive),
    Download(Download),
}

#[derive(Debug, Parser)]
struct Verbosity {
    #[arg(help = "Verbose output")]
    #[arg(global = true, short, long)]
    verbose: bool,

    #[arg(help = "Quiet output. Does not print anything unless it's a error")]
    #[arg(
        long_help = "Quiet output. Does not print anything unless it's a error. Can be helpful if you pipe the output to stdout"
    )]
    #[arg(global = true, short, long)]
    quiet: bool,
}

pub async fn cli_entrypoint() {
    let cli = Cli::parse();
    match cli.command {
        Command::Archive(archive) => execute_executor(archive).await,
        Command::Download(download) => execute_executor(download).await,
    }
}

#[async_trait::async_trait]
trait Execute {
    fn pre_check(&self) -> Result<()> {
        Ok(())
    }

    async fn execute(self) -> Result<()>;
}

async fn execute_executor(executor: impl Execute) {
    if let Err(err) = executor.execute().await {
        error!("An error occurred: {}", err);
        std::process::exit(1)
    }
}
