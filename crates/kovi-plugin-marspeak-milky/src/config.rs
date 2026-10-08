use serde::{Deserialize, Serialize};
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub prefix: String,
    table_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            prefix: "/mp".to_string(),
            table_path: None,
        }
    }
}
