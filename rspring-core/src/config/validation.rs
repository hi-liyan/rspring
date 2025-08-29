//! 配置验证模块
//! 
//! 提供配置数据的验证功能，确保配置的正确性和完整性

use crate::error::{Error, Result};
use std::collections::HashSet;
use url::Url;

/// 配置验证器
/// 
/// 提供各种配置项的验证逻辑
#[derive(Debug, Default)]
pub struct ConfigValidator;

impl ConfigValidator {
    /// 创建新的配置验证器
    pub fn new() -> Self {
        Self
    }
    
    /// 验证服务器端口
    /// 
    /// # 参数
    /// * `port` - 端口号
    /// 
    /// # 验证规则
    /// - 端口号必须在 1-65535 范围内
    /// - 避免使用系统保留端口（1-1024）
    pub fn validate_port(&self, port: u16) -> Result<()> {
        if port == 0 {
            return Err(Error::validation("端口号不能为 0"));
        }
        
        if port < 1024 {
            tracing::warn!("使用系统保留端口 {}，可能需要管理员权限", port);
        }
        
        Ok(())
    }
    
    /// 验证主机地址
    /// 
    /// # 参数
    /// * `host` - 主机地址
    /// 
    /// # 验证规则
    /// - 不能为空字符串
    /// - 支持 IPv4、IPv6 和域名格式
    pub fn validate_host(&self, host: &str) -> Result<()> {
        if host.is_empty() {
            return Err(Error::validation("主机地址不能为空"));
        }
        
        // 检查是否为有效的 IPv4 地址
        if let Ok(_) = host.parse::<std::net::Ipv4Addr>() {
            return Ok(());
        }
        
        // 检查是否为有效的 IPv6 地址
        if let Ok(_) = host.parse::<std::net::Ipv6Addr>() {
            return Ok(());
        }
        
        // 检查域名格式（简单验证）
        if host.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-') {
            return Ok(());
        }
        
