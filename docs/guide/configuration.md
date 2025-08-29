# 配置系统

AxumBoot 提供了强大而灵活的配置系统，支持多种配置源和格式，让你的应用可以轻松适应不同的运行环境。

## 🎯 核心特性

- **多格式支持**: TOML、YAML、JSON
- **多配置源**: 文件、环境变量、命令行参数
- **环境隔离**: 支持不同环境的配置文件
- **类型安全**: 强类型配置绑定
- **热重载**: 支持配置文件热重载（开发中）
- **验证机制**: 配置值验证和约束

## 📁 配置文件组织

### 1. 默认配置文件

```
src/
├── main.rs
├── application.toml          # 主配置文件
├── application-dev.toml      # 开发环境配置
├── application-prod.toml     # 生产环境配置
└── application-test.toml     # 测试环境配置
```

### 2. 配置加载顺序

配置系统按以下优先级加载配置（数字越大优先级越高）：

1. **默认配置** (最低优先级)
2. **通用配置文件** (`application.toml`)
3. **环境特定配置文件** (`application-{profile}.toml`)
4. **环境变量** (`AXUM_BOOT_*`)
5. **命令行参数** (最高优先级)

## 📝 配置文件格式

### 1. TOML 格式 (推荐)

```toml
# application.toml

# 应用基本信息
[app]
name = "My AxumBoot App"
version = "1.0.0"
description = "一个使用 AxumBoot 构建的应用"

# 服务器配置
[server]
host = "0.0.0.0"
port = 8080
max_connections = 1000
request_timeout = 30
shutdown_timeout = 5

# 数据库配置
[database]
url = "mysql://root:password@localhost:3306/myapp"
max_connections = 10
min_connections = 5
connection_timeout = 30
idle_timeout = 600
max_lifetime = 3600

# Redis 配置
[redis]
url = "redis://localhost:6379"
pool_size = 10
connection_timeout = 5
read_timeout = 5
write_timeout = 5

# 日志配置
[logging]
level = "info"
format = "json"              # json | text
output = "stdout"            # stdout | file
file_path = "logs/app.log"
max_size = "100MB"
max_files = 10
compress = true

# JWT 认证配置
[jwt]
secret = "your-secret-key"
expiration = 3600            # 秒
issuer = "axumboot-app"

# 邮件配置
[email]
enabled = true
smtp_host = "smtp.gmail.com"
smtp_port = 587
username = "your-email@gmail.com"
password = "your-app-password"
use_tls = true

# 文件上传配置
[upload]
max_size = "10MB"
allowed_types = ["image/jpeg", "image/png", "image/gif"]
upload_dir = "uploads"

# 缓存配置
[cache]
default_ttl = 300            # 默认过期时间（秒）
cleanup_interval = 60        # 清理间隔（秒）

# 监控配置
[monitoring]
metrics_enabled = true
health_check_enabled = true
prometheus_endpoint = "/metrics"

# 自定义业务配置
[business]
max_users_per_org = 100
trial_period_days = 30
feature_flags = ["feature_a", "feature_b"]
```

### 2. YAML 格式

```yaml
# application.yaml
app:
  name: "My AxumBoot App"
  version: "1.0.0"

server:
  host: "0.0.0.0"
  port: 8080
  max_connections: 1000

database:
  url: "mysql://root:password@localhost:3306/myapp"
  max_connections: 10
  min_connections: 5

logging:
  level: "info"
  format: "json"
```

### 3. JSON 格式

```json
{
  "app": {
    "name": "My AxumBoot App",
    "version": "1.0.0"
  },
  "server": {
    "host": "0.0.0.0",
    "port": 8080,
    "max_connections": 1000
  },
  "database": {
    "url": "mysql://root:password@localhost:3306/myapp",
    "max_connections": 10,
    "min_connections": 5
  }
}
```

## 🏷️ 配置类型定义

### 1. 基础配置结构

```rust
use serde::{Deserialize, Serialize};
use axum_boot_core::Configuration;

/// 应用配置
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "AxumBoot App".to_string(),
            version: "1.0.0".to_string(),
            description: None,
        }
    }
}

/// 服务器配置
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    
    #[serde(default = "default_request_timeout")]
    pub request_timeout: u64,
}

// 默认值函数
fn default_host() -> String { "127.0.0.1".to_string() }
fn default_port() -> u16 { 8080 }
fn default_max_connections() -> u32 { 1000 }
fn default_request_timeout() -> u64 { 30 }
```

### 2. 嵌套配置结构

```rust
/// 数据库配置
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
    
    // 可选的连接池配置
    pub pool: Option<PoolConfig>,
    
    // SSL 配置
    pub ssl: Option<SslConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PoolConfig {
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub test_on_borrow: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SslConfig {
    pub enabled: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub ca_path: Option<String>,
}
```

### 3. 枚举配置

