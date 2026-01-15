use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub is_bare_repo: bool,
    pub added_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: String, path: PathBuf, is_bare_repo: bool) -> Self {
        Self {
            name,
            path,
            tags: Vec::new(),
            is_bare_repo,
            added_at: Utc::now(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }

    pub fn has_any_tag(&self, tags: &[String]) -> bool {
        tags.iter().any(|t| self.has_tag(t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_project() -> Project {
        Project::new(
            "test-project".to_string(),
            PathBuf::from("/tmp/test"),
            false,
        )
    }

    #[test]
    fn test_project_creation() {
        let project = sample_project();
        assert_eq!(project.name, "test-project");
        assert_eq!(project.path, PathBuf::from("/tmp/test"));
        assert!(!project.is_bare_repo);
        assert!(project.tags.is_empty());
    }

    #[test]
    fn test_with_tags() {
        let project = sample_project().with_tags(vec!["rust".to_string(), "cli".to_string()]);
        assert_eq!(project.tags.len(), 2);
        assert!(project.tags.contains(&"rust".to_string()));
    }

    #[test]
    fn test_add_tag() {
        let mut project = sample_project();
        project.add_tag("new-tag".to_string());
        assert!(project.has_tag("new-tag"));

        // Adding duplicate should not create duplicates
        project.add_tag("new-tag".to_string());
        assert_eq!(project.tags.len(), 1);
    }

    #[test]
    fn test_remove_tag() {
        let mut project = sample_project().with_tags(vec!["a".to_string(), "b".to_string()]);
        project.remove_tag("a");
        assert!(!project.has_tag("a"));
        assert!(project.has_tag("b"));
    }

    #[test]
    fn test_has_any_tag() {
        let project = sample_project().with_tags(vec!["rust".to_string(), "cli".to_string()]);
        assert!(project.has_any_tag(&["rust".to_string()]));
        assert!(project.has_any_tag(&["python".to_string(), "cli".to_string()]));
        assert!(!project.has_any_tag(&["python".to_string(), "js".to_string()]));
    }
}
