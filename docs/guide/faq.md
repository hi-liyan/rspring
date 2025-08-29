# 常见问题解答 (FAQ)

本文档收集了 AxumBoot 框架使用过程中的常见问题和解答。

## 🚀 快速开始

### Q: 如何创建一个新的 AxumBoot 项目？

**A:** 使用以下步骤创建新项目：

```bash
# 1. 创建新的 Rust 项目
cargo new my-axum-boot-app
cd my-axum-boot-app

# 2. 添加依赖到 Cargo.toml
[dependencies]
axum-boot-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }

# 3. 创建 src/main.rs
use axum_boot_core::*;

#[axum_boot_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}

# 4. 运行项目
cargo run
```

### Q: AxumBoot 与原生 Axum 有什么区别？

**A:** AxumBoot 是基于 Axum 构建的更高级的框架：

| 特性 | Axum | AxumBoot |
|-----|------|----------|
| 学习曲线 | 需要熟悉 Rust 生态 | 类似 Spring Boot，Java 开发者友好 |
| 配置 | 手动配置所有组件 | 约定大于配置，自动装配 |
| 依赖注入 | 需要手动管理依赖 | 自动依赖注入 |
| 注解支持 | 无 | 提供类似 Spring 的注解 |
| 启动器 | 需要手动集成第三方库 | 提供开箱即用的 Starter |

### Q: AxumBoot 支持哪些 Rust 版本？

**A:** AxumBoot 要求 Rust 1.70 或更高版本。推荐使用最新的稳定版本。

```bash
# 检查 Rust 版本
rustc --version

# 更新到最新版本
rustup update
```

## ⚙️ 配置相关

### Q: 配置文件放在哪里？

**A:** AxumBoot 会按以下顺序查找配置文件：

```
项目根目录/
├── application.toml      # 主配置文件
├── application.yaml      # 或 YAML 格式
├── application.json      # 或 JSON 格式
├── application-dev.toml  # 开发环境配置
├── application-prod.toml # 生产环境配置
└── application-test.toml # 测试环境配置
```

### Q: 如何使用环境变量覆盖配置？

**A:** 使用 `AXUM_BOOT_` 前缀：

```bash
# 覆盖服务器端口
export AXUM_BOOT_SERVER_PORT=9090

# 覆盖数据库 URL
export AXUM_BOOT_DATABASE_URL="mysql://localhost:3306/prod"

# 嵌套配置使用下划线分隔
export AXUM_BOOT_DATABASE_POOL_MAX_CONNECTIONS=20
```

### Q: 如何切换运行环境？

**A:** 设置 `AXUM_BOOT_PROFILE` 环境变量：

```bash
# 开发环境
export AXUM_BOOT_PROFILE=dev

# 生产环境  
export AXUM_BOOT_PROFILE=prod

# 或通过命令行参数
cargo run -- --profile=prod
```

### Q: 配置值的优先级是什么？

**A:** 配置优先级从高到低：

1. **命令行参数** (最高)
2. **环境变量** (`AXUM_BOOT_*`)
3. **环境特定配置文件** (`application-{profile}.toml`)
4. **通用配置文件** (`application.toml`)
5. **默认配置** (最低)

## 🔄 依赖注入

### Q: 如何注册自定义组件？

**A:** 使用相应的注解：

```rust
// 服务组件
#[derive(Service)]
pub struct MyService {
    dependency: Arc<SomeDependency>,
}

// 仓储组件
#[derive(Repository)]
pub struct MyRepository {
    db_pool: Arc<DbPool>,
}

// 控制器组件
#[derive(RestController)]
pub struct MyController {
    service: Arc<MyService>,
}

// 自定义组件
#[derive(Component)]
pub struct MyCustomComponent;
```

### Q: 如何注入接口依赖？

**A:** 定义 trait 并注册实现：

```rust
// 定义接口
#[async_trait]
pub trait EmailService: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()>;
}

// 实现类
#[derive(Service)]
pub struct SmtpEmailService;

#[async_trait]
impl EmailService for SmtpEmailService {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<()> {
        // 实现邮件发送
        Ok(())
    }
}

// 注入使用
#[derive(Service)]
pub struct UserService {
    email_service: Arc<dyn EmailService>,
}
```

### Q: 组件的生命周期是怎样的？

**A:** AxumBoot 目前支持以下生命周期：

- **Singleton** (默认): 整个应用生命周期内只有一个实例
- **Prototype**: 每次注入都创建新实例 (计划中)

```rust
// 单例组件（默认）
#[derive(Service)]
pub struct SingletonService;

// 原型组件（计划中）
#[derive(Service)]
#[Scope(Prototype)]
pub struct PrototypeService;
```

## 🌐 Web 开发

### Q: 如何定义路由？

**A:** 使用控制器注解（开发中）：

