# 贡献指南

感谢您对 AxumBoot 项目的关注和贡献意愿！本指南将帮助您了解如何参与项目开发。

## 🤝 如何贡献

我们欢迎各种形式的贡献：

- 🐛 **报告 Bug** - 发现问题时请提交 Issue
- 💡 **功能建议** - 提出新功能或改进建议
- 📖 **文档改进** - 完善文档、修复错别字
- 🔧 **代码贡献** - 修复 Bug、实现新功能
- 🧪 **测试用例** - 增加测试覆盖率
- 📝 **示例项目** - 提供使用示例

## 🚀 快速开始

### 1. Fork 和 Clone

```bash
# Fork 项目到你的 GitHub 账户
# 然后 clone 到本地
git clone https://github.com/你的用户名/axum-boot.git
cd axum-boot

# 添加上游仓库
git remote add upstream https://github.com/axumboot/axum-boot.git
```

### 2. 设置开发环境

```bash
# 安装 Rust (如果还没安装)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 确保版本满足要求
rustc --version  # 应该 >= 1.70

# 安装开发工具
cargo install cargo-watch cargo-nextest cargo-expand

# 检查项目是否能正常构建
cargo check
```

### 3. 创建分支

```bash
# 从最新的 develop 分支创建功能分支
git checkout develop
git pull upstream develop
git checkout -b feature/你的功能名
```

## 📋 开发流程

### 1. 开发新功能

```bash
# 创建功能分支
git checkout -b feature/awesome-feature

# 进行开发
# ... 编写代码、测试 ...

# 提交更改
git add .
git commit -m "feat(core): 添加新的令人敬畏的功能"

# 推送到你的 fork
git push origin feature/awesome-feature
```

### 2. 修复 Bug

```bash
# 创建修复分支
git checkout -b fix/issue-123

# 修复问题
# ... 编写代码、测试 ...

# 提交更改
git add .
git commit -m "fix(web): 修复路由重复注册问题

修复了在某些情况下路由会被重复注册的问题。

Fixes #123"

# 推送分支
git push origin fix/issue-123
```

### 3. 提交 Pull Request

1. 访问 GitHub 上你的 fork 仓库
2. 点击 "Compare & pull request"
3. 选择正确的分支：
   - Base: `axumboot/axum-boot` 的 `develop` 分支
   - Head: 你的分支
4. 填写 PR 模板
5. 等待代码审查

## 📝 代码规范

### 1. Commit 消息规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<类型>(<范围>): <描述>

<详细说明>

<脚注>
```

**类型：**
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建、CI 等
- `perf`: 性能优化

**范围：**
- `core`: 核心库
- `macro`: 宏系统
- `web`: Web 启动器
- `data-mysql`: MySQL 启动器
- `data-redis`: Redis 启动器
- `examples`: 示例
- `docs`: 文档

**示例：**

```bash
feat(core): 添加配置热重载功能

实现了配置文件的监听和热重载机制：
- 监听配置文件变更
- 自动重新加载配置
- 通知相关组件配置变更

Closes #45

git commit -m "$(cat <<'EOF'
feat(core): 添加配置热重载功能

实现了配置文件的监听和热重载机制：
- 监听配置文件变更
- 自动重新加载配置  
- 通知相关组件配置变更

Closes #45
EOF
)"
```

### 2. 代码风格

请遵循项目的 [代码规范](../CLAUDE.md)：

**文档注释：**
```rust
/// 用户服务，提供用户相关的业务逻辑
/// 
/// # 功能
/// - 创建用户
/// - 查询用户
/// - 更新用户
/// 
/// # 示例
/// ```rust
/// let service = UserService::new(repository);
/// let user = service.create_user(request).await?;
/// ```
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    /// 根据 ID 查询用户
    /// 
    /// # 参数
    /// * `id` - 用户 ID
    /// 
    /// # 返回值
    /// * `Ok(Some(User))` - 找到用户
    /// * `Ok(None)` - 用户不存在
    /// * `Err(Error)` - 查询失败
    pub async fn get_user_by_id(&self, id: u64) -> Result<Option<User>> {
        // 首先验证 ID 的有效性
        if id == 0 {
            return Err(Error::validation("用户 ID 不能为 0"));
        }
        
        // 查询数据库
        self.repository.find_by_id(id).await
            .map_err(|e| Error::internal(format!("查询用户失败: {}", e)))
    }
}
```

**错误处理：**
```rust
// 好的做法
pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
    // 验证输入
    if request.name.trim().is_empty() {
        return Err(Error::validation("用户名不能为空"));
    }
    
    // 检查业务规则
    if self.repository.exists_by_email(&request.email).await? {
        return Err(Error::business("USER_EXISTS", "用户已存在"));
    }
    
    // 执行操作
    self.repository.save(User::from(request)).await
        .map_err(|e| Error::internal(format!("保存用户失败: {}", e)))
}
```

### 3. 测试要求

- 所有新功能都必须有测试用例
- Bug 修复必须包含回归测试
- 保持测试覆盖率 >= 80%

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    /// 测试用户创建功能
    /// 
    /// 验证正常情况下的用户创建流程
    #[tokio::test]
    async fn test_create_user_success() {
        // 准备测试数据
        let request = CreateUserRequest {
            name: "测试用户".to_string(),
            email: "test@example.com".to_string(),
        };
        
        // 创建模拟依赖
        let mock_repository = MockUserRepository::new();
        let service = UserService::new(Arc::new(mock_repository));
        
        // 执行测试
        let result = service.create_user(request).await;
        
        // 验证结果
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "测试用户");
        assert_eq!(user.email, "test@example.com");
    }
    
    /// 测试用户名为空的错误处理
    #[tokio::test]
    async fn test_create_user_empty_name() {
        let request = CreateUserRequest {
            name: "".to_string(),
            email: "test@example.com".to_string(),
        };
        
        let service = create_test_user_service();
        let result = service.create_user(request).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Validation { message } => {
                assert_eq!(message, "用户名不能为空");
            },
            _ => panic!("应该是验证错误"),
        }
    }
}
```

