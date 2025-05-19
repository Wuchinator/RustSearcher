use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub excluded_paths: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Option<Self> {
        fs::read_to_string("searcher_config.json")
            .ok()
            .and_then(|config| serde_json::from_str(&config).ok())
    }

    pub fn save(&self) -> std::io::Result<()> {
        fs::write("searcher_config.json", serde_json::to_string(self)?)
    }
}