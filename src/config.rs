use serde::{Deserialize, Serialize};
use std::fs;
use crate::error::LuminaError;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RouteConfig {
    pub path: String,
    pub upstream: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
}

impl AppConfig {
    pub fn load(path: &str) -> Result<Self, LuminaError> {
        let contents = fs::read_to_string(path)?;
        let config: AppConfig = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}
