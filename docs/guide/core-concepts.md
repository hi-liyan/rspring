# 核心概念

本指南将帮助你理解 AxumBoot 框架的核心概念和设计理念。

## 🎯 设计理念

### 1. 约定大于配置

AxumBoot 遵循"约定大于配置"的原则，提供合理的默认配置，减少样板代码：

```rust
// 只需要一个注解，就可以启动完整的 Web 应用
#[axum_boot_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await  // 自动配置服务器、路由、中间件等
}
```

### 2. 注解驱动开发

借鉴 Spring Boot 的成功经验，使用注解来声明组件和配置：

```rust
#[RestController]              // 声明为 REST 控制器
#[RequestMapping("/api/users")] // 定义路由前缀
pub struct UserController {
    user_service: Arc<UserService>,  // 自动注入依赖
}

#[Service]  // 声明为服务组件
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

#[Repository]  // 声明为数据访问组件
pub struct UserRepositoryImpl {
    db_pool: Arc<DbPool>,
}
```

### 3. 模块化架构

通过 Starter 机制实现功能的模块化组装：

```toml
[dependencies]
axum-boot-core = "0.1.0"           # 核心功能
axum-boot-starter-web = "0.1.0"    # Web 支持（可选）
axum-boot-starter-data-mysql = "0.1.0"  # MySQL 支持（可选）
axum-boot-starter-data-redis = "0.1.0"  # Redis 支持（可选）
```

## 🏗️ 核心组件

### 1. 应用上下文 (ApplicationContext)

应用上下文是 AxumBoot 的核心，负责管理整个应用的生命周期：

```rust
pub struct ApplicationContext {
    pub container: Arc<RwLock<Container>>,  // 依赖注入容器
    pub config: Arc<ConfigurationManager>,  // 配置管理器
}
```

**职责：**
- 管理组件的生命周期
- 提供依赖注入服务
- 处理配置管理
- 协调各个模块的初始化

### 2. 依赖注入容器 (Container)

提供类似 Spring IoC 的依赖注入功能：

```rust
pub struct Container {
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    singletons: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    // 注册组件
    pub fn register<T: 'static + Send + Sync>(&mut self, component: T);
    
    // 注册单例组件
    pub fn register_singleton<T: 'static + Send + Sync>(&mut self, component: T);
    
    // 获取组件实例
    pub fn get<T: 'static>(&self) -> Option<&T>;
    
    // 获取单例组件
    pub fn get_singleton<T: 'static>(&self) -> Option<Arc<T>>;
}
```

### 3. 配置管理 (ConfigurationManager)

支持多种配置源的统一配置管理：

```rust
pub struct ConfigurationManager {
    config: Config,
}
```

**配置优先级（从高到低）：**
1. 环境变量
2. 命令行参数
3. 特定环境配置文件（如 `application-prod.toml`）
4. 通用配置文件（`application.toml`）
5. 默认配置

### 4. 错误处理系统

统一的错误类型和处理机制：

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("配置错误: {0}")]
    Configuration(#[from] config::ConfigError),
    
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("业务错误: {message} (错误码: {code})")]
    Business { message: String, code: String },
    
    #[error("资源未找到: {resource}")]
    NotFound { resource: String },
}
```

## 🧩 组件类型

### 1. Component（组件）

最基础的组件类型，所有其他组件都继承自它：

```rust
pub trait Component: Send + Sync {
    fn component_name(&self) -> &'static str;
}

#[derive(Component)]
pub struct MyComponent {
    // 组件字段
}
```

### 2. Service（服务）

业务逻辑组件，通常包含业务处理逻辑：

```rust
#[derive(Service)]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    /// 创建用户的业务逻辑
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        // 1. 验证输入
        self.validate_user_request(&request)?;
        
        // 2. 检查用户是否已存在
        if self.user_repository.exists_by_email(&request.email).await? {
            return Err(Error::business("USER_EXISTS", "用户已存在"));
        }
        
        // 3. 创建用户
        let user = User::new(request);
        self.user_repository.save(user).await
    }
}
```

### 3. Repository（仓储）

数据访问组件，负责与数据源交互：

```rust
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>>;
    async fn save(&self, user: User) -> Result<User>;
    async fn exists_by_email(&self, email: &str) -> Result<bool>;
}

