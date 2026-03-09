use crate::storage::Storage;
use anyhow::Result;
use inquire::Select;

pub fn tag(project_name: Option<String>, tags: Vec<String>, remove: bool) -> Result<()> {
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
