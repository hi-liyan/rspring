use crate::error::{Error, Result};
use config::{Config, ConfigError, Environment, File};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub trait Configuration: DeserializeOwned + Send + Sync + 'static {}

#[derive(Debug)]
pub struct ConfigurationManager {
    config: Config,
}

impl ConfigurationManager {
    pub fn new() -> Result<Self> {
        let mut config = Config::builder()
            // Start with default configuration
            .add_source(File::with_name("application").required(false))
            // Add environment-specific configuration
            .add_source(File::with_name(&format!("application-{}", 
                std::env::var("PROFILE").unwrap_or_else(|_| "dev".to_string())
            )).required(false))
            // Override with environment variables
            .add_source(Environment::with_prefix("AXUM_BOOT").separator("_"))
            .build()?;
        
        Ok(Self { config })
    }
    
    pub fn get<T>(&self, key: &str) -> Result<T> 
    where 
        T: DeserializeOwned,
    {
        self.config.get(key).map_err(Error::Configuration)
    }
    
    pub fn bind<T>(&self) -> Result<T>
    where
        T: Configuration,
    {
        self.config.try_deserialize().map_err(Error::Configuration)
    }
    
    pub fn get_string(&self, key: &str) -> Result<String> {
        self.get(key)
    }
    
    pub fn get_int(&self, key: &str) -> Result<i64> {
        self.get(key)
    }
    
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get(key)
    }
}

impl Default for ConfigurationManager {
    fn default() -> Self {
        Self::new().expect("Failed to create configuration manager")
    }
}

// Common configuration structures
#[derive(serde::Deserialize, Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

impl Configuration for ServerConfig {}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mysql://root:password@localhost:3306/axumboot".to_string(),
            max_connections: 10,
            min_connections: 5,
        }
    }
}

impl Configuration for DatabaseConfig {}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            pool_size: 10,
        }
    }
}

impl Configuration for RedisConfig {}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub pattern: Option<String>,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            pattern: Some("%d{ISO8601} [%t] %-5level %logger{36} - %msg%n".to_string()),
        }
    }
}

impl Configuration for LoggingConfig {}