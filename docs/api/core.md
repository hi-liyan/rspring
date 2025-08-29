# rspring-core API 参考

`rspring-core` 是 RSpring 框架的核心库，提供了应用启动、配置管理、依赖注入、错误处理等基础功能。

## 📦 模块概览

```rust
use rspring_core::{
    // 应用启动
    RSpringApplication, ApplicationContext,
    
    // 配置管理
    Configuration, ConfigurationManager,
    
    // 依赖注入
    Container, Component, Service, Repository, Controller,
    
    // 错误处理
    Error, Result,
    
    // 通用类型
    ApiResponse, Page, PageResult,
    
    // 宏
    rspring_application, Component, Service, Repository, RestController,
};
```

## 🚀 应用启动

### RSpringApplication

主应用类，负责应用的启动和生命周期管理。

```rust
pub struct RSpringApplication {
    context: ApplicationContext,
}

impl RSpringApplication {
    /// 创建新的应用实例
    pub fn new() -> Result<Self>;
    
    /// 启动应用
    /// 
    /// # 启动流程
    /// 1. 加载配置
    /// 2. 初始化日志系统
    /// 3. 创建应用上下文
    /// 4. 启动服务组件
    /// 5. 等待关闭信号
    /// 
    /// # 返回值
    /// - `Ok(())`: 应用正常关闭
    /// - `Err(Error)`: 启动或运行过程中出现错误
    pub async fn run(&self) -> Result<()>;
    
    /// 获取应用上下文
    pub fn context(&self) -> &ApplicationContext;
}
```

**使用示例：**

```rust
use rspring_core::*;

#[rspring_application]
pub struct MyApplication;

#[tokio::main]
async fn main() -> Result<()> {
    MyApplication::run().await
}
```

### ApplicationContext

应用上下文，管理整个应用的组件和配置。

```rust
pub struct ApplicationContext {
    pub container: Arc<RwLock<Container>>,
    pub config: Arc<ConfigurationManager>,
}

impl ApplicationContext {
    /// 创建新的应用上下文
    pub fn new() -> Result<Self>;
    
    /// 注册组件
    pub async fn register<T: 'static + Send + Sync>(&self, component: T);
    
    /// 注册单例组件
    pub async fn register_singleton<T: 'static + Send + Sync>(&self, component: T);
    
    /// 获取配置管理器
    pub fn config_manager(&self) -> &ConfigurationManager;
    
    /// 获取依赖注入容器
    pub fn container(&self) -> &Arc<RwLock<Container>>;
}
```

## ⚙️ 配置管理

### ConfigurationManager

配置管理器，负责加载和管理应用配置。

```rust
pub struct ConfigurationManager {
    config: Config,
}

impl ConfigurationManager {
    /// 创建配置管理器
    /// 
    /// 自动加载以下配置源（按优先级顺序）：
    /// 1. application.toml/yaml/json
    /// 2. application-{profile}.toml/yaml/json
    /// 3. 环境变量 (RSPRING_*)
    pub fn new() -> Result<Self>;
    
    /// 获取配置值
    /// 
    /// # 参数
    /// - `key`: 配置键，支持点号分隔的嵌套路径
    /// 
    /// # 示例
    /// ```rust
    /// let port: u16 = config.get("server.port")?;
    /// ```
    pub fn get<T>(&self, key: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned;
    
    /// 绑定配置到结构体
    /// 
    /// # 示例
    /// ```rust
    /// let server_config: ServerConfig = config.bind()?;
    /// ```
    pub fn bind<T>(&self) -> Result<T>
    where
        T: Configuration;
    
    /// 获取字符串配置
    pub fn get_string(&self, key: &str) -> Result<String>;
    
    /// 获取整数配置
    pub fn get_int(&self, key: &str) -> Result<i64>;
    
    /// 获取布尔配置
    pub fn get_bool(&self, key: &str) -> Result<bool>;
}
```

### Configuration Trait

配置结构体需要实现的标记 trait。

```rust
pub trait Configuration: serde::de::DeserializeOwned + Send + Sync + 'static {}
```

**内置配置结构体：**

```rust
/// 服务器配置
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

/// 数据库配置
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

/// Redis 配置
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

