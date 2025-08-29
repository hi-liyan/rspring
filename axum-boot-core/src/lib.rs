pub mod application;
pub mod config;
pub mod container;
pub mod error;
pub mod logging;

// Re-export common types and traits
pub use application::{AxumBootApplication, ApplicationContext};
pub use config::{Configuration, ConfigurationManager};
pub use container::{Container, Component, Service, Repository, Controller};
pub use error::{Error, Result};

// Re-export macros
pub use axum_boot_macro::*;

// Common response types
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

// Pagination support
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page: u64,
    pub size: u64,
}

impl Default for Page {
    fn default() -> Self {
        Self { page: 0, size: 20 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T> {
    pub content: Vec<T>,
    pub page: u64,
    pub size: u64,
    pub total: u64,
    pub total_pages: u64,
}

impl<T> PageResult<T> {
    pub fn new(content: Vec<T>, page: u64, size: u64, total: u64) -> Self {
        let total_pages = if size > 0 { (total + size - 1) / size } else { 0 };
        Self {
            content,
            page,
            size,
            total,
            total_pages,
        }
    }
}