```rust
/// 日志级别
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// 日志格式
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Json,
    Text,
    Pretty,
}

/// 日志配置
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct LoggingConfig {
    #[serde(default)]
    pub level: LogLevel,
    
    #[serde(default)]
    pub format: LogFormat,
    
    pub output: LogOutput,
    
    // 文件日志配置
    pub file: Option<FileLogConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogOutput {
    Stdout,
    File,
    Both,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileLogConfig {
    pub path: String,
    pub max_size: String,      // "100MB"
    pub max_files: u32,
    pub compress: bool,
}
```

## 🌍 环境配置

### 1. 环境变量支持

AxumBoot 支持通过环境变量覆盖配置：

```bash
# 设置环境变量（使用 AXUM_BOOT_ 前缀）
export AXUM_BOOT_SERVER_PORT=9090
export AXUM_BOOT_DATABASE_URL="mysql://user:pass@prod-db:3306/app"
export AXUM_BOOT_LOGGING_LEVEL=debug

# 运行应用
cargo run
```

### 2. 环境变量命名规则

- 前缀：`AXUM_BOOT_`
- 分隔符：`_`（下划线）
- 大小写：全大写
- 嵌套：用下划线分隔

配置路径 `server.port` → 环境变量 `AXUM_BOOT_SERVER_PORT`
配置路径 `database.pool.max_connections` → 环境变量 `AXUM_BOOT_DATABASE_POOL_MAX_CONNECTIONS`

### 3. Profile 支持

```bash
# 设置运行环境
export AXUM_BOOT_PROFILE=prod

# 或者通过命令行参数
cargo run -- --profile=prod
```

对应的配置文件加载顺序：
1. `application.toml`
2. `application-prod.toml`

## 🔧 配置使用

### 1. 在服务中注入配置

```rust
use axum_boot_core::*;

#[derive(Service)]
pub struct EmailService {
    config: EmailConfig,
}

impl EmailService {
    /// 构造函数注入配置
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }
    
    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("邮件发送已禁用");
            return Ok(());
        }
        
        // 使用配置发送邮件
        let smtp_client = SmtpClient::new(&self.config.smtp_host, self.config.smtp_port);
        // ... 发送逻辑
        Ok(())
    }
}
```

### 2. 手动获取配置

```rust
use axum_boot_core::*;

#[derive(Service)]
pub struct ConfigService {
    config_manager: Arc<ConfigurationManager>,
}

impl ConfigService {
    pub async fn get_database_url(&self) -> Result<String> {
        self.config_manager.get_string("database.url")
    }
    
    pub async fn get_server_config(&self) -> Result<ServerConfig> {
        self.config_manager.bind::<ServerConfig>()
    }
    
    pub async fn get_feature_flag(&self, flag_name: &str) -> bool {
        self.config_manager
            .get::<Vec<String>>("business.feature_flags")
            .unwrap_or_default()
            .contains(&flag_name.to_string())
    }
}
```

### 3. 条件配置

```rust
/// 只有当邮件功能启用时才注册邮件服务
#[derive(Service)]
#[ConditionalOnProperty(name = "email.enabled", value = "true")]
pub struct EmailService {
    config: EmailConfig,
}

/// 当没有 Redis 配置时使用内存缓存
#[derive(Service)]
#[ConditionalOnMissingProperty("redis.url")]
pub struct InMemoryCacheService;

/// 根据配置值选择不同的实现
#[derive(Service)]
#[ConditionalOnProperty(name = "cache.type", value = "redis")]
pub struct RedisCacheService;

#[derive(Service)]
#[ConditionalOnProperty(name = "cache.type", value = "memory")]
pub struct MemoryCacheService;
```

## 🔍 配置验证

### 1. 基础验证

```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize, Validate, Configuration)]
pub struct ServerConfig {
    #[validate(length(min = 1))]
    pub host: String,
    
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
    
    #[validate(range(min = 1, max = 10000))]
    pub max_connections: u32,
    
    #[validate(url)]
    pub base_url: Option<String>,
}
```

### 2. 自定义验证

```rust
use validator::ValidationError;

fn validate_database_url(url: &str) -> Result<(), ValidationError> {
    if !url.starts_with("mysql://") && !url.starts_with("postgres://") {
        return Err(ValidationError::new("invalid_database_url"));
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Validate, Configuration)]
pub struct DatabaseConfig {
    #[validate(custom = "validate_database_url")]
    pub url: String,
    
    #[validate(range(min = 1, max = 100))]
    pub max_connections: u32,
}
```

### 3. 启动时验证

```rust
#[axum_boot_application]
pub struct Application;

impl Application {
    pub async fn run() -> Result<()> {
        // 在应用启动时验证所有配置
        let config_manager = ConfigurationManager::new()?;
        
        // 验证服务器配置
        let server_config: ServerConfig = config_manager.bind()?;
        server_config.validate()
            .map_err(|e| Error::validation(format!("服务器配置无效: {:?}", e)))?;
        
        // 验证数据库配置
        let database_config: DatabaseConfig = config_manager.bind()?;
        database_config.validate()
            .map_err(|e| Error::validation(format!("数据库配置无效: {:?}", e)))?;
        
        // 启动应用...
        Ok(())
    }
}
```

## 🔄 配置热重载

### 1. 启用热重载