/// 日志配置
#[derive(Debug, Clone, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub pattern: Option<String>,
}
```

## 🔄 依赖注入

### Container

依赖注入容器，管理组件的生命周期。

```rust
pub struct Container {
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    singletons: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    /// 创建新的容器
    pub fn new() -> Self;
    
    /// 注册组件
    /// 
    /// # 参数
    /// - `component`: 要注册的组件实例
    /// 
    /// # 示例
    /// ```rust
    /// let mut container = Container::new();
    /// container.register(MyService::new());
    /// ```
    pub fn register<T: 'static + Send + Sync>(&mut self, component: T);
    
    /// 注册单例组件
    pub fn register_singleton<T: 'static + Send + Sync>(&mut self, component: T);
    
    /// 获取组件实例
    /// 
    /// # 返回值
    /// - `Some(&T)`: 组件实例的引用
    /// - `None`: 组件不存在
    pub fn get<T: 'static>(&self) -> Option<&T>;
    
    /// 获取单例组件
    /// 
    /// # 返回值
    /// - `Some(Arc<T>)`: 组件实例的智能指针
    /// - `None`: 组件不存在
    pub fn get_singleton<T: 'static>(&self) -> Option<Arc<T>>;
    
    /// 检查容器是否包含指定类型的组件
    pub fn contains<T: 'static>(&self) -> bool;
}
```

### 组件 Traits

```rust
/// 基础组件 trait
pub trait Component: Send + Sync {
    /// 获取组件名称
    fn component_name(&self) -> &'static str;
}

/// 服务组件标记 trait
pub trait Service: Component {}

/// 仓储组件标记 trait  
pub trait Repository: Component {}

/// 控制器组件标记 trait
pub trait Controller: Component {}
```

## ❌ 错误处理

### Error 枚举

统一的错误类型定义。

```rust
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// 配置错误
    #[error("配置错误: {0}")]
    Configuration(#[from] config::ConfigError),
    
    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    /// 验证错误
    #[error("验证错误: {message}")]
    Validation { message: String },
    
    /// 业务错误
    #[error("业务错误: {message}")]
    Business { message: String, code: String },
    
    /// 资源未找到
    #[error("资源未找到: {resource}")]
    NotFound { resource: String },
    
    /// 未授权访问
    #[error("未授权访问")]
    Unauthorized,
    
    /// 内部服务器错误
    #[error("内部服务器错误: {message}")]
    Internal { message: String },
}

impl Error {
    /// 创建验证错误
    pub fn validation(message: impl Into<String>) -> Self;
    
    /// 创建业务错误
    pub fn business(code: impl Into<String>, message: impl Into<String>) -> Self;
    
    /// 创建未找到错误
    pub fn not_found(resource: impl Into<String>) -> Self;
    
    /// 创建内部错误
    pub fn internal(message: impl Into<String>) -> Self;
}
```

### Result 类型

```rust
/// 框架统一的 Result 类型
pub type Result<T> = std::result::Result<T, Error>;
```

## 📊 通用类型

### ApiResponse

统一的 API 响应格式。

```rust
/// API 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 响应状态码
    pub code: i32,
    /// 响应消息
    pub message: String,
    /// 响应数据
    pub data: Option<T>,
    /// 时间戳
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    /// 
    /// # 参数
    /// - `data`: 响应数据
    /// 
    /// # 返回值
    /// 状态码为 200 的成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    /// 创建错误响应
    /// 
    /// # 参数
    /// - `code`: 错误状态码
    /// - `message`: 错误消息
    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}
```

**使用示例：**

```rust
// 成功响应
let response = ApiResponse::success(user);

// 错误响应
let response = ApiResponse::error(404, "用户不存在");
```

### 分页支持

```rust
/// 分页参数
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    /// 页码（从 0 开始）
    pub page: u64,
    /// 页大小
    pub size: u64,
}

impl Default for Page {
    fn default() -> Self {
        Self { page: 0, size: 20 }
    }
}

/// 分页结果
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T> {
    /// 当前页数据
    pub content: Vec<T>,
    /// 当前页码
    pub page: u64,
    /// 页大小
    pub size: u64,
    /// 总记录数
    pub total: u64,
    /// 总页数
    pub total_pages: u64,
}

