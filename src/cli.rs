use crate::config::Shell;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "spm")]
#[command(about = "Side Project Manager - manage your projects", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a project
    Add {
        /// Project path (default: current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Project name (default: folder name)
        #[arg(short, long)]
        name: Option<String>,

        /// Comma-separated tags
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// List all projects
    List {
        /// Filter by tags
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Interactively select a project
    Pick,

    /// Remove a project
    #[command(alias = "rm")]
    Remove {
        /// Project name to remove
        name: Option<String>,
    },

    /// Generate shell hooks
    Init {
        /// Shell to generate hooks for
        shell: Shell,
    },

    /// Manage project tags
    Tag {
        /// Project name
        project: String,

        /// Tags to add
        tags: Vec<String>,

        /// Remove tags instead of adding them
        #[arg(short, long)]
        remove: bool,
    },

    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Get a configuration value
    Get {
        /// Key to read (default_shell)
        key: String,
    },

    /// Set a configuration value
    Set {
        /// Key to set
        key: String,
        /// Value to set
        value: String,
    },
}