## 🧪 测试

### 1. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块的测试
cargo test user_service

# 运行集成测试
cargo test --test integration

# 生成测试覆盖率报告
cargo tarpaulin --out Html
```

### 2. 性能测试

```bash
# 运行性能基准测试
cargo bench

# 分析性能
cargo flamegraph --bin my-app
```

## 📖 文档

### 1. API 文档

```bash
# 生成文档
cargo doc --open

# 检查文档链接
cargo doc --no-deps
```

### 2. 示例代码

确保示例代码可以正常编译和运行：

```bash
# 测试文档中的代码示例
cargo test --doc

# 运行示例项目
cd examples/hello-world
cargo run
```

## 🔍 代码审查

### 1. 自我审查清单

提交 PR 前请检查：

- [ ] 代码遵循项目规范
- [ ] 添加了适当的测试用例
- [ ] 更新了相关文档
- [ ] 所有测试都通过
- [ ] 没有引入新的编译警告
- [ ] commit 消息符合规范

### 2. PR 模板

```markdown
## 变更说明
简要描述本次变更的内容和原因。

## 变更类型
- [ ] Bug 修复
- [ ] 新功能
- [ ] 破坏性变更
- [ ] 文档更新
- [ ] 性能优化

## 测试
- [ ] 添加了新的测试用例
- [ ] 所有现有测试都通过
- [ ] 手动测试通过

## 检查清单
- [ ] 代码遵循项目规范
- [ ] 自我审查完成
- [ ] 相关文档已更新
- [ ] commit 消息符合规范
```

## 🚀 发布流程

### 1. 版本规划

我们使用 [Semantic Versioning](https://semver.org/)：

- `MAJOR`: 破坏性变更
- `MINOR`: 新功能（向后兼容）
- `PATCH`: Bug 修复（向后兼容）

### 2. 发布步骤

```bash
# 1. 更新版本号
cargo set-version 0.2.0

# 2. 更新 CHANGELOG.md
# 记录本次发布的所有变更

# 3. 创建发布分支
git checkout -b release/0.2.0

# 4. 提交版本变更
git add .
git commit -m "chore(release): 发布 v0.2.0"

# 5. 创建 PR 到 master
# 6. 合并后创建 tag
git tag v0.2.0
git push origin v0.2.0

# 7. 发布到 crates.io
cargo publish -p axum-boot-core
cargo publish -p axum-boot-macro
cargo publish -p axum-boot-starter-web
```

## 🎯 贡献领域

### 1. 核心功能开发

- 依赖注入系统优化
- 配置系统增强
- 错误处理改进
- 性能优化

### 2. 启动器开发

- Web 启动器完善
- 数据库启动器
- 缓存启动器
- 消息队列启动器

### 3. 工具和集成

- IDE 插件
- 项目模板
- Docker 镜像
- Kubernetes 配置

### 4. 文档和示例

- 使用指南
- 最佳实践
- API 文档
- 示例项目

## 🏆 贡献者认可

我们重视每一份贡献：

- 所有贡献者都会在 README 中列出
- 重大贡献会在发布说明中特别感谢
- 长期贡献者可以成为项目维护者

## ❓ 获取帮助

如果你需要帮助：

- 📖 查看 [文档](../README.md)
- 💬 在 [Discussions](https://github.com/axumboot/axum-boot/discussions) 中提问
- 🐛 在 [Issues](https://github.com/axumboot/axum-boot/issues) 中报告问题
- 📧 发邮件给维护者团队

## 📄 行为准则

我们致力于维护一个友好、包容的社区环境：

- 尊重不同观点和经验水平
- 接受建设性的批评
- 专注于对社区最有利的事情
- 对其他社区成员表现出同理心

## 🙏 感谢

感谢您考虑为 AxumBoot 做出贡献！每一份贡献都让这个项目变得更好。

---

**Happy Coding!** 🎉