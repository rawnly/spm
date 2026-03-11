use crate::config::config_dir;
use crate::project::Project;
use std::collections::BinaryHeap;
use std::path::PathBuf;
use std::result::Result;
use std::{fs, io};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("project with name '{0}' already exists")]
    DuplicateProjectName(String),
    #[error("project with path '{0}' already exists")]
    DuplicateProjectPath(String),
    #[error("project '{0}' not found")]
    ProjectNotFound(String),

    #[error(transparent)]
    FromJson(#[from] serde_json::Error),

    #[error(transparent)]
    IOError(#[from] io::Error),
}

#[derive(Debug, Clone)]
pub struct Storage {
    projects: Vec<Project>,
}

impl Storage {
    pub fn load() -> Result<Self, StorageError> {
        let path = Self::path();
        let projects = if path.exists() {
            let content = fs::read_to_string(&path)?;
            serde_json::from_str(&content)?
        } else {
            Vec::new()
        };
        Ok(Self { projects })
    }

    pub fn save(&self) -> Result<(), StorageError> {
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

    pub fn add(&mut self, project: Project) -> Result<(), StorageError> {
        if self.find_by_name(&project.name).is_some() {
            return Err(StorageError::DuplicateProjectName(project.name));
        }
        if self.find_by_path(&project.path).is_some() {
            return Err(StorageError::DuplicateProjectPath(
                project.path.to_string_lossy().to_string(),
            ));
        }
        self.projects.push(project);
        self.save()
    }

    pub fn remove_all(&mut self) -> Result<(), StorageError> {
        self.projects = vec![];
        self.save()
    }

    pub fn remove_all_filtered(&mut self, tags: &[String]) -> Result<(), StorageError> {
        self.projects.retain(|project| project.has_any_tag(tags));
        self.save()
    }

    pub fn remove(&mut self, name: &str) -> Result<(), StorageError> {
        let len_before = self.projects.len();
        self.projects.retain(|p| p.name != name);
        if self.projects.len() == len_before {
            return Err(StorageError::ProjectNotFound(name.to_string()));
        }
        self.save()
    }

    pub fn list(&self) -> Vec<&Project> {
        let v: Vec<&Project> = self.projects.iter().collect();
        let bh: BinaryHeap<&Project> = BinaryHeap::from(v);

        bh.into_vec()
    }

    pub fn list_filtered(&self, tags: &[String]) -> Vec<&Project> {
        if tags.is_empty() {
            self.list()
        } else {
            let v: Vec<&Project> = self
                .projects
                .iter()
                .filter(|p| p.has_any_tag(tags))
                .collect();
            let bh = BinaryHeap::from(v);

            bh.into_vec()
        }
    }

    pub fn update_access(&mut self, name: &str) -> Result<(), StorageError> {
        self.update(name, |p| p.on_access())
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

    pub fn update<F>(&mut self, name: &str, f: F) -> Result<(), StorageError>
    where
        F: FnOnce(&mut Project),
    {
        let project = self
            .find_by_name_mut(name)
            .ok_or_else(|| StorageError::ProjectNotFound(name.to_string()))?;
        f(project);
        self.save()
    }
}