        Err(Error::validation(format!("无效的主机地址: {}", host)))
    }
    
    /// 验证数据库连接 URL
    /// 
    /// # 参数
    /// * `url` - 数据库连接 URL
    /// 
    /// # 验证规则
    /// - URL 格式必须正确
    /// - 支持的协议：mysql, postgresql, sqlite
    pub fn validate_database_url(&self, url: &str) -> Result<()> {
        let parsed_url = Url::parse(url)
            .map_err(|_| Error::validation(format!("无效的数据库 URL: {}", url)))?;
        
        let valid_schemes = ["mysql", "postgresql", "sqlite", "postgres"];
        if !valid_schemes.contains(&parsed_url.scheme()) {
            return Err(Error::validation(format!(
                "不支持的数据库类型: {}，支持的类型: {}",
                parsed_url.scheme(),
                valid_schemes.join(", ")
            )));
        }
        
        Ok(())
    }
    
    /// 验证 Redis 连接 URL
    /// 
    /// # 参数
    /// * `url` - Redis 连接 URL
    pub fn validate_redis_url(&self, url: &str) -> Result<()> {
        let parsed_url = Url::parse(url)
            .map_err(|_| Error::validation(format!("无效的 Redis URL: {}", url)))?;
        
        if parsed_url.scheme() != "redis" && parsed_url.scheme() != "rediss" {
            return Err(Error::validation(format!(
                "不支持的 Redis 协议: {}，支持的协议: redis, rediss",
                parsed_url.scheme()
            )));
        }
        
        Ok(())
    }
    
    /// 验证连接池配置
    /// 
    /// # 参数
    /// * `min_connections` - 最小连接数
    /// * `max_connections` - 最大连接数
    pub fn validate_connection_pool(&self, min_connections: u32, max_connections: u32) -> Result<()> {
        if min_connections == 0 {
            return Err(Error::validation("最小连接数不能为 0"));
        }
        
        if max_connections == 0 {
            return Err(Error::validation("最大连接数不能为 0"));
        }
        
        if min_connections > max_connections {
            return Err(Error::validation("最小连接数不能大于最大连接数"));
        }
        
        if max_connections > 1000 {
            tracing::warn!("最大连接数 {} 可能过大，建议不超过 1000", max_connections);
        }
        
        Ok(())
    }
    
    /// 验证日志级别
    /// 
    /// # 参数
    /// * `level` - 日志级别字符串
    pub fn validate_log_level(&self, level: &str) -> Result<()> {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        
        if !valid_levels.contains(&level.to_lowercase().as_str()) {
            return Err(Error::validation(format!(
                "无效的日志级别: {}，支持的级别: {}",
                level,
                valid_levels.join(", ")
            )));
        }
        
        Ok(())
    }
    
    /// 验证邮件地址格式
    /// 
    /// # 参数
    /// * `email` - 邮件地址
    pub fn validate_email(&self, email: &str) -> Result<()> {
        if email.is_empty() {
            return Err(Error::validation("邮件地址不能为空"));
        }
        
        if !email.contains('@') {
            return Err(Error::validation("邮件地址格式无效"));
        }
        
        let parts: Vec<&str> = email.splitn(2, '@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(Error::validation("邮件地址格式无效"));
        }
        
        // 简单的域名验证
        let domain = parts[1];
        if !domain.contains('.') {
            return Err(Error::validation("邮件域名格式无效"));
        }
        
        Ok(())
    }
    
    /// 验证文件路径
    /// 
    /// # 参数
    /// * `path` - 文件路径
    /// * `must_exist` - 文件是否必须存在
    pub fn validate_file_path(&self, path: &str, must_exist: bool) -> Result<()> {
        if path.is_empty() {
            return Err(Error::validation("文件路径不能为空"));
        }
        
        let path = std::path::Path::new(path);
        
        if must_exist && !path.exists() {
            return Err(Error::validation(format!("文件不存在: {}", path.display())));
        }
        
        // 检查父目录是否存在（如果文件本身不需要存在）
        if !must_exist {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    return Err(Error::validation(format!(
                        "目录不存在: {}",
                        parent.display()
                    )));
                }
            }
        }
        
        Ok(())
    }
    
    /// 验证必需的配置项
    /// 
    /// # 参数
    /// * `required_keys` - 必需的配置键列表
    /// * `available_keys` - 可用的配置键列表
    pub fn validate_required_keys(&self, required_keys: &[&str], available_keys: &HashSet<String>) -> Result<()> {
        let missing_keys: Vec<&str> = required_keys
            .iter()
            .filter(|&&key| !available_keys.contains(key))
            .copied()
            .collect();
        
        if !missing_keys.is_empty() {
            return Err(Error::validation(format!(
                "缺少必需的配置项: {}",
                missing_keys.join(", ")
            )));
        }
        
        Ok(())
    }
    
    /// 验证数值范围
    /// 
    /// # 参数
    /// * `value` - 要验证的数值
    /// * `min` - 最小值（包含）
    /// * `max` - 最大值（包含）
    /// * `name` - 配置项名称
    pub fn validate_range<T>(&self, value: T, min: T, max: T, name: &str) -> Result<()>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min || value > max {
            return Err(Error::validation(format!(
                "{} 的值 {} 超出范围 [{}, {}]",
                name, value, min, max
            )));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_port() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_port(8080).is_ok());
        assert!(validator.validate_port(80).is_ok()); // 会有警告但不会失败
        assert!(validator.validate_port(65535).is_ok());
        
        assert!(validator.validate_port(0).is_err());
    }

    #[test]
    fn test_validate_host() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_host("localhost").is_ok());
        assert!(validator.validate_host("127.0.0.1").is_ok());
        assert!(validator.validate_host("0.0.0.0").is_ok());
        assert!(validator.validate_host("::1").is_ok());
        assert!(validator.validate_host("example.com").is_ok());
        
        assert!(validator.validate_host("").is_err());
        assert!(validator.validate_host("invalid@host").is_err());
    }

    #[test]
    fn test_validate_database_url() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_database_url("mysql://user:pass@localhost:3306/db").is_ok());
        assert!(validator.validate_database_url("postgresql://user:pass@localhost:5432/db").is_ok());
        assert!(validator.validate_database_url("sqlite:./app.db").is_ok());
        
        assert!(validator.validate_database_url("invalid://url").is_err());
        assert!(validator.validate_database_url("http://example.com").is_err());
        assert!(validator.validate_database_url("not-a-url").is_err());
    }

    #[test]
    fn test_validate_redis_url() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_redis_url("redis://localhost:6379").is_ok());
        assert!(validator.validate_redis_url("rediss://localhost:6380").is_ok());
        
        assert!(validator.validate_redis_url("mysql://localhost").is_err());
        assert!(validator.validate_redis_url("invalid-url").is_err());
    }

    #[test]
    fn test_validate_connection_pool() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_connection_pool(5, 10).is_ok());
        assert!(validator.validate_connection_pool(1, 1).is_ok());
        
        assert!(validator.validate_connection_pool(0, 10).is_err());
        assert!(validator.validate_connection_pool(5, 0).is_err());
        assert!(validator.validate_connection_pool(10, 5).is_err());
    }

    #[test]
    fn test_validate_log_level() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_log_level("info").is_ok());
        assert!(validator.validate_log_level("debug").is_ok());
        assert!(validator.validate_log_level("ERROR").is_ok()); // 大小写不敏感
        
        assert!(validator.validate_log_level("invalid").is_err());
        assert!(validator.validate_log_level("").is_err());
    }

    #[test]
    fn test_validate_email() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_email("user@example.com").is_ok());
        assert!(validator.validate_email("test.email@domain.co.uk").is_ok());
        
        assert!(validator.validate_email("").is_err());
        assert!(validator.validate_email("invalid-email").is_err());
        assert!(validator.validate_email("@example.com").is_err());
        assert!(validator.validate_email("user@").is_err());
        assert!(validator.validate_email("user@domain").is_err());
    }

    #[test]
    fn test_validate_range() {
        let validator = ConfigValidator::new();
        
        assert!(validator.validate_range(5, 1, 10, "test").is_ok());
        assert!(validator.validate_range(1, 1, 10, "test").is_ok());
        assert!(validator.validate_range(10, 1, 10, "test").is_ok());
        
        assert!(validator.validate_range(0, 1, 10, "test").is_err());
        assert!(validator.validate_range(11, 1, 10, "test").is_err());
    }

    #[test]
    fn test_validate_required_keys() {
        let validator = ConfigValidator::new();
        let mut available_keys = HashSet::new();
        available_keys.insert("server.host".to_string());
        available_keys.insert("server.port".to_string());
        
        let required_keys = vec!["server.host", "server.port"];
        assert!(validator.validate_required_keys(&required_keys, &available_keys).is_ok());
        
        let required_keys = vec!["server.host", "server.port", "server.ssl"];
        assert!(validator.validate_required_keys(&required_keys, &available_keys).is_err());
    }
}