```rust
#[derive(RestController)]
#[RequestMapping("/api/users")]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    #[GetMapping("/{id}")]
    pub async fn get_user(&self, #[PathVariable] id: u64) -> Result<ApiResponse<User>> {
        // 处理逻辑
    }
    
    #[PostMapping]
    pub async fn create_user(&self, #[RequestBody] request: CreateUserRequest) -> Result<ApiResponse<User>> {
        // 处理逻辑
    }
}
```

### Q: 如何处理请求参数？

**A:** 使用参数注解：

```rust
impl UserController {
    // 路径参数
    #[GetMapping("/users/{id}")]
    pub async fn get_user(&self, #[PathVariable] id: u64) -> Result<ApiResponse<User>> {
        // id 从路径中提取
    }
    
    // 查询参数
    #[GetMapping("/users")]
    pub async fn list_users(
        &self, 
        #[RequestParam] page: Option<u64>,
        #[RequestParam] size: Option<u64>
    ) -> Result<ApiResponse<Vec<User>>> {
        let page = page.unwrap_or(0);
        let size = size.unwrap_or(20);
        // 处理逻辑
    }
    
    // 请求体
    #[PostMapping("/users")]
    pub async fn create_user(&self, #[RequestBody] request: CreateUserRequest) -> Result<ApiResponse<User>> {
        // request 从请求体中反序列化
    }
}
```

### Q: 如何统一处理错误？

**A:** AxumBoot 提供统一的错误处理：

```rust
// 业务逻辑中抛出错误
impl UserService {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        if request.email.is_empty() {
            return Err(Error::validation("邮箱不能为空"));
        }
        
        if self.user_exists(&request.email).await? {
            return Err(Error::business("USER_EXISTS", "用户已存在"));
        }
        
        // 正常处理
        Ok(user)
    }
}

// 控制器中处理
impl UserController {
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<ApiResponse<User>> {
        match self.user_service.create_user(request).await {
            Ok(user) => Ok(ApiResponse::success(user)),
            Err(Error::Validation { message }) => {
                Ok(ApiResponse::error(400, message))
            },
            Err(Error::Business { code, message }) => {
                Ok(ApiResponse::error(409, format!("{}: {}", code, message)))
            },
            Err(e) => {
                tracing::error!("创建用户失败: {}", e);
                Ok(ApiResponse::error(500, "内部服务器错误"))
            }
        }
    }
}
```

## 🗄️ 数据访问

### Q: 如何连接数据库？

**A:** 使用数据库启动器（开发中）：

```toml
[dependencies]
axum-boot-starter-data-mysql = "0.1.0"

# 配置文件
[database]
url = "mysql://root:password@localhost:3306/mydb"
max_connections = 10
```

```rust
#[derive(Repository)]
pub struct UserRepository {
    db_pool: Arc<DbPool>,
}

impl UserRepository {
    pub async fn find_by_id(&self, id: u64) -> Result<Option<User>> {
        sqlx::query_as!(
            User,
            "SELECT id, name, email FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&*self.db_pool)
        .await
        .map_err(Error::Database)
    }
}
```

### Q: 如何使用事务？

**A:** 使用事务注解：

```rust
#[derive(Service)]
pub struct UserService {
    repository: Arc<UserRepository>,
}

impl UserService {
    #[Transactional]
    pub async fn create_user_with_profile(&self, request: CreateUserRequest) -> Result<User> {
        // 这个方法中的所有数据库操作都在同一个事务中
        let user = self.repository.save_user(request.into()).await?;
        let profile = UserProfile::new(user.id, request.profile);
        self.repository.save_profile(profile).await?;
        Ok(user)
    }
}
```

## 🔧 开发和调试

### Q: 如何启用详细日志？

**A:** 配置日志级别：

```toml
# application.toml
[logging]
level = "debug"  # trace | debug | info | warn | error

# 或使用环境变量
export RUST_LOG=debug
export AXUM_BOOT_LOGGING_LEVEL=debug
```

### Q: 如何进行热重载开发？

**A:** 使用 `cargo-watch`：

```bash
# 安装 cargo-watch
cargo install cargo-watch

# 启用热重载
cargo watch -x run

# 或者只重新编译
cargo watch -x check
```

### Q: 如何调试依赖注入问题？

**A:** 启用容器调试日志：

```toml
[logging]
level = "debug"

# 或者通过环境变量
export RUST_LOG="axum_boot_core::container=debug"
```

你会看到类似的日志：

```
DEBUG axum_boot_core::container: 注册组件: UserService
DEBUG axum_boot_core::container: 注册组件: UserRepository  
DEBUG axum_boot_core::container: 解析依赖: UserService -> UserRepository
DEBUG axum_boot_core::container: 依赖注入完成
```

## 🧪 测试

### Q: 如何编写单元测试？

