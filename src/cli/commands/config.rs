use crate::{cli::ConfigAction, config::Config, storage::Storage};

pub fn config(action: ConfigAction) -> Result<()> {
    match action {
        ConfigAction::View => {
            let config = Config::load()?;

            let storage_path = Storage::path();
            let config_path = Config::path();

            println!("Config Path: {}", config_path.display());
            println!("Storage Path: {}", storage_path.display());

            println!();

            let json = serde_json::to_string_pretty(&config)?;
            println!("{json}");
        }
        ConfigAction::Get { key } => {
            let config = Config::load()?;
            match config.get(&key) {
                Some(value) => println!("{}", value),
                None => println!("(not set)"),
            }
        }
        ConfigAction::Set { key, value } => {
            let mut config = Config::load()?;
            config.set(&key, &value)?;
            config.save()?;
            println!("{}={}", key, value);
        }
    }

    Ok(())
}
