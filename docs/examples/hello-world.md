# Hello World 示例

这是 AxumBoot 框架的入门示例，展示如何创建一个最简单的 Web 应用。

## 🎯 示例目标

通过这个示例，你将学会：

- 如何创建 AxumBoot 应用
- 如何配置应用
- 如何使用注解
- 如何处理 HTTP 请求
- 如何使用统一的响应格式

## 📁 项目结构

```
hello-world/
├── Cargo.toml
├── application.toml
└── src/
    └── main.rs
```

## 📝 代码实现

### Cargo.toml

```toml
[package]
name = "hello-world"
version = "0.1.0"
edition = "2021"

[dependencies]
# AxumBoot 核心依赖
axum-boot-core = "0.1.0"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 序列化
serde = { version = "1.0", features = ["derive"] }
```

### application.toml

```toml
# 服务器配置
[server]
host = "0.0.0.0"
port = 8080

# 日志配置
[logging]
level = "info"

# 应用配置
[app]
name = "Hello World App"
version = "1.0.0"
greeting = "Hello, AxumBoot!"
```

### src/main.rs

```rust
use axum_boot_core::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 问候响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct GreetingResponse {
    pub message: String,
    pub timestamp: i64,
    pub app_info: AppInfo,
}

/// 应用信息
#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

/// 自定义应用配置
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub greeting: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "AxumBoot App".to_string(),
            version: "1.0.0".to_string(),
            greeting: "Hello, World!".to_string(),
        }
    }
}

/// 问候服务
/// 
/// 负责生成问候消息的业务逻辑
#[derive(Service)]
pub struct GreetingService {
    config: AppConfig,
}

impl GreetingService {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }
    
    /// 生成通用问候消息
    pub fn get_greeting(&self) -> GreetingResponse {
        GreetingResponse {
            message: self.config.greeting.clone(),
            timestamp: chrono::Utc::now().timestamp(),
            app_info: AppInfo {
                name: self.config.name.clone(),
                version: self.config.version.clone(),
            },
        }
    }
    
    /// 生成个性化问候消息
    pub fn get_personal_greeting(&self, name: &str) -> GreetingResponse {
        GreetingResponse {
            message: format!("Hello, {}! Welcome to AxumBoot!", name),
            timestamp: chrono::Utc::now().timestamp(),
            app_info: AppInfo {
                name: self.config.name.clone(),
                version: self.config.version.clone(),
            },
        }
    }
    
    /// 获取应用信息
    pub fn get_app_info(&self) -> AppInfo {
        AppInfo {
            name: self.config.name.clone(),
            version: self.config.version.clone(),
        }
    }
}

/// Hello World 控制器
/// 
/// 提供基本的问候 API 端点
#[derive(RestController)]
pub struct HelloController {
    greeting_service: Arc<GreetingService>,
}

impl HelloController {
    pub fn new(greeting_service: Arc<GreetingService>) -> Self {
        Self { greeting_service }
    }
    
    /// 根路径问候
    /// 
    /// GET /
    pub async fn hello(&self) -> Result<ApiResponse<GreetingResponse>> {
        let greeting = self.greeting_service.get_greeting();
        Ok(ApiResponse::success(greeting))
    }
    
    /// 个性化问候
    /// 
    /// GET /hello/{name}
    pub async fn hello_name(&self, name: String) -> Result<ApiResponse<GreetingResponse>> {
        if name.trim().is_empty() {
            return Ok(ApiResponse::error(400, "姓名不能为空"));
        }
        
        let greeting = self.greeting_service.get_personal_greeting(&name);
        Ok(ApiResponse::success(greeting))
    }
    
    /// 获取应用信息
    /// 
    /// GET /info
    pub async fn app_info(&self) -> Result<ApiResponse<AppInfo>> {
        let info = self.greeting_service.get_app_info();
        Ok(ApiResponse::success(info))
    }
    
    /// 健康检查
    /// 
    /// GET /health
    pub async fn health_check(&self) -> Result<ApiResponse<&'static str>> {
        Ok(ApiResponse::success("OK"))
    }
}

/// 应用程序入口
/// 
/// 使用 #[axum_boot_application] 注解标记为 AxumBoot 应用
#[axum_boot_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    // 启动应用
    // 框架会自动：
    // 1. 加载配置文件
    // 2. 初始化日志系统
    // 3. 扫描和注册组件
    // 4. 启动 Web 服务器
    // 5. 注册路由
    Application::run().await
}
```

## 🚀 运行应用

### 1. 启动应用

```bash
# 进入项目目录
cd hello-world

# 运行应用
cargo run
```

你应该看到类似的输出：

```
INFO  Starting AxumBoot application...
INFO  Loading configuration from application.toml
INFO  Registering components...
INFO  Starting web server on 0.0.0.0:8080
INFO  AxumBoot application started successfully!
```

### 2. 测试 API

#### 基础问候

```bash
curl http://localhost:8080/

# 响应
{
  "code": 200,
  "message": "success",
  "data": {
    "message": "Hello, AxumBoot!",
    "timestamp": 1709875200,
    "app_info": {
      "name": "Hello World App",
      "version": "1.0.0"
    }
  },
  "timestamp": 1709875200
}
```

#### 个性化问候

