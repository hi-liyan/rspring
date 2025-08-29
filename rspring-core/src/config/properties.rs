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

/// 数据库配置
/// 
/// 通用的数据库连接配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    /// 数据库连接 URL
    /// 
    /// 支持 MySQL、PostgreSQL、SQLite 等
    /// 
    /// # 示例
    /// - MySQL: `mysql://user:password@localhost:3306/database`
    /// - PostgreSQL: `postgresql://user:password@localhost:5432/database`
    /// - SQLite: `sqlite:./app.db`
    pub url: String,
    /// 最大连接数
    /// 
    /// # 默认值
    /// `10`
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    /// 最小连接数
    /// 
    /// # 默认值
    /// `5`
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    /// 连接超时时间（秒）
    /// 
    /// # 默认值
    /// `30`
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
    /// 连接池配置
    #[serde(default)]
    pub pool: Option<DatabasePool>,
}

/// 数据库连接池配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabasePool {
    /// 最小连接数
    pub min: u32,
    /// 最大连接数
    pub max: u32,
    /// 超时时间（秒）
    pub timeout: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "mysql://root:password@localhost:3306/rspring".to_string(),
            max_connections: default_max_connections(),
            min_connections: default_min_connections(),
            connection_timeout: default_connection_timeout(),
            pool: None,
        }
    }
}

impl Configuration for DatabaseConfig {}

/// Redis 配置
/// 
/// Redis 连接相关配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RedisConfig {
    /// Redis 连接 URL
    /// 
    /// # 示例
    /// `redis://localhost:6379/0`
    pub url: String,
    /// 连接池大小
    /// 
    /// # 默认值
    /// `10`
    #[serde(default = "default_redis_pool_size")]
    pub pool_size: u32,
    /// 连接超时时间（毫秒）
    /// 
    /// # 默认值
    /// `5000`
    #[serde(default = "default_redis_timeout")]
    pub timeout: u64,
    /// 是否启用集群模式
    /// 
    /// # 默认值
    /// `false`
    #[serde(default)]
    pub cluster_mode: bool,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379/0".to_string(),
            pool_size: default_redis_pool_size(),
            timeout: default_redis_timeout(),
            cluster_mode: false,
        }
    }
}

impl Configuration for RedisConfig {}

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

/// 邮件服务配置
/// 
/// SMTP 邮件发送配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MailConfig {
    /// SMTP 服务器地址
    pub smtp_host: String,
    /// SMTP 端口
    /// 
    /// # 默认值
    /// `587` (STARTTLS)
    #[serde(default = "default_smtp_port")]
    pub smtp_port: u16,
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
    /// 发件人邮箱
    pub from: String,
    /// 发件人姓名（可选）
    #[serde(default)]
    pub from_name: Option<String>,
    /// 是否启用 TLS
    /// 
    /// # 默认值
    /// `true`
    #[serde(default = "default_mail_tls")]
    pub tls: bool,
}

impl Configuration for MailConfig {}

// 默认值函数

fn default_workers() -> Option<usize> {
    Some(num_cpus::get())
}

fn default_max_connections() -> u32 {
    10
}

fn default_min_connections() -> u32 {
    5
}

fn default_connection_timeout() -> u64 {
    30
}

fn default_redis_pool_size() -> u32 {
    10
}

fn default_redis_timeout() -> u64 {
    5000
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

fn default_smtp_port() -> u16 {
    587
}

fn default_mail_tls() -> bool {
    true
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

    /// 测试数据库配置默认值
    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        assert!(config.url.contains("mysql://"));
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.min_connections, 5);
        assert_eq!(config.connection_timeout, 30);
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