**A:** 为服务和仓储编写测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_create_user() {
        // 创建模拟依赖
        let mock_repository = MockUserRepository::new();
        let service = UserService::new(Arc::new(mock_repository));
        
        // 准备测试数据
        let request = CreateUserRequest {
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        };
        
        // 执行测试
        let result = service.create_user(request).await;
        
        // 验证结果
        assert!(result.is_ok());
    }
}
```

### Q: 如何进行集成测试？

**A:** 创建测试应用实例：

```rust
#[tokio::test]
async fn test_user_api() {
    // 创建测试应用
    let app = create_test_app().await;
    
    // 测试 API
    let response = app
        .post("/api/users")
        .json(&json!({
            "name": "Test User",
            "email": "test@example.com"
        }))
        .send()
        .await;
    
    assert_eq!(response.status(), 200);
    
    let user: ApiResponse<User> = response.json().await;
    assert_eq!(user.data.unwrap().name, "Test User");
}

async fn create_test_app() -> TestApp {
    // 创建测试配置
    let config = TestConfig::default();
    
    // 启动测试应用
    TestApp::from_config(config).await
}
```

## 🚀 部署

### Q: 如何构建生产版本？

**A:** 使用 release 模式构建：

```bash
# 构建优化版本
cargo build --release

# 可执行文件位于
./target/release/your-app-name
```

### Q: 如何配置生产环境？

**A:** 创建生产配置文件：

```toml
# application-prod.toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "${DATABASE_URL}"  # 使用环境变量
max_connections = 50

[logging]
level = "warn"
format = "json"
```

```bash
# 设置环境变量
export AXUM_BOOT_PROFILE=prod
export DATABASE_URL="mysql://user:pass@prod-db:3306/app"

# 运行应用
./target/release/your-app
```

### Q: 如何使用 Docker 部署？

**A:** 创建 Dockerfile：

```dockerfile
# 多阶段构建
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/your-app ./
COPY application-prod.toml ./application.toml

EXPOSE 8080

CMD ["./your-app"]
```

```bash
# 构建镜像
docker build -t your-app:latest .

# 运行容器
docker run -p 8080:8080 -e AXUM_BOOT_PROFILE=prod your-app:latest
```

## 🐛 故障排除

### Q: 应用启动失败怎么办？

**A:** 检查以下几个方面：

1. **配置文件**：确保配置文件格式正确
```bash
# 验证 TOML 语法
cargo install toml-cli
toml get application.toml server.port
```

2. **依赖版本**：确保所有依赖版本兼容
```bash
cargo tree
```

3. **端口占用**：检查端口是否被占用
```bash
# Linux/macOS
lsof -i :8080

# Windows
netstat -an | findstr :8080
```

### Q: 依赖注入失败怎么办？

**A:** 检查组件注册：

1. 确保组件有正确的注解
2. 检查依赖的类型是否正确
3. 启用调试日志查看注册过程

```rust
// 错误：缺少注解
pub struct UserService;  // ❌

// 正确：有注解
#[derive(Service)]
pub struct UserService;  // ✅
```

### Q: 数据库连接失败怎么办？

**A:** 检查连接配置：

1. **URL 格式**：确保数据库 URL 格式正确
```toml
# MySQL
url = "mysql://username:password@host:port/database"

# PostgreSQL  
url = "postgres://username:password@host:port/database"
```

2. **网络连接**：测试数据库连接
```bash
# 测试 MySQL 连接
mysql -h localhost -P 3306 -u username -p

# 测试 PostgreSQL 连接
psql -h localhost -p 5432 -U username -d database
```

3. **权限检查**：确保数据库用户有足够权限

## 📚 更多帮助

### Q: 在哪里可以找到更多文档？

**A:** 查看以下资源：

- 📖 [官方文档](../README.md)
- 🎯 [快速开始](../guide/quick-start.md)
- 🔧 [API 参考](../api/core.md)
- 💡 [示例项目](../examples/hello-world.md)
- 🤝 [贡献指南](../contributing/contributing.md)

### Q: 如何获得社区支持？

**A:** 可以通过以下方式：

- 💬 [GitHub Discussions](https://github.com/axumboot/axum-boot/discussions) - 讨论和问答
- 🐛 [GitHub Issues](https://github.com/axumboot/axum-boot/issues) - 报告 bug 和功能请求
- 📧 邮件联系维护者团队
- 🌟 加入我们的社区群组

### Q: 如何报告 Bug？

**A:** 在 GitHub Issues 中提交，包含：

1. **环境信息**：Rust 版本、操作系统、AxumBoot 版本
2. **重现步骤**：详细的重现步骤
3. **期望行为**：你期望发生什么
4. **实际行为**：实际发生了什么
5. **相关日志**：错误日志和堆栈跟踪

---

如果你的问题没有在这里找到答案，请在 [Discussions](https://github.com/axumboot/axum-boot/discussions) 中提问！