use crate::{fuzzy_scorer, storage::Storage, Project};
use inquire::{Confirm, Select};

pub fn remove(name: Option<String>, tags: Option<Vec<String>>, all: bool) -> Result<()> {
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