#[derive(Repository)]
pub struct UserRepositoryImpl {
    db_pool: Arc<DbPool>,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: u64) -> Result<Option<User>> {
        // 数据库查询逻辑
        sqlx::query_as!(
            User,
            "SELECT id, name, email, created_at FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(Error::Database)
    }
}
```

### 4. Controller（控制器）

处理 HTTP 请求的组件：

```rust
#[derive(RestController)]
#[RequestMapping("/api/users")]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    #[GetMapping("/{id}")]
    pub async fn get_user(
        &self, 
        #[PathVariable] id: u64
    ) -> Result<ApiResponse<User>> {
        let user = self.user_service.get_user(id).await?;
        Ok(ApiResponse::success(user))
    }
    
    #[PostMapping]
    pub async fn create_user(
        &self,
        #[RequestBody] request: CreateUserRequest,
    ) -> Result<ApiResponse<User>> {
        let user = self.user_service.create_user(request).await?;
        Ok(ApiResponse::success(user))
    }
}
```

## 📊 生命周期管理

### 1. 应用启动流程

```
1. 解析配置文件和环境变量
2. 初始化日志系统
3. 创建应用上下文
4. 扫描和注册组件
5. 执行依赖注入
6. 启动 Web 服务器
7. 注册关闭钩子
```

### 2. 组件生命周期

```rust
pub enum ComponentScope {
    Singleton,  // 单例：整个应用生命周期内只有一个实例
    Prototype,  // 原型：每次请求都创建新实例
    Request,    // 请求：每个HTTP请求一个实例
    Session,    // 会话：每个用户会话一个实例
}
```

## 🔄 依赖注入

### 1. 构造函数注入

```rust
#[derive(Service)]
pub struct OrderService {
    user_service: Arc<UserService>,
    product_service: Arc<ProductService>,
    payment_service: Arc<PaymentService>,
}

impl OrderService {
    // AxumBoot 会自动注入所需的依赖
    pub fn new(
        user_service: Arc<UserService>,
        product_service: Arc<ProductService>,
        payment_service: Arc<PaymentService>,
    ) -> Self {
        Self {
            user_service,
            product_service,
            payment_service,
        }
    }
}
```

### 2. 接口注入

```rust
// 定义接口
#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn send_notification(&self, message: &str) -> Result<()>;
}

// 实现类
#[derive(Service)]
pub struct EmailNotificationService;

#[async_trait]
impl NotificationService for EmailNotificationService {
    async fn send_notification(&self, message: &str) -> Result<()> {
        // 发送邮件通知
        Ok(())
    }
}

// 注入接口
#[derive(Service)]
pub struct UserService {
    notification_service: Arc<dyn NotificationService>,
}
```

### 3. 条件注入

```rust
#[derive(Service)]
#[ConditionalOnProperty(name = "feature.email.enabled", value = "true")]
pub struct EmailService;

#[derive(Service)]
#[ConditionalOnMissingBean(EmailService)]
pub struct MockEmailService;
```

## 📝 配置系统

### 1. 配置文件结构

```toml
# application.toml

# 服务器配置
[server]
host = "0.0.0.0"
port = 8080
max_connections = 1000

# 数据库配置
[database]
url = "mysql://user:password@localhost:3306/mydb"
max_connections = 10
min_connections = 5
connection_timeout = 30

# Redis 配置
[redis]
url = "redis://localhost:6379"
pool_size = 10

# 日志配置
[logging]
level = "info"
format = "json"
output = "stdout"

