use std::{process::{Output, Command}, io, path::PathBuf};

use config::{Config, ConfigError};
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Group {
    pub group_id: String,
    pub sequences: Vec<Sequence>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Sequence {
    pub keys: Vec<String>,
    pub action: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debounce_time: u64,
    pub groups: Vec<Group>,
}

impl Sequence {
    pub fn execute(&self) -> io::Result<Output> {
        let action: Vec<&str> = self.action.split(" ").collect();
        Command::new(&action[0]).args(&action[1..]).output()
    }
}

impl Settings {
    pub fn new(config: &Option<PathBuf>, default: &str) -> Result<Self, ConfigError> {
        let settings_path = &config.clone().unwrap_or_else(|| {
            let mut exe_path = std::env::current_exe().expect("Failed to get current executable path");
            exe_path.pop(); // Remove the executable name
            exe_path.push(default); // Append the default configuration file name
            exe_path
        });

        let s = Config::builder()
            .add_source(config::File::from(settings_path.clone()))
            .build()?;

        s.try_deserialize()
    }
}
