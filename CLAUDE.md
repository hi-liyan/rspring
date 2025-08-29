# AxumBoot 项目开发规范

## 代码风格规范

### 1. 注释规范

#### 1.1 文档注释
所有公共 API 必须使用文档注释，格式如下：

```rust
/// 用户服务，提供用户相关的业务逻辑处理
/// 
/// # 功能
/// - 创建用户
/// - 查询用户信息
/// - 更新用户数据
/// - 删除用户
/// 
/// # 示例
/// ```rust
/// let user_service = UserService::new(user_repository);
/// let user = user_service.create_user(request).await?;
/// ```
pub struct UserService {
    user_repository: Arc<dyn UserRepository>,
}

impl UserService {
    /// 根据ID查询用户信息
    /// 
    /// # 参数
    /// * `id` - 用户ID
    /// 
    /// # 返回值
    /// * `Ok(Some(User))` - 查询成功，返回用户信息
    /// * `Ok(None)` - 用户不存在
    /// * `Err(Error)` - 查询失败
    /// 
    /// # 错误
    /// 当数据库连接失败或查询异常时返回错误
    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<User>> {
        // 实现代码...
    }
}
```

#### 1.2 行内注释
对于关键且复杂的逻辑，必须添加清晰的中文注释：

```rust
pub fn auto_wire(&mut self) -> Result<()> {
    // 获取所有待装配的组件
    let mut pending_components = self.get_pending_components();
    
    // 使用拓扑排序解决依赖顺序问题
    // 这里需要检测循环依赖，如果发现循环依赖则抛出错误
    let sorted_components = self.topological_sort(&pending_components)?;
    
    // 按依赖顺序逐个装配组件
    for component in sorted_components {
        // 检查组件的依赖是否都已经注册
        if !self.check_dependencies(&component) {
            return Err(Error::dependency_not_found(&component.name));
        }
        
        // 执行依赖注入
        self.inject_dependencies(&component)?;
    }
    
    Ok(())
}
```

#### 1.3 模块注释
每个模块文件开头必须有模块说明：

```rust
//! 依赖注入容器模块
//! 
//! 提供类似 Spring IoC 容器的功能，支持：
//! - 组件自动注册和发现
//! - 依赖自动注入
//! - 生命周期管理
//! - 循环依赖检测

use std::collections::HashMap;
```

### 2. 错误处理规范

#### 2.1 错误类型定义
```rust
/// 应用程序错误类型
/// 
/// 定义了所有可能发生的错误类型，便于统一处理
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// 配置相关错误
    #[error("配置错误: {0}")]
    Configuration(#[from] config::ConfigError),
    
    /// 数据库相关错误  
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),
    
    /// 业务逻辑错误
    #[error("业务错误: {message} (错误码: {code})")]
    Business { message: String, code: String },
}
```

#### 2.2 错误传播
```rust
/// 创建新用户
/// 
/// # 业务规则
/// - 用户名不能重复
/// - 邮箱格式必须正确  
/// - 密码长度至少8位
pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
    // 验证输入参数
    self.validate_create_request(&request)?;
    
    // 检查用户名是否已存在
    if self.user_repository.exists_by_username(&request.username).await? {
        return Err(Error::business("USER_EXISTS", "用户名已存在"));
    }
    
    // 创建用户实体
    let user = User::from_request(request);
    
    // 保存到数据库
    self.user_repository.save(user).await
        .map_err(|e| Error::internal(format!("保存用户失败: {}", e)))
}
```

### 3. 代码组织规范

#### 3.1 文件结构
```
src/
├── lib.rs              // 库入口，重新导出主要API
├── application/        // 应用启动相关
│   ├── mod.rs
│   ├── context.rs      // 应用上下文
│   └── lifecycle.rs    // 生命周期管理
├── config/             // 配置管理
│   ├── mod.rs
│   ├── manager.rs      // 配置管理器
│   └── properties.rs   // 配置属性定义
├── container/          // 依赖注入容器
│   ├── mod.rs
│   ├── registry.rs     // 组件注册
│   └── injection.rs    // 依赖注入
└── web/               // Web层支持
    ├── mod.rs
    ├── controller.rs   // 控制器支持
    └── router.rs      // 路由管理
```

#### 3.2 导入规范
```rust
// 标准库导入
use std::collections::HashMap;
use std::sync::Arc;

// 第三方库导入
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

