pub mod application;
pub mod config;
pub mod container;
pub mod error;
pub mod logging;
pub mod macros;

// Re-export common types and traits
pub use application::{AxumBootApplication, ApplicationContext};
pub use config::{Configuration, ConfigurationManager};
pub use container::{Container, Component, Service, Repository, Controller};
pub use error::{Error, Result};

// Re-export macros
pub use macros::*;

// Re-export common external types
pub use serde::{Deserialize, Serialize};