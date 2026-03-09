use crate::{git, Project, Storage};
use anyhow::Result;
use std::path::PathBuf;

pub fn add(path: PathBuf, name: Option<String>, tags: Option<Vec<String>>) -> Result<()> {
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
