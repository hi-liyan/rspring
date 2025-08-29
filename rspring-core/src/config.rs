//! 配置管理模块
//! 
//! 提供统一的配置读取和管理功能，支持多种格式和验证机制

pub mod manager;
pub mod properties;
pub mod validation;

// 重新导出常用类型
pub use manager::ConfigurationManager;
pub use properties::*;
pub use validation::ConfigValidator;

// 为了向后兼容，保持原有的类型别名
pub type Configuration = dyn properties::Configuration;