# 自定义配置
[app]
name = "My AxumBoot App"
version = "1.0.0"
debug = false
```

### 2. 配置绑定

```rust
#[derive(Deserialize, Configuration)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub debug: bool,
}

#[derive(Service)]
pub struct AppService {
    config: AppConfig,
}

impl AppService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    
    pub fn get_app_info(&self) -> String {
        format!("{} v{}", self.config.name, self.config.version)
    }
}
```

## 🎭 事件系统

### 1. 应用事件

```rust
pub enum ApplicationEvent {
    Starting,
    Started,
    Ready,
    Stopping,
    Stopped,
}

#[async_trait]
pub trait ApplicationEventListener: Send + Sync {
    async fn on_event(&self, event: &ApplicationEvent) -> Result<()>;
}
```

### 2. 自定义事件

```rust
#[derive(Debug)]
pub struct UserCreatedEvent {
    pub user_id: u64,
    pub email: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Component)]
pub struct UserEventListener;

#[async_trait]
impl EventListener<UserCreatedEvent> for UserEventListener {
    async fn handle(&self, event: &UserCreatedEvent) -> Result<()> {
        tracing::info!("用户创建事件: {}", event.user_id);
        // 发送欢迎邮件
        // 更新统计信息
        Ok(())
    }
}
```

## 🔍 最佳实践

### 1. 组件设计原则

- **单一职责**: 每个组件只负责一个特定的功能
- **依赖倒置**: 依赖抽象接口而不是具体实现
- **开闭原则**: 对扩展开放，对修改关闭

### 2. 错误处理策略

```rust
// 在 Service 层处理业务逻辑错误
impl UserService {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        // 验证输入
        if request.email.is_empty() {
            return Err(Error::validation("邮箱不能为空"));
        }
        
        // 检查业务规则
        if self.user_exists(&request.email).await? {
            return Err(Error::business("USER_EXISTS", "用户已存在"));
        }
        
        // 执行操作
        self.user_repository.save(User::from(request)).await
    }
}

// 在 Controller 层处理 HTTP 错误
impl UserController {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<ApiResponse<User>> {
        match self.user_service.create_user(request).await {
            Ok(user) => Ok(ApiResponse::success(user)),
            Err(Error::Business { code, message }) => {
                Ok(ApiResponse::error(400, format!("{}: {}", code, message)))
            },
            Err(e) => {
                tracing::error!("创建用户失败: {}", e);
                Ok(ApiResponse::error(500, "内部服务器错误"))
            }
        }
    }
}
```

### 3. 异步编程最佳实践

```rust
// 使用 Arc 共享状态
#[derive(Service)]
pub struct CacheService {
    cache: Arc<RwLock<HashMap<String, String>>>,
}

// 使用 async/await 处理异步操作
impl CacheService {
    pub async fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
    
    pub async fn set(&self, key: String, value: String) {
        let mut cache = self.cache.write().await;
        cache.insert(key, value);
    }
}

// 批量操作使用并发
impl UserService {
    pub async fn get_users_batch(&self, ids: Vec<u64>) -> Result<Vec<User>> {
        let futures = ids.into_iter()
            .map(|id| self.user_repository.find_by_id(id));
        
        let results = futures::future::join_all(futures).await;
        
        let mut users = Vec::new();
        for result in results {
            if let Ok(Some(user)) = result {
                users.push(user);
            }
        }
        
        Ok(users)
    }
}
```

## 🚀 下一步

现在你已经理解了 AxumBoot 的核心概念，可以继续学习：

- 📋 [配置系统详解](configuration.md) - 掌握配置的高级用法
- 🔄 [依赖注入详解](dependency-injection.md) - 深入理解依赖注入
- 🌐 [Web 开发指南](web-development.md) - 学习 Web 开发的最佳实践
- 🗄️ [数据访问指南](data-access.md) - 掌握数据库操作技巧