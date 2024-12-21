use std::{collections::HashMap, fs, path::PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ServiceConfig {
    image: String,
    ports: Option<Vec<String>>,
    environment: Option<Vec<String>>, 
    volumes: Option<Vec<String>>,
    command: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct ServiceConfigManager {
    configs: HashMap<String, ServiceConfig>,
}

impl ServiceConfigManager {
    /// Creates a new ServiceConfigManager by loading service configurations from a file.
    pub fn new(config_file_path: PathBuf) -> Result<Self, std::io::Error> {
        let base_dir: PathBuf = std::env::current_dir().expect("Failed to get current directory");
        let service_config_path = base_dir.join(config_file_path);

        let file_content = fs::read_to_string(service_config_path)?;
        let configs: HashMap<String, ServiceConfig> = serde_json::from_str(&file_content)?;

        Ok(Self { configs })
    }

    /// Finds a service configuration by its name.
    pub fn find_service_config_by_name(&self, name: &str) -> Option<&ServiceConfig> {
        self.configs.get(name)
    }
}