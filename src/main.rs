mod cli;
mod config;
mod git;
mod project;
mod shell;
mod storage;
mod utils;
mod version_check;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use project::Project;
use storage::Storage;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let check_process = tokio::spawn(async { version_check::is_update_available().await });

    match cli.command.clone() {
        Command::Add { path, name, tags } => cli::commands::add(path, name, tags),
        Command::List { tags, json } => cli::commands::list(tags, json),
        Command::Pick { tags, query } => cli::commands::pick(query, tags),
        Command::Remove { name, all, tags } => cli::commands::remove(name, tags, all),
        Command::Tag {
            project,
            tags,
            remove,
        } => cli::commands::tag(project, tags, remove),
        Command::Config { action } => cli::commands::config(action),
        Command::CheckUpdate => {
            if let Some(v) = version_check::is_update_available().await? {
                println!("A new version is available: {v}")
            } else {
                println!("Congrats! You're on the latest available version.")
            }

            Ok(())
        }
        Command::Init { shell } => cmd_init(shell),
    }?;

    if !matches!(cli.command, Command::CheckUpdate) {
        if let Some(latest) = check_process.await?? {
            println!();
            println!("A new update is available: {latest}");
            println!("Please update via: `brew update spm`");
            println!();
        }
    }

    Ok(())
}

fn cmd_init(shell: config::Shell) -> Result<()> {
    let hook = shell::generate_hook(shell);
    println!("{}", hook);
    Ok(())
}
