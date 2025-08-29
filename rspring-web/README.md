# rspring-web

[![Crates.io](https://img.shields.io/crates/v/rspring-web.svg)](https://crates.io/crates/rspring-web)
[![Documentation](https://docs.rs/rspring-web/badge.svg)](https://docs.rs/rspring-web)

RSpring 框架的 Web 开发启动器，基于 Axum 提供 Spring Boot 风格的 Web 开发体验。

## 特性

- 🌐 **REST 控制器** - Spring Boot 风格的控制器注解
- 🛣️ **自动路由** - 基于注解的路由注册
- 📊 **统一响应** - 标准化的 API 响应格式
- 🔧 **中间件支持** - 灵活的中间件配置
- 📈 **分页支持** - 内置分页查询支持
- ⚡ **高性能** - 基于 Axum 的零成本 Web 框架

## 快速开始

```toml
[dependencies]
rspring-web = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

```rust
use rspring_web::*;
use std::sync::Arc;

#[derive(RestController)]
#[RequestMapping("/api/users")]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    #[GetMapping("/{id}")]
    pub async fn get_user(&self, #[PathVariable] id: u64) -> Result<ApiResponse<User>> {
        let user = self.user_service.get_user(id).await?;
        Ok(ApiResponse::success(user))
    }

    #[PostMapping]
    pub async fn create_user(&self, #[RequestBody] request: CreateUserRequest) -> Result<ApiResponse<User>> {
        let user = self.user_service.create_user(request).await?;
        Ok(ApiResponse::success(user))
    }
}

#[rspring_application]
pub struct Application;

#[tokio::main]
async fn main() -> Result<()> {
    Application::run().await
}
```

## 注解支持

- `#[RestController]` - 标记 REST 控制器
- `#[RequestMapping]` - 定义基础路由路径  
- `#[GetMapping]` / `#[PostMapping]` / `#[PutMapping]` / `#[DeleteMapping]` - HTTP 方法映射
- `#[PathVariable]` - 路径变量提取
- `#[RequestBody]` - 请求体绑定
- `#[RequestParam]` - 查询参数提取

## 文档

- [GitHub 仓库](https://github.com/hi-liyan/rspring)
- [完整文档](https://github.com/hi-liyan/rspring/tree/main/docs)
- [Web 开发指南](https://github.com/hi-liyan/rspring/tree/main/docs/modules/rspring-web.md)

## 许可证

MIT License - 查看 [LICENSE](../LICENSE) 文件了解详情。