use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Shell {
    Zsh,
    Bash,
    Fish,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub use_zellij: bool,
    pub default_shell: Option<Shell>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_zellij: false,
            default_shell: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::path();
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn path() -> PathBuf {
        config_dir().join("config.json")
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "use_zellij" => Some(self.use_zellij.to_string()),
            "default_shell" => self.default_shell.map(|s| s.to_string()),
            _ => None,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "use_zellij" => {
                self.use_zellij = value.parse()?;
            }
            "default_shell" => {
                self.default_shell = Some(value.parse()?);
            }
            _ => anyhow::bail!("Unknown config key: {}", key),
        }
        Ok(())
    }
}

pub fn config_dir() -> PathBuf {
    // Respect XDG_CONFIG_HOME if set (useful for tests)
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join("spm");
    }

    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("spm")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.use_zellij);
        assert!(config.default_shell.is_none());
    }

    #[test]
    fn test_config_get() {
        let config = Config {
            use_zellij: true,
            default_shell: Some(Shell::Fish),
        };
        assert_eq!(config.get("use_zellij"), Some("true".to_string()));
        assert_eq!(config.get("default_shell"), Some("fish".to_string()));
        assert_eq!(config.get("unknown"), None);
    }

    #[test]
    fn test_config_set() {
        let mut config = Config::default();
        config.set("use_zellij", "true").unwrap();
        assert!(config.use_zellij);

        config.set("default_shell", "zsh").unwrap();
        assert_eq!(config.default_shell, Some(Shell::Zsh));
    }

    #[test]
    fn test_config_set_invalid_key() {
        let mut config = Config::default();
        let result = config.set("invalid_key", "value");
        assert!(result.is_err());
    }

    #[test]
    fn test_shell_display() {
        assert_eq!(Shell::Zsh.to_string(), "zsh");
        assert_eq!(Shell::Bash.to_string(), "bash");
        assert_eq!(Shell::Fish.to_string(), "fish");
    }

    #[test]
    fn test_shell_parse() {
        assert_eq!("zsh".parse::<Shell>().unwrap(), Shell::Zsh);
        assert_eq!("bash".parse::<Shell>().unwrap(), Shell::Bash);
        assert_eq!("fish".parse::<Shell>().unwrap(), Shell::Fish);
    }
}
