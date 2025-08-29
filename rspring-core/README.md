# rspring-core

[![Crates.io](https://img.shields.io/crates/v/rspring-core.svg)](https://crates.io/crates/rspring-core)
[![Documentation](https://docs.rs/rspring-core/badge.svg)](https://docs.rs/rspring-core)

RSpring 框架的核心模块，提供依赖注入、配置管理、应用生命周期管理等基础功能。

## 特性

- 🔄 **依赖注入** - 类似 Spring 的自动装配和组件管理
- 📋 **配置管理** - 支持多种配置源（TOML、YAML、JSON、环境变量）
- 🏷️ **注解驱动** - 基于宏的组件声明和配置
- 🚀 **应用生命周期** - 统一的应用启动和关闭管理
- ⚡ **高性能** - 零成本抽象，充分利用 Rust 性能优势

## 快速开始

```toml
[dependencies]
rspring-core = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

```rust
use rspring_core::*;

#[derive(Service)]
pub struct MyService;

#[rspring_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}
```

## 使用场景

- CLI 工具和命令行应用
- 后台服务和定时任务
- 数据处理管道
- 微服务内部组件
- 任何需要依赖注入和配置管理的 Rust 应用

## 文档

- [GitHub 仓库](https://github.com/hi-liyan/rspring)
- [完整文档](https://github.com/hi-liyan/rspring/tree/main/docs)
- [示例代码](https://github.com/hi-liyan/rspring/tree/main/examples)

## 许可证

MIT License - 查看 [LICENSE](../LICENSE) 文件了解详情。