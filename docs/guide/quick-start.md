# 快速开始指南

本指南将帮助你在 5 分钟内创建第一个 AxumBoot 应用。

## 📋 前置条件

- Rust 1.70 或更高版本
- 基本的 Rust 语言知识
- (可选) Java Spring Boot 开发经验

## 🚀 创建新项目

### 1. 初始化项目

```bash
# 创建新的 Rust 项目
cargo new my-axum-boot-app
cd my-axum-boot-app
```

### 2. 添加依赖

编辑 `Cargo.toml` 文件：

```toml
[package]
name = "my-axum-boot-app"
version = "0.1.0"
edition = "2021"

[dependencies]
# AxumBoot 核心依赖
axum-boot-core = "0.1.0"
axum-boot-starter-web = "0.1.0"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 序列化支持
serde = { version = "1.0", features = ["derive"] }
```

### 3. 创建主应用类

在 `src/main.rs` 中：

```rust
use axum_boot_core::*;

/// 应用程序入口
/// 使用 #[axum_boot_application] 注解标记为 AxumBoot 应用
#[axum_boot_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    // 启动应用
    Application::run().await
}
```

### 4. 创建配置文件

创建 `application.toml`：

```toml
# 服务器配置
[server]
host = "0.0.0.0"
port = 8080

# 日志配置
[logging]
level = "info"
```

### 5. 运行应用

```bash
cargo run
```

你应该看到类似的输出：

```
INFO  Starting AxumBoot application...
INFO  Server will start on 0.0.0.0:8080
INFO  AxumBoot application started successfully!
```

## 🎯 添加第一个 Controller

让我们添加一个简单的 REST API：

### 1. 创建用户模型

在 `src/main.rs` 中添加：

```rust
use serde::{Deserialize, Serialize};

/// 用户数据模型
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}
```

### 2. 创建用户服务

```rust
use axum_boot_core::*;
use std::sync::Arc;

/// 用户服务
/// 使用 #[Service] 注解标记为服务组件
#[derive(Service)]
pub struct UserService;

impl UserService {
    /// 获取用户信息
    pub async fn get_user(&self, id: u64) -> Result<User> {
        // 模拟数据库查询
        Ok(User {
            id,
            name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
        })
    }
    
    /// 创建新用户
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        // 模拟创建用户
        Ok(User {
            id: 1001,
            name: request.name,
            email: request.email,
        })
    }
}
```

### 3. 创建用户控制器

```rust
/// 用户控制器
/// 使用 #[RestController] 注解标记为 REST 控制器
#[derive(RestController)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    /// 根据ID获取用户
    /// GET /api/users/{id}
    pub async fn get_user(&self, id: u64) -> Result<ApiResponse<User>> {
        let user = self.user_service.get_user(id).await?;
        Ok(ApiResponse::success(user))
    }
    
    /// 创建新用户
    /// POST /api/users
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<ApiResponse<User>> {
        let user = self.user_service.create_user(request).await?;
        Ok(ApiResponse::success(user))
    }
    
    /// 获取所有用户
    /// GET /api/users
    pub async fn list_users(&self) -> Result<ApiResponse<Vec<User>>> {
        // 模拟用户列表
        let users = vec![
            User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string() },
            User { id: 2, name: "李四".to_string(), email: "lisi@example.com".to_string() },
        ];
        Ok(ApiResponse::success(users))
    }
}
```

### 4. 完整的代码示例

`src/main.rs` 的完整内容：

```rust
use axum_boot_core::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 用户数据模型
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
}

/// 创建用户请求
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

/// 用户服务
#[derive(Service)]
pub struct UserService;

impl UserService {
    pub async fn get_user(&self, id: u64) -> Result<User> {
        Ok(User {
            id,
            name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
        })
    }
    
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        Ok(User {
            id: 1001,
            name: request.name,
            email: request.email,
        })
    }
}

/// 用户控制器
#[derive(RestController)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    pub async fn get_user(&self, id: u64) -> Result<ApiResponse<User>> {
        let user = self.user_service.get_user(id).await?;
        Ok(ApiResponse::success(user))
    }
    
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<ApiResponse<User>> {
        let user = self.user_service.create_user(request).await?;
        Ok(ApiResponse::success(user))
    }
    
    pub async fn list_users(&self) -> Result<ApiResponse<Vec<User>>> {
        let users = vec![
            User { id: 1, name: "张三".to_string(), email: "zhangsan@example.com".to_string() },
            User { id: 2, name: "李四".to_string(), email: "lisi@example.com".to_string() },
        ];
        Ok(ApiResponse::success(users))
    }
}

/// 应用程序入口
#[axum_boot_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}
```

## 🧪 测试 API

启动应用后，你可以使用以下命令测试 API：

### 获取用户列表

```bash
curl http://localhost:8080/api/users
```

响应：
```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": 1,
      "name": "张三",
      "email": "zhangsan@example.com"
    },
    {
      "id": 2,
      "name": "李四", 
      "email": "lisi@example.com"
    }
  ],
  "timestamp": 1709875200
}
```

### 获取单个用户

```bash
curl http://localhost:8080/api/users/1
```

### 创建新用户

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "王五",
    "email": "wangwu@example.com"
  }'
```

## 🎉 恭喜！

你已经成功创建了第一个 AxumBoot 应用！这个简单的应用演示了：

- ✅ **应用启动** - 使用 `#[axum_boot_application]` 注解
- ✅ **依赖注入** - 自动装配服务组件
- ✅ **REST API** - 使用 `#[RestController]` 创建控制器
- ✅ **配置管理** - 通过 `application.toml` 配置应用
- ✅ **统一响应** - 使用 `ApiResponse` 统一 API 响应格式

## 🚀 下一步

现在你已经掌握了基础知识，可以继续探索：

- 📚 [核心概念](core-concepts.md) - 深入理解 AxumBoot 的设计理念
- 🗄️ [数据访问](data-access.md) - 学习数据库操作和 ORM
- 🔧 [配置系统](configuration.md) - 掌握高级配置技巧
- 🛡️ [错误处理](error-handling.md) - 构建健壮的错误处理机制
- 🎯 [最佳实践](project-structure.md) - 学习项目组织的最佳实践

## 💡 小贴士

1. **热重载**: 使用 `cargo watch -x run` 实现代码热重载
2. **日志调试**: 设置 `RUST_LOG=debug` 查看详细日志
3. **性能分析**: 使用 `--release` 模式获得最佳性能
4. **IDE 支持**: 推荐使用 VS Code + rust-analyzer 扩展

有问题？查看 [FAQ](faq.md) 或访问我们的 [社区](community.md)！