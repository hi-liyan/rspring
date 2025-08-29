# RSpring 文档中心

欢迎使用 RSpring - 一个类似 Spring Boot 的 Rust Web 框架！

## 📖 文档导航

### 🚀 快速开始
- [**快速开始指南**](guide/quick-start.md) - 5分钟快速上手
- [**安装指南**](guide/installation.md) - 详细安装步骤
- [**第一个应用**](guide/first-application.md) - 创建你的第一个 RSpring 应用

### 📚 开发指南
- [**核心概念**](guide/core-concepts.md) - 理解 RSpring 的核心理念
- [**配置系统**](guide/configuration.md) - 配置文件和环境变量
- [**依赖注入**](guide/dependency-injection.md) - 组件管理和自动装配
- [**Web 开发**](guide/web-development.md) - 控制器、路由和中间件
- [**数据访问**](guide/data-access.md) - 数据库操作和 ORM
- [**缓存系统**](guide/caching.md) - Redis 集成和缓存注解
- [**错误处理**](guide/error-handling.md) - 统一错误处理和异常管理
- [**日志系统**](guide/logging.md) - 日志配置和最佳实践

### 🎯 最佳实践
- [**项目结构**](guide/project-structure.md) - 推荐的项目组织方式
- [**性能优化**](guide/performance.md) - 性能调优指南
- [**安全性**](guide/security.md) - 安全开发最佳实践
- [**测试策略**](guide/testing.md) - 单元测试和集成测试

### 📚 模块文档

深入了解每个模块的架构、API 和最佳实践：

- [**rspring-core**](modules/rspring-core.md) - 核心框架模块
  - 应用上下文和生命周期管理
  - 通用配置系统
  - 依赖注入容器
  - 核心错误处理和日志

- [**rspring-starter-web**](modules/rspring-starter-web.md) - Web 开发 Starter
  - Axum 框架集成和简化
  - REST API 控制器注解
  - 统一响应格式和错误处理
  - 中间件和路由管理

- [**rspring-starter-data-sqlx**](modules/rspring-starter-data-sqlx.md) - SQLx 数据访问 Starter
  - SQLx 集成和连接池管理
  - 多数据库支持（Feature 选择）
  - 声明式事务管理
  - 数据源自动配置

- [**rspring-starter-data-redis**](modules/rspring-starter-data-redis.md) - Redis 数据 Starter
  - Redis 客户端集成
  - 简化的 Redis 操作 API
  - 多数据结构支持
  - 连接池和序列化管理

- [**rspring-starter-mail**](modules/rspring-starter-mail.md) - 邮件发送 Starter
  - 邮件客户端集成
  - 邮件模板系统
  - SMTP 配置和自动装配
  - 异步邮件发送和附件支持

### 🔧 API 参考
- [**rspring-core**](api/core.md) - 核心库 API
- [**rspring-macro**](api/macro.md) - 宏和注解 API
- [**rspring-web**](api/starter-web.md) - Web 启动器 API
- [**rspring-data-mysql**](api/starter-data-mysql.md) - MySQL 启动器 API
- [**rspring-data-redis**](api/starter-data-redis.md) - Redis 启动器 API

### 💡 示例项目
- [**Hello World**](examples/hello-world.md) - 基础示例
- [**用户管理系统**](examples/user-management.md) - 完整的 CRUD 应用
- [**博客系统**](examples/blog-system.md) - 带认证的博客应用
- [**电商系统**](examples/ecommerce.md) - 微服务架构示例
- [**API 网关**](examples/api-gateway.md) - 网关服务示例

### 🚀 扩展开发
- [**Starter 开发指南**](modules/starter-development-guide.md) - 自定义 Starter 开发
  - 自动装配实现原理
  - Starter 开发最佳实践
  - 条件装配和配置绑定
  - 测试和发布指南
- [中间件开发](guide/middleware-development.md) - 自定义中间件
- [插件系统](guide/plugin-system.md) - 插件架构和开发

### 🤝 贡献指南
- [**贡献指南**](contributing/contributing.md) - 如何为项目贡献代码
- [**开发环境**](contributing/development-setup.md) - 开发环境搭建
- [**代码规范**](contributing/coding-standards.md) - 代码风格和规范
- [**发布流程**](contributing/release-process.md) - 版本发布流程

### ❓ 帮助与支持
- [**FAQ**](guide/faq.md) - 常见问题解答
- [**故障排除**](guide/troubleshooting.md) - 问题诊断和解决
- [**迁移指南**](guide/migration.md) - 从其他框架迁移
- [**社区**](guide/community.md) - 社区资源和交流

## 🌟 特性概览

### 🔥 核心特性
- **约定大于配置** - 最小化配置，最大化生产力
- **注解驱动开发** - 熟悉的 Spring Boot 风格注解
- **依赖注入** - 自动装配和生命周期管理
- **模块化架构** - 可插拔的 Starter 系统

### ⚡ 高性能
- **零成本抽象** - 充分利用 Rust 的性能优势
- **异步优先** - 基于 Tokio 的高并发处理
- **内存安全** - Rust 的内存安全保证

### 🧩 丰富生态
- **Web 开发** - 基于 Axum 的高性能 Web 服务
- **数据访问** - 支持 MySQL、PostgreSQL、Redis 等
- **监控集成** - 内置指标收集和健康检查

## 🎯 设计理念

RSpring 旨在让 Java 开发者能够平滑过渡到 Rust 生态，提供：

1. **熟悉的开发体验** - 类似 Spring Boot 的注解和配置方式
2. **Rust 的性能优势** - 享受 Rust 的零成本抽象和内存安全
3. **现代化架构** - 异步优先、云原生友好
4. **完整的工具链** - 从开发到部署的全流程支持

## 📄 版本信息

- **当前版本**: 0.1.0
- **Rust 版本要求**: 1.70+
- **许可证**: MIT

## 🚀 快速链接

- [GitHub 仓库](https://github.com/hi-liyan/rspring)
- [crates.io](https://crates.io/crates/rspring-core)
- [在线文档](https://docs.rs/rspring-core)
- [示例代码](https://github.com/hi-liyan/rspring/tree/main/examples)

---

**准备开始了吗？** 👉 [快速开始指南](guide/quick-start.md)