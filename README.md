# AxumBoot

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Docs](https://img.shields.io/badge/docs-latest-blue.svg)](docs/README.md)
[![Examples](https://img.shields.io/badge/examples-available-green.svg)](examples/)

🚀 **AxumBoot** - A Spring Boot-like framework for Rust, built on top of Axum

让 Java 程序员可以平滑使用 Rust + Axum 技术栈，提供类似 SpringBoot 的开发体验：约定大于配置、自动装配、注解驱动。

## ✨ 特性

- 🔥 **约定大于配置** - 提供合理的默认配置，最小化样板代码
- 🧩 **模块化设计** - 可插拔的 starter 系统，按需引入功能
- 🏷️ **注解驱动** - 基于 Rust 宏的注解式编程体验
- 🔄 **依赖注入** - 自动装配和生命周期管理
- 🌐 **Web 优先** - 基于高性能的 Axum 框架
- 💾 **数据访问** - 集成 SQLx/SeaORM，支持多种数据库
- ⚡ **高性能** - 充分利用 Rust 的零成本抽象和内存安全

## 📖 文档

### 🚀 快速开始
- [**5分钟快速上手**](docs/guide/quick-start.md) - 创建你的第一个 AxumBoot 应用
- [**核心概念**](docs/guide/core-concepts.md) - 理解框架的设计理念
- [**配置系统**](docs/guide/configuration.md) - 掌握配置管理

### 📚 开发指南
- [**依赖注入**](docs/guide/dependency-injection.md) - 组件管理和自动装配
- [**Web 开发**](docs/guide/web-development.md) - 控制器、路由和中间件
- [**数据访问**](docs/guide/data-access.md) - 数据库操作和 ORM
- [**错误处理**](docs/guide/error-handling.md) - 统一错误处理机制
- [**常见问题**](docs/guide/faq.md) - FAQ 和故障排除

### 🔧 模块文档
- [**axum-boot-core**](docs/modules/axum-boot-core.md) - 核心框架模块
- [**axum-boot-macro**](docs/modules/axum-boot-macro.md) - 宏系统模块
- [**axum-boot-starter-web**](docs/modules/axum-boot-starter-web.md) - Web 开发启动器
- [**axum-boot-starter-data-mysql**](docs/modules/axum-boot-starter-data-mysql.md) - MySQL 数据访问启动器
- [**axum-boot-starter-data-redis**](docs/modules/axum-boot-starter-data-redis.md) - Redis 缓存启动器

### 💡 示例项目
- [**Hello World**](docs/examples/hello-world.md) - 基础入门示例
- [**更多示例**](examples/) - 完整的应用示例

### 🤝 贡献
- [**贡献指南**](docs/contributing/contributing.md) - 如何参与项目开发
- [**开发规范**](CLAUDE.md) - 代码风格和提交规范

> 📋 查看 [**完整文档目录**](docs/README.md) 获取更多资源

## 🏗️ 架构

```
axum-boot/
├── axum-boot-core/           # 核心框架
├── axum-boot-macro/          # 宏和注解系统  
├── axum-boot-starter-web/    # Web 启动器
├── axum-boot-starter-data-mysql/  # MySQL 启动器
├── axum-boot-starter-data-redis/  # Redis 启动器
└── examples/                 # 示例项目
```

## 🚀 快速开始

### 1. 添加依赖

```toml
[dependencies]
axum-boot-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### 2. 创建应用

```rust
use axum_boot_core::*;

#[axum_boot_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}
```

### 3. 配置文件 (application.toml)

```toml
[server]
host = "0.0.0.0"
port = 8080

[logging]
level = "info"
```

### 4. 运行应用

```bash
cargo run
```

## 📋 开发状态

### ✅ 已完成
- [x] 项目基础架构和 Workspace 配置
- [x] 基础宏系统框架
- [x] 核心库基础功能（配置、容器、错误处理、日志）
- [x] Hello World 示例
- [x] 完整文档系统

### 🔄 进行中  
- [ ] 完善宏系统（Web 注解、依赖注入注解）
- [ ] 优化依赖注入容器

### 📝 待开发
- [ ] Web 启动器（Axum 集成、控制器支持）
- [ ] MySQL 启动器（数据库连接、ORM 集成）
- [ ] Redis 启动器（缓存、Session 支持）
- [ ] 完整示例项目

## 🎯 设计目标

让 Java 程序员能够这样写 Rust：

```rust
#[RestController]
#[RequestMapping("/api/users")]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    #[GetMapping("/{id}")]
    pub async fn get_user(&self, #[PathVariable] id: i64) -> Result<ApiResponse<User>> {
        let user = self.user_service.get_user_by_id(id).await?;
        Ok(ApiResponse::success(user))
    }

    #[PostMapping]  
    pub async fn create_user(
        &self, 
        #[RequestBody] request: CreateUserRequest
    ) -> Result<ApiResponse<User>> {
        let user = self.user_service.create_user(request).await?;
        Ok(ApiResponse::success(user))
    }
}

#[Service]
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

#[Repository]
pub struct UserRepositoryImpl {
    db_pool: Arc<DbPool>,
}
```

## 🌟 为什么选择 AxumBoot？

### 对于 Java 开发者
- 🎯 **熟悉的开发模式** - 类似 Spring Boot 的注解和架构
- 📚 **平滑学习曲线** - 复用现有的架构知识
- 🔄 **渐进式迁移** - 逐步从 Java 迁移到 Rust

### 对于 Rust 开发者
- ⚡ **高性能 Web 框架** - 基于 Axum 的零成本抽象
- 🛡️ **内存安全** - Rust 的编译期安全保证
- 🔧 **现代工具链** - 完整的开发、测试、部署支持

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

查看 [贡献指南](docs/contributing/contributing.md) 了解详细信息。

## 📄 许可证

本项目采用 MIT OR Apache-2.0 双重许可证。

## 🔗 相关链接

- [GitHub 仓库](https://github.com/axumboot/axum-boot)
- [crates.io](https://crates.io/crates/axum-boot-core)
- [在线文档](https://docs.rs/axum-boot-core)
- [示例代码](examples/)

---

**Ready to boot up?** 🚀 [开始使用 AxumBoot](docs/guide/quick-start.md)