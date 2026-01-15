use crate::config::config_dir;
use crate::project::Project;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct Storage {
    projects: Vec<Project>,
}

impl Storage {
    pub fn load() -> Result<Self> {
        let path = Self::path();
        let projects = if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };
        Ok(Self { projects })
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.projects)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn path() -> PathBuf {
        config_dir().join("projects.json")
    }

    pub fn add(&mut self, project: Project) -> Result<()> {
        if self.find_by_name(&project.name).is_some() {
            anyhow::bail!("Project '{}' already exists", project.name);
        }
        if self.find_by_path(&project.path).is_some() {
            anyhow::bail!("Project at path '{}' already exists", project.path.display());
        }
        self.projects.push(project);
        self.save()
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        let len_before = self.projects.len();
        self.projects.retain(|p| p.name != name);
        if self.projects.len() == len_before {
            anyhow::bail!("Project '{}' not found", name);
        }
        self.save()
    }

    pub fn list(&self) -> &[Project] {
        &self.projects
    }

    pub fn list_filtered(&self, tags: &[String]) -> Vec<&Project> {
        if tags.is_empty() {
            self.projects.iter().collect()
        } else {
            self.projects.iter().filter(|p| p.has_any_tag(tags)).collect()
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.name == name)
    }

    pub fn find_by_name_mut(&mut self, name: &str) -> Option<&mut Project> {
        self.projects.iter_mut().find(|p| p.name == name)
    }

    pub fn find_by_path(&self, path: &PathBuf) -> Option<&Project> {
        self.projects.iter().find(|p| p.path == *path)
    }

    pub fn update<F>(&mut self, name: &str, f: F) -> Result<()>
    where
        F: FnOnce(&mut Project),
    {
        let project = self
            .find_by_name_mut(name)
            .ok_or_else(|| anyhow::anyhow!("Project '{}' not found", name))?;
        f(project);
        self.save()
    }
}
