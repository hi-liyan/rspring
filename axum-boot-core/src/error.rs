use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Configuration(#[from] config::ConfigError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Business error: {message}")]
    Business { message: String, code: String },
    
    #[error("Resource not found: {resource}")]
    NotFound { resource: String },
    
    #[error("Unauthorized access")]
    Unauthorized,
    
    #[error("Internal server error: {message}")]
    Internal { message: String },
}

impl Error {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation { message: message.into() }
    }
    
    pub fn business(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Business { 
            code: code.into(), 
            message: message.into() 
        }
    }
    
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound { resource: resource.into() }
    }
    
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal { message: message.into() }
    }
}