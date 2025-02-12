use serde::Deserialize;
use config::{Config, ConfigError, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub arch_node: ArchNodeSettings,
    pub redis: RedisSettings,
    pub indexer: IndexerSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    #[serde(default = "default_cors_origin")]
    pub cors_allow_origin: String,
    #[serde(default = "default_cors_methods")]
    pub cors_allow_methods: String,
    #[serde(default = "default_cors_headers")]
    pub cors_allow_headers: String,
}

// Default functions for CORS settings
fn default_cors_origin() -> String {
    "*".to_string()
}

fn default_cors_methods() -> String {
    "GET, POST, OPTIONS".to_string()
}

fn default_cors_headers() -> String {
    "Content-Type, Authorization".to_string()
}

#[derive(Debug, Deserialize)]
pub struct ArchNodeSettings {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisSettings {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct IndexerSettings {
    pub batch_size: usize,
    pub concurrent_batches: usize,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Add debug prints
        println!("Environment variables:");
        for (key, value) in std::env::vars() {
            println!("{}: {}", key, value);
        }
        
        let config = Config::builder()
            .add_source(Environment::default().separator("__"))
            // Add default values for critical settings
            .set_default("application.host", "0.0.0.0")?
            .set_default("application.port", 8080)?
            .set_default("indexer.batch_size", 100)?
            .set_default("indexer.concurrent_batches", 5)?
            .build()?;

        config.try_deserialize()
    }
}