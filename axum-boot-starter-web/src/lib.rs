pub mod controller;
pub mod macros;
pub mod response;

// Re-export core functionality
pub use axum_boot_core::*;

// Re-export Web-specific types
pub use controller::*;
pub use macros::*;
pub use response::*;

// Re-export axum types for convenience
pub use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::{IntoResponse, Response},
    routing::{get, post, put, delete, patch},
    Json, Router,
};

// Re-export tower types
pub use tower::ServiceBuilder;
pub use tower_http::cors::CorsLayer;