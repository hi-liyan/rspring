//! 配置管理器模块
//! 
//! 提供统一的配置读取和管理功能，支持 TOML、YAML、JSON 多种格式
//! 以及环境变量覆盖机制

use crate::error::{Error, Result};
use config::{Config, ConfigError, Environment, File};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// 配置管理器
/// 
/// 通用配置读取工具，支持多种格式和环境变量覆盖
#[derive(Debug)]
pub struct ConfigurationManager {
    /// 内部配置对象
    config: Config,
    /// 配置文件路径列表
    config_paths: Vec<String>,
    /// 环境变量前缀
    env_prefix: String,
}

impl ConfigurationManager {
    /// 创建新的配置管理器
    /// 
    /// # 配置文件加载顺序
    /// 1. `application.{toml|yaml|json}` - 基础配置
    /// 2. `application-{profile}.{toml|yaml|json}` - 环境配置
    /// 3. 环境变量 (RSPRING_*)
    /// 
    /// # 错误
    /// 当配置加载失败时返回错误
    pub fn new() -> Result<Self> {
        Self::with_prefix("RSPRING")
    }
    
    /// 使用自定义环境变量前缀创建配置管理器
    /// 
    /// # 参数
    /// * `env_prefix` - 环境变量前缀，如 "MYAPP"
    /// 
    /// # 示例
    /// ```rust
    /// let config = ConfigurationManager::with_prefix("MYAPP")?;
    /// // 将读取 MYAPP_SERVER_PORT 等环境变量
    /// ```
    pub fn with_prefix(env_prefix: &str) -> Result<Self> {
        let profile = std::env::var("PROFILE")
            .unwrap_or_else(|_| "dev".to_string());
        
        let mut config_builder = Config::builder();
        let mut config_paths = Vec::new();
        
        // 尝试加载基础配置文件 (TOML, YAML, JSON)
        let base_names = ["application"];
        let profile_names = [format!("application-{}", profile)];
        let extensions = ["toml", "yaml", "yml", "json"];
        
        // 加载基础配置
        for name in &base_names {
            for ext in &extensions {
                let config_file = format!("{}.{}", name, ext);
                config_builder = config_builder.add_source(
                    File::with_name(&config_file).required(false)
                );
                config_paths.push(config_file);
            }
        }
        
        // 加载环境特定配置
        for name in &profile_names {
            for ext in &extensions {
                let config_file = format!("{}.{}", name, ext);
                config_builder = config_builder.add_source(
                    File::with_name(&config_file).required(false)
                );
                config_paths.push(config_file);
            }
        }
        
        // 添加环境变量覆盖
        config_builder = config_builder.add_source(
            Environment::with_prefix(env_prefix).separator("_")
        );
        
        let config = config_builder.build()
            .map_err(Error::Configuration)?;
        
        Ok(Self {
            config,
            config_paths,
            env_prefix: env_prefix.to_string(),
        })
    }
    
    /// 获取单个配置值
    /// 
    /// 支持所有 serde 反序列化类型，包括：
    /// - 基础类型：String, i32, u64, bool 等
    /// - 集合类型：Vec<T>, HashMap<K, V>
    /// - 可选类型：Option<T>
    /// - 自定义结构体
    /// 
    /// # 参数
    /// * `key` - 配置键，支持点分隔路径如 "server.port"
    /// 
    /// # 示例
    /// ```rust
    /// // 基础类型
    /// let port: u16 = config.get("server.port")?;
    /// let debug: bool = config.get("app.debug")?;
    /// 
    /// // 集合类型
    /// let features: Vec<String> = config.get("custom.features")?;
    /// 
    /// // 复杂结构
    /// let db_config: HashMap<String, i32> = config.get("database.connections")?;
    /// ```
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
        self.config.get(key)
            .map_err(Error::Configuration)
    }
    
    /// 获取配置章节
    /// 
    /// 将整个配置章节绑定到结构体
    /// 
    /// # 参数
    /// * `section` - 配置章节名称
    /// 
    /// # 示例
    /// ```rust
    /// #[derive(Deserialize)]
    /// pub struct ServerConfig {
    ///     pub host: String,
    ///     pub port: u16,
    /// }
    /// 
    /// let server: ServerConfig = config.get_section("server")?;
    /// ```
    pub fn get_section<T: DeserializeOwned>(&self, section: &str) -> Result<T> {
        self.config.get(section)
            .map_err(Error::Configuration)
    }
    
    /// 获取整个配置文件
    /// 
    /// 将整个配置文件绑定到结构体
    pub fn get_all<T: DeserializeOwned>(&self) -> Result<T> {
        self.config.try_deserialize()
            .map_err(Error::Configuration)
    }
    
    /// 检查配置项是否存在
    /// 
    /// # 参数
    /// * `key` - 配置键
    /// 
    /// # 返回值
    /// 如果配置项存在返回 true，否则返回 false
    pub fn contains_key(&self, key: &str) -> bool {
        self.config.get::<serde_json::Value>(key).is_ok()
    }
    
    /// 获取所有配置键
    /// 
    /// 返回配置中所有可用的键名列表
    pub fn keys(&self) -> Vec<String> {
        self.get_keys_from_value("", &self.config.cache())
    }
    
    /// 获取指定前缀的所有配置键
    /// 
    /// # 参数
    /// * `prefix` - 键前缀
    /// 
    /// # 示例
    /// ```rust
    /// // 获取所有 "database." 开头的配置
    /// let db_keys = config.keys_with_prefix("database");
    /// ```
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.keys()
            .into_iter()
            .filter(|key| key.starts_with(prefix))
            .collect()
    }
    
    /// 从配置值中递归提取键名
    fn get_keys_from_value(&self, prefix: &str, value: &serde_json::Value) -> Vec<String> {
        let mut keys = Vec::new();
        
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let full_key = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{}.{}", prefix, k)
                    };
                    
                    keys.push(full_key.clone());
                    
                    // 递归处理嵌套对象
                    if v.is_object() {
                        keys.extend(self.get_keys_from_value(&full_key, v));
                    }
                }
            }
            _ => {
                if !prefix.is_empty() {
                    keys.push(prefix.to_string());
                }
            }
        }
        
        keys
    }
    
    /// 获取配置文件路径列表
    pub fn config_paths(&self) -> &[String] {
        &self.config_paths
    }
    
    /// 获取环境变量前缀
    pub fn env_prefix(&self) -> &str {
        &self.env_prefix
    }
    
    /// 重新加载配置
    /// 
    /// 重新读取配置文件和环境变量
    pub fn reload(&mut self) -> Result<()> {
        *self = Self::with_prefix(&self.env_prefix.clone())?;
        Ok(())
    }
    
    // 向后兼容的方法
    
    /// 获取字符串配置值（向后兼容）
    pub fn get_string(&self, key: &str) -> Result<String> {
        self.get(key)
    }
    
    /// 获取整数配置值（向后兼容）
    pub fn get_int(&self, key: &str) -> Result<i64> {
        self.get(key)
    }
    
    /// 获取布尔配置值（向后兼容）
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get(key)
    }
    
    /// 绑定配置到结构体（向后兼容）
    pub fn bind<T: DeserializeOwned>(&self) -> Result<T> {
        self.get_all()
    }
}

