use crate::config::Shell;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use strum::EnumIter;

pub mod commands;

#[derive(Parser, Clone)]
#[command(name = "bvo")]
#[command(
    about = "Bivio - your project wayfinder",
    long_about = "Bivio is a fast project navigator with tags, fuzzy pickers, and first-class worktree support."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Clone, EnumIter, strum_macros::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum Command {
    /// Register a project (path, name, tags)
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

    /// List projects (optionally filtered by tags)
    List {
        /// Filter by tags
        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        #[arg(long)]
        json: bool,
    },

    /// Interactive picker with fuzzy search
    Pick {
        /// Search project
        query: Option<String>,

        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,
    },

    /// Remove a project (alias: rm)
    #[command(alias = "rm")]
    Remove {
        #[arg(short, long)]
        all: bool,

        #[arg(short, long, value_delimiter = ',')]
        tags: Option<Vec<String>>,

        /// Project name to remove
        name: Option<String>,
    },

    /// Print shell integration hooks
    Init {
        /// Shell to generate hooks for
        shell: Option<Shell>,
    },

    /// Add or remove project tags
    Tag {
        /// Project name
        project: Option<String>,

        /// Tags to add
        tags: Vec<String>,

        /// Remove tags instead of adding them
        #[arg(short, long)]
        remove: bool,
    },

    /// Read or update configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Check for new releases
    CheckUpdate,
}

#[derive(Subcommand, Clone, Default)]
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

    /// Prints current configuration
    #[default]
    View,
}