```bash
curl http://localhost:8080/hello/张三

# 响应
{
  "code": 200,
  "message": "success",
  "data": {
    "message": "Hello, 张三! Welcome to AxumBoot!",
    "timestamp": 1709875200,
    "app_info": {
      "name": "Hello World App",
      "version": "1.0.0"
    }
  },
  "timestamp": 1709875200
}
```

#### 应用信息

```bash
curl http://localhost:8080/info

# 响应
{
  "code": 200,
  "message": "success",
  "data": {
    "name": "Hello World App",
    "version": "1.0.0"
  },
  "timestamp": 1709875200
}
```

#### 健康检查

```bash
curl http://localhost:8080/health

# 响应
{
  "code": 200,
  "message": "success",
  "data": "OK",
  "timestamp": 1709875200
}
```

#### 错误处理测试

```bash
# 测试空姓名
curl http://localhost:8080/hello/

# 响应
{
  "code": 400,
  "message": "姓名不能为空",
  "data": null,
  "timestamp": 1709875200
}
```

## 📚 代码解析

### 1. 应用入口

```rust
#[axum_boot_application]
pub struct Application;
```

- `#[axum_boot_application]` 宏将结构体标记为 AxumBoot 应用
- 自动生成 `run()` 方法，负责应用的完整启动流程

### 2. 配置管理

```rust
#[derive(Debug, Clone, Deserialize, Configuration)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub greeting: String,
}
```

- 实现了 `Configuration` trait 的结构体会被自动绑定配置
- 配置值来自 `application.toml` 文件中的 `[app]` 部分

### 3. 服务组件

```rust
#[derive(Service)]
pub struct GreetingService {
    config: AppConfig,
}
```

- `#[Service]` 标记该结构体为服务组件
- 框架会自动注入 `AppConfig` 依赖
- 包含业务逻辑，不直接处理 HTTP 请求

### 4. 控制器组件

```rust
#[derive(RestController)]
pub struct HelloController {
    greeting_service: Arc<GreetingService>,
}
```

- `#[RestController]` 标记该结构体为 REST 控制器
- 框架会自动注入 `GreetingService` 依赖
- 处理 HTTP 请求，调用服务层完成业务逻辑

### 5. 统一响应格式

```rust
Ok(ApiResponse::success(greeting))
```

- `ApiResponse<T>` 提供统一的 API 响应格式
- 包含状态码、消息、数据和时间戳
- 支持成功和错误响应

## 🔧 自定义配置

你可以通过修改 `application.toml` 来自定义应用行为：

```toml
[server]
host = "127.0.0.1"  # 只监听本地
port = 9090         # 更改端口

[app]
name = "我的应用"
version = "2.0.0"
greeting = "欢迎使用 AxumBoot!"

[logging]
level = "debug"     # 更详细的日志
```

## 🌍 环境变量

也可以通过环境变量覆盖配置：

```bash
# 设置环境变量
export AXUM_BOOT_SERVER_PORT=9090
export AXUM_BOOT_APP_GREETING="Hello from environment!"

# 运行应用
cargo run
```

## 🧪 扩展练习

### 1. 添加新的端点

在 `HelloController` 中添加一个时间端点：

```rust
/// 获取当前时间
/// 
/// GET /time
pub async fn current_time(&self) -> Result<ApiResponse<String>> {
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    Ok(ApiResponse::success(now))
}
```

### 2. 添加 POST 端点

添加一个接收数据的端点：

```rust
#[derive(Debug, Deserialize)]
pub struct CustomGreeting {
    pub name: String,
    pub message: String,
}

/// 自定义问候
/// 
/// POST /custom-greeting
pub async fn custom_greeting(
    &self, 
    request: CustomGreeting
) -> Result<ApiResponse<GreetingResponse>> {
    let response = GreetingResponse {
        message: format!("{}, {}!", request.message, request.name),
        timestamp: chrono::Utc::now().timestamp(),
        app_info: self.greeting_service.get_app_info(),
    };
    Ok(ApiResponse::success(response))
}
```

测试：

```bash
curl -X POST http://localhost:8080/custom-greeting \
  -H "Content-Type: application/json" \
  -d '{"name": "世界", "message": "你好"}'
```

### 3. 添加数据验证

使用 validator 库添加输入验证：

```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct CustomGreeting {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    
    #[validate(length(min = 1, max = 100))]
    pub message: String,
}

pub async fn custom_greeting(
    &self, 
    request: CustomGreeting
) -> Result<ApiResponse<GreetingResponse>> {
    // 验证输入
    if let Err(e) = request.validate() {
        return Ok(ApiResponse::error(400, format!("验证失败: {:?}", e)));
    }
    
    // 业务逻辑...
}
```

## 🎉 总结

这个 Hello World 示例展示了 AxumBoot 框架的核心特性：

- ✅ **简单启动** - 只需一个注解就能创建 Web 应用
- ✅ **配置管理** - 自动加载和绑定配置文件
- ✅ **依赖注入** - 自动装配组件依赖
- ✅ **统一响应** - 一致的 API 响应格式
- ✅ **注解驱动** - 声明式的组件定义
- ✅ **类型安全** - 编译时的类型检查

## 🚀 下一步

- 查看 [用户管理系统示例](user-management.md) - 更复杂的 CRUD 应用
- 查看 [核心概念指南](../guide/core-concepts.md) - 深入理解框架设计
- 查看 [配置系统指南](../guide/configuration.md) - 掌握配置管理技巧