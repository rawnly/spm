mod cli;
mod config;
mod git;
mod project;
mod shell;
mod storage;
mod utils;
mod version_check;

/// Creates a fuzzy matcher and scorer variable for use with `inquire::Select`.
/// Spaces in the input are stripped so "my proj" matches "my-project".
///
/// Usage: `fuzzy_scorer!(scorer_name, Type);`
macro_rules! fuzzy_scorer {
    ($name:ident, $T:ty) => {
        let matcher =
            ::std::cell::RefCell::new(::frizbee::Matcher::new("", &::frizbee::Config::default()));
        let $name: ::inquire::type_aliases::Scorer<$T> = &|input, _, str_val, _| {
            if input.is_empty() {
                return Some(0);
            }
            let needle = input.replace(' ', "");
            let mut m = matcher.borrow_mut();
            m.set_needle(&needle);
            m.smith_waterman_one(str_val.as_bytes(), 0, true)
                .map(|r| r.score as i64)
        };
    };
}

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command, ConfigAction};
use config::Config;
use inquire::ui::{RenderConfig, Styled};
use inquire::{Confirm, Select};
use project::Project;
use std::path::PathBuf;
use storage::Storage;

use crate::git::Worktree;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let check_process = tokio::spawn(async { version_check::is_update_available().await });

    match cli.command.clone() {
        Command::Add { path, name, tags } => cmd_add(path, name, tags),
        Command::List { tags, json } => cmd_list(tags, json),
        Command::Pick { tags, query } => cmd_pick(query, tags),
        Command::Remove { name, all, tags } => cmd_remove(name, tags, all),
        Command::Init { shell } => cmd_init(shell),
        Command::Tag {
            project,
            tags,
            remove,
        } => cmd_tag(project, tags, remove),
        Command::Config { action } => cmd_config(action),
        Command::CheckUpdate => {
            if let Some(v) = version_check::is_update_available().await? {
                println!("A new version is available: {v}")
            } else {
                println!("Congrats! You're on the latest available version.")
            }

            Ok(())
        }
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

fn cmd_list(tags: Option<Vec<String>>, json: bool) -> Result<()> {
    let storage = Storage::load()?;
    let tags = tags.unwrap_or_default();
    let projects = storage.list_filtered(&tags);
    let json_projects = serde_json::to_string(&projects)?;

    if projects.is_empty() {
        if json {
            println!("{}", json_projects);
        } else {
            println!("No projects found");
        }

        return Ok(());
    }

    for project in projects {
        let tags_str = if project.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", project.tags.join(", "))
        };

        let bare_indicator = if project.is_bare_repo { " (bare)" } else { "" };
        let broken_indicator = if project.exists() { "" } else { "!" };

        if !json {
            println!(
                "{}{}{} - {}{}",
                broken_indicator,
                project.name,
                bare_indicator,
                project.path.display(),
                tags_str
            );
            continue;
        }
    }

    if json {
        println!("{}", json_projects);
    }

    Ok(())
}

fn cmd_pick(query: Option<String>, tags: Option<Vec<String>>) -> Result<()> {
    let mut storage = Storage::load()?;

    let projects: Vec<Project> = match tags {
        None => storage.list().into_iter().cloned().collect(),
        Some(tags) => storage.list_filtered(&tags).into_iter().cloned().collect(),
    };

    if projects.is_empty() {
        eprintln!("No projects available");
        std::process::exit(1);
    }

    if projects.iter().any(|p| !p.exists()) {
        println!("WARN - Some projects points to non-existing path");
        println!("use `spm list` to show broken projects")
    }

    fuzzy_scorer!(fuzzy_project_scorer, Project);

    let prompt_project_selection = |projects: &[Project], q: Option<String>| {
        Select::new("Select a project:", projects.to_vec())
            .with_starting_filter_input(&q.unwrap_or_default())
            .with_vim_mode(false)
            .with_scorer(&fuzzy_project_scorer)
            .with_render_config(
                RenderConfig::default_colored()
                    .with_scroll_up_prefix(Styled::new("↑"))
                    .with_scroll_down_prefix(Styled::new("↓")),
            )
            .prompt()
    };

    let project = if let Some(ref q) = query {
        let names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();
        let fuzzy_filtered = frizbee::match_list_indices(q, &names, &frizbee::Config::default());
        let pre_filtered: Vec<&Project> = fuzzy_filtered
            .iter()
            .filter_map(|m| projects.get(m.index as usize))
            .collect();

        if pre_filtered.len() == 1 {
            pre_filtered[0].clone()
        } else if pre_filtered.len() > 1 {
            prompt_project_selection(&projects, Some(q.clone()))?
        } else {
            prompt_project_selection(&projects, None)?
        }
    } else {
        prompt_project_selection(&projects, None)?
    };

    // If bare repo, show available worktrees
    let final_path = if project.is_bare_repo {
        match git::list_worktrees(&project.path) {
            Ok(worktrees) if worktrees.len() == 1 => worktrees.first().unwrap().path.clone(),
            Ok(worktrees) if !worktrees.is_empty() => {
                fuzzy_scorer!(fuzzy_worktree_scorer, Worktree);

                let wt_selection = Select::new("Select a worktree:", worktrees)
                    .with_scorer(&fuzzy_worktree_scorer)
                    .with_help_message("<ESC> to skip this and navigate to the project root")
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

    storage.update_access(&project.name)?;
    println!("{}", final_path.display());

    Ok(())
}

fn cmd_remove(name: Option<String>, tags: Option<Vec<String>>, all: bool) -> Result<()> {
    let mut storage = Storage::load()?;

    if all {
        if !Confirm::new("Do you really want to remove all projects?").prompt()? {
            println!("operation aborted by the user.");
            return Ok(());
        }

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

            fuzzy_scorer!(fuzzy_project_scorer, &Project);
            let selected_project = Select::new("select a project:", projects)
                .with_scorer(&fuzzy_project_scorer)
                .prompt()?;

            selected_project.name.clone()
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
        ConfigAction::View => {
            let config = Config::load()?;

            let storage_path = Storage::path();
            let config_path = Config::path();

            println!("Config Path: {}", config_path.display());
            println!("Storage Path: {}", storage_path.display());

            println!();

            let json = serde_json::to_string_pretty(&config)?;
            println!("{json}");
        }
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
