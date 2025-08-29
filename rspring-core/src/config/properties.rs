//! 配置属性定义模块
//! 
//! 定义了常用的配置结构体，便于应用程序使用

use serde::{Deserialize, Serialize};

/// 配置特征
/// 
/// 标记接口，用于表示可以从配置文件中反序列化的类型
pub trait Configuration: for<'de> Deserialize<'de> + Send + Sync + 'static {}

/// 服务器配置
/// 
/// 包含服务器启动相关的配置项
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ServerConfig {
    /// 服务器绑定地址
    /// 
    /// # 默认值
    /// `"0.0.0.0"`
    pub host: String,
    /// 服务器监听端口
    /// 
    /// # 默认值
    /// `8080`
    pub port: u16,
    /// 工作线程数
    /// 
    /// # 默认值
    /// CPU 核心数
    #[serde(default = "default_workers")]
    pub workers: Option<usize>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: None,
        }
    }
}

impl Configuration for ServerConfig {}

/// 应用程序配置
/// 
/// 包含应用程序基本信息配置
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AppConfig {
    /// 应用程序名称
    pub name: String,
    /// 应用程序版本
    pub version: String,
    /// 是否启用调试模式
    /// 
    /// # 默认值
    /// `false`
    #[serde(default)]
    pub debug: bool,
    /// 应用程序描述
    #[serde(default)]
    pub description: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "RSpring Application".to_string(),
            version: "1.0.0".to_string(),
            debug: false,
            description: None,
        }
    }
}

impl Configuration for AppConfig {}


/// 日志配置
/// 
/// 应用程序日志系统配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    /// 日志级别
    /// 
    /// 支持的级别：trace, debug, info, warn, error
    /// 
    /// # 默认值
    /// `"info"`
    #[serde(default = "default_log_level")]
    pub level: String,
    /// 日志格式
    /// 
    /// 支持：json, pretty, compact
    /// 
    /// # 默认值
    /// `"pretty"`
    #[serde(default = "default_log_format")]
    pub format: String,
    /// 日志文件路径（可选）
    /// 
    /// 如果设置，日志将同时输出到控制台和文件
    #[serde(default)]
    pub file: Option<String>,
    /// 日志文件最大大小（MB）
    /// 
    /// # 默认值
    /// `100`
    #[serde(default = "default_log_file_size")]
    pub max_file_size: u64,
    /// 日志文件保留数量
    /// 
    /// # 默认值
    /// `7`
    #[serde(default = "default_log_file_count")]
    pub max_files: u32,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
            file: None,
            max_file_size: default_log_file_size(),
            max_files: default_log_file_count(),
        }
    }
}

impl Configuration for LoggingConfig {}


// 默认值函数

fn default_workers() -> Option<usize> {
    Some(num_cpus::get())
}


fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

fn default_log_file_size() -> u64 {
    100
}

fn default_log_file_count() -> u32 {
    7
}


#[cfg(test)]
mod tests {
    use super::*;

    /// 测试服务器配置默认值
    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.port, 8080);
        assert!(config.workers.is_none());
    }

    /// 测试应用配置默认值
    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.name, "RSpring Application");
        assert_eq!(config.version, "1.0.0");
        assert!(!config.debug);
        assert!(config.description.is_none());
    }


    /// 测试日志配置默认值
    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, "info");
        assert_eq!(config.format, "pretty");
        assert!(config.file.is_none());
        assert_eq!(config.max_file_size, 100);
        assert_eq!(config.max_files, 7);
    }

    /// 测试配置序列化和反序列化
    #[test]
    fn test_config_serialization() {
        let config = ServerConfig {
            host: "localhost".to_string(),
            port: 3000,
            workers: Some(4),
        };
        
        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(config, deserialized);
    }
}