impl Default for ConfigurationManager {
    fn default() -> Self {
        Self::new().expect("Failed to create configuration manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::fs;
    use tempfile::tempdir;
    
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestConfig {
        app: AppSection,
        server: ServerSection,
    }
    
    #[derive(Debug, Deserialize, PartialEq)]
    struct AppSection {
        name: String,
        version: String,
        debug: bool,
    }
    
    #[derive(Debug, Deserialize, PartialEq)]
    struct ServerSection {
        host: String,
        port: u16,
    }

    /// 测试从 TOML 文件读取配置
    #[test]
    fn test_load_toml_config() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("application.toml");
        
        let config_content = r#"
[app]
name = "Test App"
version = "1.0.0"
debug = true

[server]
host = "0.0.0.0"
port = 8080
"#;
        
        fs::write(&config_path, config_content).unwrap();
        
        // 切换到临时目录
        let _guard = std::env::set_current_dir(&dir).unwrap();
        
        let config = ConfigurationManager::new().unwrap();
        
        let app_name: String = config.get("app.name").unwrap();
        assert_eq!(app_name, "Test App");
        
        let port: u16 = config.get("server.port").unwrap();
        assert_eq!(port, 8080);
    }

    /// 测试配置章节绑定
    #[test]
    fn test_get_section() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("application.toml");
        
        let config_content = r#"
[server]
host = "localhost"
port = 3000
"#;
        
        fs::write(&config_path, config_content).unwrap();
        let _guard = std::env::set_current_dir(&dir).unwrap();
        
        let config = ConfigurationManager::new().unwrap();
        let server_config: ServerSection = config.get_section("server").unwrap();
        
        assert_eq!(server_config.host, "localhost");
        assert_eq!(server_config.port, 3000);
    }

    /// 测试环境变量覆盖
    #[test]
    fn test_environment_override() {
        // 设置环境变量
        std::env::set_var("RSPRING_SERVER_PORT", "9000");
        
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("application.toml");
        
        let config_content = r#"
[server]
host = "localhost"
port = 3000
"#;
        
        fs::write(&config_path, config_content).unwrap();
        let _guard = std::env::set_current_dir(&dir).unwrap();
        
        let config = ConfigurationManager::new().unwrap();
        let port: u16 = config.get("server.port").unwrap();
        
        // 环境变量应该覆盖配置文件
        assert_eq!(port, 9000);
        
        // 清理环境变量
        std::env::remove_var("RSPRING_SERVER_PORT");
    }

    /// 测试配置键获取
    #[test]
    fn test_keys() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("application.toml");
        
        let config_content = r#"
[app]
name = "Test"

[server]
host = "localhost"
port = 3000

[database]
url = "mysql://localhost"
"#;
        
        fs::write(&config_path, config_content).unwrap();
        let _guard = std::env::set_current_dir(&dir).unwrap();
        
        let config = ConfigurationManager::new().unwrap();
        let keys = config.keys();
        
        // 应该包含所有配置键
        assert!(keys.contains(&"app".to_string()));
        assert!(keys.contains(&"server".to_string()));
        assert!(keys.contains(&"database".to_string()));
    }

    /// 测试前缀键查询
    #[test]
    fn test_keys_with_prefix() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("application.toml");
        
        let config_content = r#"
[server]
host = "localhost"
port = 3000

[database]
url = "mysql://localhost"
pool_size = 10
"#;
        
        fs::write(&config_path, config_content).unwrap();
        let _guard = std::env::set_current_dir(&dir).unwrap();
        
        let config = ConfigurationManager::new().unwrap();
        let server_keys = config.keys_with_prefix("server");
        
        assert!(server_keys.iter().any(|k| k.starts_with("server")));
    }

    /// 测试配置项存在检查
    #[test]
    fn test_contains_key() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("application.toml");
        
        let config_content = r#"
[server]
port = 3000
"#;
        
        fs::write(&config_path, config_content).unwrap();
        let _guard = std::env::set_current_dir(&dir).unwrap();
        
        let config = ConfigurationManager::new().unwrap();
        
        assert!(config.contains_key("server.port"));
        assert!(!config.contains_key("server.host"));
        assert!(!config.contains_key("nonexistent"));
    }
}