impl<T> PageResult<T> {
    /// 创建分页结果
    /// 
    /// # 参数
    /// - `content`: 当前页数据
    /// - `page`: 当前页码
    /// - `size`: 页大小
    /// - `total`: 总记录数
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
```

## 🏷️ 宏

### #[rspring_application]

标记应用入口的属性宏。

```rust
/// 标记结构体为 RSpring 应用
/// 
/// 自动生成 `run()` 方法，用于启动应用
/// 
/// # 示例
/// ```rust
/// #[rspring_application]
/// pub struct Application;
/// 
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     Application::run().await
/// }
/// ```
#[proc_macro_attribute]
pub fn rspring_application(_args: TokenStream, input: TokenStream) -> TokenStream;
```

### #[derive(Component)]

为结构体实现 Component trait 的派生宏。

```rust
/// 为结构体自动实现 Component trait
/// 
/// # 示例
/// ```rust
/// #[derive(Component)]
/// pub struct MyComponent {
///     name: String,
/// }
/// ```
#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream;
```

### #[derive(Service)]

为结构体实现 Service trait 的派生宏。

```rust
/// 为结构体自动实现 Service trait
/// 
/// 同时会实现 Component trait
/// 
/// # 示例
/// ```rust
/// #[derive(Service)]
/// pub struct UserService {
///     repository: Arc<dyn UserRepository>,
/// }
/// ```
#[proc_macro_derive(Service)]
pub fn service_derive(input: TokenStream) -> TokenStream;
```

### #[derive(Repository)]

为结构体实现 Repository trait 的派生宏。

```rust
/// 为结构体自动实现 Repository trait
/// 
/// 同时会实现 Component trait
/// 
/// # 示例
/// ```rust
/// #[derive(Repository)]
/// pub struct UserRepository {
///     db_pool: Arc<DbPool>,
/// }
/// ```
#[proc_macro_derive(Repository)]
pub fn repository_derive(input: TokenStream) -> TokenStream;
```

### #[derive(RestController)]

为结构体实现 Controller trait 的派生宏。

```rust
/// 为结构体自动实现 Controller trait
/// 
/// 同时会实现 Component trait
/// 
/// # 示例
/// ```rust
/// #[derive(RestController)]
/// pub struct UserController {
///     user_service: Arc<UserService>,
/// }
/// ```
#[proc_macro_derive(RestController)]
pub fn rest_controller_derive(input: TokenStream) -> TokenStream;
```

## 📝 使用示例

### 完整的应用示例

```rust
use rspring_core::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

// 数据模型
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

// 仓储层
#[derive(Repository)]
pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(&self, id: u64) -> Result<Option<User>> {
        // 模拟数据库查询
        Ok(Some(User {
            id,
            name: "测试用户".to_string(),
            email: "test@example.com".to_string(),
        }))
    }
    
    pub async fn save(&self, user: User) -> Result<User> {
        // 模拟保存用户
        Ok(user)
    }
}

// 服务层
#[derive(Service)]
pub struct UserService {
    repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }
    
    pub async fn get_user(&self, id: u64) -> Result<Option<User>> {
        self.repository.find_by_id(id).await
    }
    
    pub async fn create_user(&self, name: String, email: String) -> Result<User> {
        let user = User { id: 1, name, email };
        self.repository.save(user).await
    }
}

// 控制器层
#[derive(RestController)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
    
    pub async fn get_user(&self, id: u64) -> Result<ApiResponse<User>> {
        match self.user_service.get_user(id).await? {
            Some(user) => Ok(ApiResponse::success(user)),
            None => Ok(ApiResponse::error(404, "用户不存在")),
        }
    }
}

// 应用入口
#[rspring_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}
```

### 配置使用示例

```rust
use rspring_core::*;

// 自定义配置结构
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct MyAppConfig {
    pub name: String,
    pub version: String,
    pub debug: bool,
}

#[derive(Service)]
pub struct ConfigService {
    config: MyAppConfig,
}

impl ConfigService {
    pub fn new(config: MyAppConfig) -> Self {
        Self { config }
    }
    
    pub fn get_app_info(&self) -> String {
        format!("{} v{}", self.config.name, self.config.version)
    }
    
    pub fn is_debug(&self) -> bool {
        self.config.debug
    }
}
```

## 🚀 下一步

- 查看 [axum-boot-macro API](macro.md) - 宏系统详细文档
- 查看 [axum-boot-starter-web API](starter-web.md) - Web 启动器文档
- 查看 [使用指南](../guide/quick-start.md) - 快速开始教程