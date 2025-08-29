# axum-boot-core 重新设计方案

## 🎯 模块重新定位

### 核心理念
axum-boot-core 应该是一个**纯粹的依赖注入和配置管理框架**，不包含任何 Web 特定的功能，可以独立用于各种类型的 Rust 应用。

## 📦 重新设计的模块结构

### axum-boot-core (纯核心)
```
axum-boot-core/
├── src/
│   ├── lib.rs                 # 只导出核心功能
│   ├── application.rs         # 应用上下文和生命周期
│   ├── container.rs          # 依赖注入容器
│   ├── config.rs            # 配置管理系统
│   ├── error.rs             # 通用错误类型（非HTTP）
│   ├── logging.rs           # 日志系统
│   └── component/           # 组件注解
│       ├── mod.rs
│       ├── component.rs     # @Component
│       ├── service.rs       # @Service  
│       └── repository.rs    # @Repository
```

**导出的 API**：
```rust
// lib.rs - 只导出纯核心功能
pub use application::*;
pub use container::*;
pub use config::*;
pub use error::{Error, Result};  // 通用错误，无HTTP状态码
pub use logging::*;

// 基础组件注解
pub use axum_boot_macro::{
    Component,
    Service,
    Repository,
    Configuration,
    axum_boot_application
};

// 重新导出常用类型
pub use serde::{Deserialize, Serialize};
```

### axum-boot-starter-web (Web专用)
```
axum-boot-starter-web/
├── src/
│   ├── lib.rs                 # 导出Web功能 + 重新导出core
│   ├── response.rs           # ApiResponse<T>
│   ├── pagination.rs         # Page<T>, PageResult<T>
│   ├── error/
│   │   ├── mod.rs
│   │   ├── web_error.rs      # HTTP状态码错误
│   │   └── handler.rs        # Web错误处理器
│   ├── controller/
│   ├── middleware/
│   └── server.rs
```

**导出的 API**：
```rust
// lib.rs - Web功能 + 重新导出core
pub use axum_boot_core::*;  // 重新导出所有核心功能

// Web 特定功能
pub use response::*;         // ApiResponse<T>
pub use pagination::*;       // Page<T>, PageResult<T>
pub use error::web::*;       // WebError, HTTP状态码
pub use controller::*;       // 控制器注解
pub use middleware::*;       // Web中间件

// Web 注解
pub use axum_boot_macro::{
    RestController,
    RequestMapping,
    GetMapping,
    PostMapping,
    // ... 其他Web注解
};
```

## 🚀 使用场景对比

### 场景1：CLI 应用（只用 core）
```rust
// Cargo.toml
[dependencies]
axum-boot-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }

// main.rs
use axum_boot_core::*;

#[derive(Service)]
pub struct DataProcessor {
    config: AppConfig,
}

#[axum_boot_application]
pub struct CliApplication;

#[tokio::main]
async fn main() -> Result<()> {
    // 纯粹的依赖注入和配置管理，无Web功能
    CliApplication::run().await
}
```

### 场景2：Web 应用（使用 starter-web）
```rust
// Cargo.toml
[dependencies]
axum-boot-starter-web = "0.1.0"  # 自动包含core
tokio = { version = "1.0", features = ["full"] }

// main.rs
use axum_boot_starter_web::*;  // 包含core + web功能

#[derive(RestController)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    #[GetMapping("/{id}")]
    pub async fn get_user(&self, id: u64) -> Result<ApiResponse<User>> {
        // ApiResponse 来自 starter-web
        Ok(ApiResponse::success(user))
    }
}

#[axum_boot_application]
pub struct WebApplication;

#[tokio::main] 
async fn main() -> Result<()> {
    WebApplication::run().await  // 自动启动Web服务器
}
```

### 场景3：后台任务服务（只用 core）
```rust
use axum_boot_core::*;

#[derive(Service)]
pub struct TaskScheduler {
    email_service: Arc<EmailService>,
    db_service: Arc<DatabaseService>,
}

#[axum_boot_application]
pub struct TaskApplication;

impl TaskApplication {
    // 自定义启动逻辑，无Web服务器
    pub async fn run() -> Result<()> {
        let context = ApplicationContext::new().await?;
        let scheduler = context.get::<TaskScheduler>()?;
        
        // 启动定时任务
        scheduler.start_scheduled_tasks().await?;
        
        // 保持运行
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}
```

## 🔧 错误类型重新设计

### axum-boot-core::Error (通用错误)
```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("配置错误: {0}")]
    Configuration(#[from] config::ConfigError),
    
    #[error("容器错误: {message}")]
    Container { message: String },
    
    #[error("组件未找到: {component}")]
    ComponentNotFound { component: String },
    
    #[error("业务错误: {message}")]
    Business { message: String },
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    // 注意：没有HTTP状态码
}
```

### axum-boot-starter-web::WebError (Web错误)
```rust
#[derive(Error, Debug)]  
pub enum WebError {
    #[error(transparent)]
    Core(#[from] axum_boot_core::Error),  // 包装核心错误
    
    #[error("HTTP错误 {status}: {message}")]
    Http { status: u16, message: String },
    
    #[error("验证失败: {0}")]
    Validation(String),
    
    #[error("认证失败")]
    Unauthorized,
    
    #[error("权限不足")]
    Forbidden,
}

impl WebError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            WebError::Core(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebError::Http { status, .. } => StatusCode::from_u16(*status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            WebError::Validation(_) => StatusCode::BAD_REQUEST,
            WebError::Unauthorized => StatusCode::UNAUTHORIZED,
            WebError::Forbidden => StatusCode::FORBIDDEN,
        }
    }
}
```

## 📋 迁移影响

### 现有代码需要调整
1. **ApiResponse 引用** - 从 `axum_boot_core` 改为 `axum_boot_starter_web`
2. **分页类型** - 从 `axum_boot_core` 改为 `axum_boot_starter_web`
3. **Web错误处理** - 使用新的 WebError 类型

### 向后兼容性
- axum-boot-starter-web 重新导出所有 core 功能
- 现有 Web 应用只需调整 import 即可

## 🎯 优势

### 1. **清晰的模块边界**
- core：纯依赖注入和配置管理
- starter-web：Web开发专用功能

### 2. **更好的复用性**
- core 可用于CLI、后台服务、数据处理等场景
- 不强制引入Web依赖

### 3. **更小的依赖**
- 非Web应用不需要引入 axum、tower 等Web相关依赖
- 更快的编译速度

### 4. **符合单一职责原则**
- 每个模块职责清晰
- 更容易维护和测试