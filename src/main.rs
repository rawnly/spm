mod cli;
mod config;
mod git;
mod project;
mod shell;
mod storage;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, ConfigAction};
use config::Config;
use inquire::Select;
use project::Project;
use std::path::PathBuf;
use storage::Storage;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Add { path, name, tags } => cmd_add(path, name, tags),
        Command::List { tags } => cmd_list(tags),
        Command::Pick { tags } => cmd_pick(tags),
        Command::Remove { name, all, tags } => cmd_remove(name, tags, all),
        Command::Init { shell } => cmd_init(shell),
        Command::Tag {
            project,
            tags,
            remove,
        } => cmd_tag(project, tags, remove),
        Command::Config { action } => cmd_config(action),
    }
}

fn cmd_add(path: PathBuf, name: Option<String>, tags: Option<Vec<String>>) -> Result<()> {
    let path = std::fs::canonicalize(&path)?;

    let name = name.unwrap_or_else(|| {
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string()
    });

    let is_bare = git::is_bare_repo(&path);
    let mut project = Project::new(name.clone(), path.clone(), is_bare);

    if let Some(tags) = tags {
        project = project.with_tags(tags);
    }

    let mut storage = Storage::load()?;
    storage.add(project)?;

    println!("Project '{}' added", name);
    if is_bare {
        println!("  (bare repository detected)");
    }
    println!("  Path: {}", path.display());

    Ok(())
}

fn cmd_list(tags: Option<Vec<String>>) -> Result<()> {
    let storage = Storage::load()?;
    let tags = tags.unwrap_or_default();
    let projects = storage.list_filtered(&tags);

    if projects.is_empty() {
        println!("No projects found");
        return Ok(());
    }

    for project in projects {
        let tags_str = if project.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", project.tags.join(", "))
        };

        let bare_indicator = if project.is_bare_repo { " (bare)" } else { "" };

        println!(
            "{}{} - {}{}",
            project.name,
            bare_indicator,
            project.path.display(),
            tags_str
        );
    }

    Ok(())
}

fn cmd_pick(tags: Option<Vec<String>>) -> Result<()> {
    let storage = Storage::load()?;

    let projects = match tags {
        None => storage.list(),
        Some(tags) => storage.list_filtered(&tags),
    };

    if projects.is_empty() {
        eprintln!("No projects available");
        std::process::exit(1);
    }

    let project = Select::new("Select a project:", projects)
        .with_vim_mode(true)
        .prompt()?;

    // If bare repo, show available worktrees
    let final_path = if project.is_bare_repo {
        match git::list_worktrees(&project.path) {
            Ok(worktrees) if !worktrees.is_empty() => {
                let wt_selection = Select::new("Select a worktree:", worktrees)
                    .with_help_message("you can skip this to pick the project root")
                    .with_vim_mode(false);

                match wt_selection.prompt_skippable() {
                    Ok(Some(selected)) => selected.path.clone(),
                    Err(_) => std::process::exit(1),
                    _ => project.path.clone(),
                }
            }
            _ => project.path.clone(),
        }
    } else {
        project.path.clone()
    };

    println!("{}", final_path.display());

    Ok(())
}

fn cmd_remove(name: Option<String>, tags: Option<Vec<String>>, all: bool) -> Result<()> {
    let mut storage = Storage::load()?;

    if all {
        match tags {
            None => {
                for project in storage.list() {
                    println!("Project {} removed", &project.name);
                }

                return storage.remove_all();
            }
            Some(tags) => {
                for project in storage.list_filtered(&tags) {
                    println!("Project {} removed", &project.name);
                }

                return storage.remove_all_filtered(&tags);
            }
        }
    }

    let name = match name {
        Some(n) => n,
        None => {
            let projects = storage.list();
            if projects.is_empty() {
                println!("No projects to remove");
                return Ok(());
            }

            let options: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();

            Select::new("Select project to remove:", options)
                .with_vim_mode(true)
                .prompt()?
        }
    };

    storage.remove(&name)?;
    println!("Project '{}' removed", name);

    Ok(())
}

fn cmd_init(shell: config::Shell) -> Result<()> {
    let hook = shell::generate_hook(shell);
    println!("{}", hook);
    Ok(())
}

fn cmd_tag(project_name: Option<String>, tags: Vec<String>, remove: bool) -> Result<()> {
    let mut storage = Storage::load()?;

    let project_name = match project_name {
        Some(name) => name,
        None => {
            let project = Select::new("Select a project:", storage.list()).prompt()?;

            project.clone().name
        }
    };

    storage.update(&project_name, |project| {
        for tag in &tags {
            if remove {
                project.remove_tag(tag);
            } else {
                project.add_tag(tag.clone());
            }
        }
    })?;

    let action = if remove { "removed from" } else { "added to" };
    println!("Tags {} '{}'", action, project_name);

    Ok(())
}

fn cmd_config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::Get { key } => {
            let config = Config::load()?;
            match config.get(&key) {
                Some(value) => println!("{}", value),
                None => println!("(not set)"),
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = Config::load()?;
            config.set(&key, &value)?;
            config.save()?;
            println!("{}={}", key, value);
        }
    }

    Ok(())
}