```toml
# application.toml
[config]
hot_reload = true
watch_interval = 5  # 秒
```

### 2. 监听配置变更

```rust
use axum_boot_core::*;

#[derive(Service)]
pub struct ConfigWatcher {
    config_manager: Arc<ConfigurationManager>,
}

impl ConfigWatcher {
    pub async fn start_watching(&self) -> Result<()> {
        let mut watcher = self.config_manager.create_watcher().await?;
        
        while let Some(event) = watcher.next().await {
            match event {
                ConfigEvent::Changed { key, old_value, new_value } => {
                    tracing::info!("配置 {} 从 {:?} 变更为 {:?}", key, old_value, new_value);
                    
                    // 通知相关服务配置已变更
                    self.notify_config_change(&key, &new_value).await?;
                }
                ConfigEvent::Error(e) => {
                    tracing::error!("配置监听错误: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn notify_config_change(&self, key: &str, new_value: &str) -> Result<()> {
        // 发送配置变更事件
        // 让相关服务重新加载配置
        Ok(())
    }
}
```

## 🔒 敏感信息处理

### 1. 配置文件中的敏感信息

```toml
# application.toml
[database]
# 不要直接在配置文件中写密码
url = "mysql://user:${DB_PASSWORD}@localhost:3306/myapp"

[jwt]
# 使用环境变量
secret = "${JWT_SECRET}"

[email]
# 引用外部配置文件
password_file = "/etc/secrets/email_password"
```

### 2. 加密配置

```rust
use axum_boot_core::*;

#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct SecretConfig {
    #[serde(deserialize_with = "decrypt_value")]
    pub database_password: String,
    
    #[serde(deserialize_with = "decrypt_value")]
    pub api_key: String,
}

fn decrypt_value<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let encrypted_value: String = Deserialize::deserialize(deserializer)?;
    
    // 解密逻辑
    let decrypted = decrypt(&encrypted_value)
        .map_err(|e| serde::de::Error::custom(format!("解密失败: {}", e)))?;
    
    Ok(decrypted)
}
```

## 🚀 最佳实践

### 1. 配置组织

```rust
// 将相关配置组织在一起
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct AppConfiguration {
    pub app: AppConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: Option<RedisConfig>,
    pub email: Option<EmailConfig>,
    pub logging: LoggingConfig,
}

// 使用建造者模式创建配置
impl AppConfiguration {
    pub fn builder() -> AppConfigurationBuilder {
        AppConfigurationBuilder::new()
    }
}

pub struct AppConfigurationBuilder {
    config: AppConfiguration,
}

impl AppConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            config: AppConfiguration::default(),
        }
    }
    
    pub fn server(mut self, server: ServerConfig) -> Self {
        self.config.server = server;
        self
    }
    
    pub fn database(mut self, database: DatabaseConfig) -> Self {
        self.config.database = database;
        self
    }
    
    pub fn build(self) -> AppConfiguration {
        self.config
    }
}
```

### 2. 配置测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_server_config() {
        let config = ServerConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
    }
    
    #[test]
    fn test_config_validation() {
        let config = ServerConfig {
            host: "".to_string(),  // 无效的主机
            port: 0,               // 无效的端口
            ..Default::default()
        };
        
        assert!(config.validate().is_err());
    }
    
    #[tokio::test]
    async fn test_config_loading() {
        std::env::set_var("AXUM_BOOT_SERVER_PORT", "9090");
        
        let config_manager = ConfigurationManager::new().unwrap();
        let server_config: ServerConfig = config_manager.bind().unwrap();
        
        assert_eq!(server_config.port, 9090);
    }
}
```

### 3. 配置文档化

```rust
/// 应用配置
/// 
/// 支持的配置文件格式：TOML、YAML、JSON
/// 配置文件位置：application.{toml|yaml|json}
/// 
/// 环境变量前缀：AXUM_BOOT_
/// 
/// # 示例
/// 
/// ```toml
/// [app]
/// name = "My App"
/// version = "1.0.0"
/// 
/// [server]
/// host = "0.0.0.0"
/// port = 8080
/// ```
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct AppConfig {
    /// 应用名称
    /// 
    /// 环境变量：AXUM_BOOT_APP_NAME
    /// 默认值："AxumBoot App"
    pub name: String,
    
    /// 应用版本
    /// 
    /// 环境变量：AXUM_BOOT_APP_VERSION
    /// 默认值："1.0.0"
    pub version: String,
    
    /// 应用描述（可选）
    /// 
    /// 环境变量：AXUM_BOOT_APP_DESCRIPTION
    pub description: Option<String>,
}
```

## 🚀 下一步

现在你已经掌握了 AxumBoot 配置系统的强大功能，可以继续学习：

- 🔄 [依赖注入详解](dependency-injection.md) - 深入理解依赖注入机制
- 🌐 [Web 开发指南](web-development.md) - 学习 Web 开发的最佳实践
- 🗄️ [数据访问指南](data-access.md) - 掌握数据库操作技巧
- 🛡️ [错误处理指南](error-handling.md) - 构建健壮的错误处理机制