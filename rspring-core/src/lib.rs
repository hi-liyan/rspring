//! RSpring Core Library
//! 
//! RSpring 框架的核心库，提供应用启动、配置管理、依赖注入、错误处理和日志系统等基础功能。
//! 
//! # 特性
//! - 应用生命周期管理
//! - 通用配置系统支持 TOML/YAML/JSON 
//! - 依赖注入容器
//! - 核心错误处理
//! - 日志集成
//! - 核心组件注解

pub mod application;
pub mod config;
pub mod container;
pub mod error;
pub mod logging;
pub mod macros;

// 重新导出常用类型和特征
pub use application::{RSpringApp, RSpringApplication, ApplicationContext, AxumBootApplication};
pub use config::{Configuration, ConfigurationManager, AppConfig, ServerConfig, LoggingConfig};
pub use container::{
    Container, Component, Service, Repository, Controller,
    DependencyInjector, ComponentRegistry
};
pub use error::{Error, Result};

// 重新导出宏
pub use macros::*;

// 重新导出常用外部类型
pub use serde::{Deserialize, Serialize};