// 本项目导入
use crate::config::ConfigurationManager;
use crate::error::{Error, Result};
```

### 4. 测试规范

#### 4.1 单元测试
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    /// 测试用户创建功能
    /// 
    /// 验证正常情况下用户创建流程
    #[tokio::test]
    async fn test_create_user_success() {
        // 准备测试数据
        let request = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        // 创建模拟依赖
        let mock_repository = MockUserRepository::new();
        let user_service = UserService::new(Arc::new(mock_repository));
        
        // 执行测试
        let result = user_service.create_user(request).await;
        
        // 验证结果
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.username, "testuser");
    }

    /// 测试重复用户名的错误处理
    #[tokio::test]
    async fn test_create_user_duplicate_username() {
        // 测试实现...
    }
}
```

### 5. 性能规范

#### 5.1 异步代码
```rust
/// 批量查询用户信息
/// 
/// 使用并发查询提高性能，避免串行等待
pub async fn get_users_batch(&self, ids: Vec<i64>) -> Result<Vec<User>> {
    // 使用 join_all 并发执行查询，提高性能
    let futures = ids.into_iter().map(|id| self.get_user_by_id(id));
    let results = futures::future::join_all(futures).await;
    
    // 收集成功的结果，记录失败的情况
    let mut users = Vec::new();
    for result in results {
        match result {
            Ok(Some(user)) => users.push(user),
            Ok(None) => {
                // 用户不存在，记录警告但继续处理
                tracing::warn!("用户不存在，已跳过");
            },
            Err(e) => {
                // 查询失败，记录错误并返回
                tracing::error!("批量查询用户失败: {}", e);
                return Err(e);
            }
        }
    }
    
    Ok(users)
}
```

## Git 提交规范

### 1. 提交消息格式

```
<type>(<scope>): <subject>

<body>

<footer>
```

### 2. 提交类型 (type)

- `feat`: 新功能
- `fix`: 修复bug
- `docs`: 文档变更
- `style`: 代码格式化，不影响代码运行
- `refactor`: 重构代码
- `test`: 增加测试
- `chore`: 构建过程或辅助工具的变动
- `perf`: 性能优化
- `ci`: CI/CD相关变更
- `build`: 构建系统变更

### 3. 提交示例

```bash
# 新功能
feat(core): 实现依赖注入容器的自动装配功能

添加了组件自动发现和依赖注入的核心逻辑：
- 实现拓扑排序解决依赖顺序问题
- 添加循环依赖检测机制
- 支持单例和原型两种生命周期

Closes #123

# 修复bug
fix(macro): 修复 #[Component] 宏生成代码的语法错误

修复了宏展开时缺少分号导致的编译错误
影响范围：所有使用 #[Component] 注解的结构体

# 文档更新
docs(readme): 更新快速开始指南

增加了更详细的安装步骤和配置说明
添加了常见问题解答章节

# 重构
refactor(container): 优化依赖注入容器的性能

- 使用 HashMap 替换 Vec 提高查找效率
- 减少不必要的克隆操作
- 优化内存使用
```

### 4. Scope 范围

- `core`: 核心库 (axum-boot-core)
- `macro`: 宏系统 (axum-boot-macro) 
- `web`: Web启动器 (axum-boot-starter-web)
- `data-mysql`: MySQL启动器
- `data-redis`: Redis启动器
- `examples`: 示例项目
- `docs`: 文档
- `ci`: 持续集成

### 5. 提交最佳实践

1. **提交频率**: 每个逻辑单元完成后及时提交
2. **提交粒度**: 一个提交只解决一个问题
3. **消息语言**: 使用中文描述，便于团队理解
4. **测试保证**: 提交前确保所有测试通过
5. **代码审查**: 重要变更需要经过代码审查

### 6. 分支管理

```
master          # 主分支，保持稳定
├── develop     # 开发分支，集成最新功能  
├── feature/*   # 功能分支
├── fix/*       # 修复分支
└── release/*   # 发布分支
```

## 开发流程

### 1. 功能开发流程
1. 从 `develop` 创建 `feature/功能名` 分支
2. 在功能分支上开发，遵循代码规范
3. 编写测试用例，确保覆盖率
4. 提交代码，使用规范的提交消息
5. 创建 Pull Request 到 `develop` 分支
6. 经过代码审查后合并

### 2. 发布流程
1. 从 `develop` 创建 `release/版本号` 分支
2. 进行最后的测试和bug修复
3. 更新版本号和变更日志
4. 合并到 `master` 并打标签
5. 发布到 crates.io

## 性能要求

- 启动时间 < 1秒
- 内存占用合理，避免内存泄漏
- 支持高并发请求处理
- 编译时间优化，减少依赖

## 安全要求

- 输入参数验证
- SQL注入防护  
- XSS攻击防护
- 敏感信息不得记录到日志
- 密码等敏感数据必须加密存储