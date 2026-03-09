use crate::storage::Storage;
use anyhow::Result;

pub fn list(tags: Option<Vec<String>>, json: bool) -> Result<()> {
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
