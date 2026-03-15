use crate::{fuzzy_scorer, git, git::Worktree, storage::Storage, Project};
use anyhow::Result;
use inquire::{
    ui::{RenderConfig, Styled},
    Select,
};

pub fn pick(query: Option<String>, tags: Option<Vec<String>>) -> Result<()> {
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
        let binary = env!("CARGO_BIN_NAME");

        println!("WARN - Some projects points to non-existing path");
        println!("run `{binary} list` to show broken projects")
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
