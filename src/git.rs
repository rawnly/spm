use anyhow::Result;
use git2::Repository;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn get_repo_root(path: &Path) -> Result<PathBuf> {
    let repo = Repository::discover(path)?;
    if repo.is_bare() {
        Ok(repo.path().to_path_buf())
    } else if let Some(workdir) = repo.workdir() {
        Ok(workdir.to_path_buf())
    } else {
        Ok(repo.path().to_path_buf())
    }
}

pub fn is_bare_repo(path: &Path) -> bool {
    Repository::open(path)
        .map(|repo| repo.is_bare())
        .unwrap_or(false)
}

pub fn list_worktrees(bare_repo_path: &Path) -> Result<Vec<Worktree>> {
    let repo = Repository::open(bare_repo_path)?;
    let mut worktrees = Vec::new();

    for name in repo.worktrees()?.iter().flatten() {
        if let Ok(wt) = repo.find_worktree(name) {
            let wtrepo = Repository::open(wt.path())?;

            worktrees.push(Worktree {
                name: name.to_string(),
                path: wt.path().to_path_buf(),
                branch: wtrepo.head()?.name().map(|s| s.to_string()),
            });
        }
    }

    Ok(worktrees)
}

#[derive(Debug, Clone)]
pub struct Worktree {
    pub name: String,
    pub path: PathBuf,
    pub branch: Option<String>,
}

impl std::fmt::Display for Worktree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(branch) = &self.branch {
            return write!(f, "{} [{}]", self.name, branch.replace("refs/heads/", ""));
        }

        write!(f, "{} ({})", self.name, self.path.display())
    }
}
