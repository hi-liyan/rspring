# 数据处理器示例

这个示例展示了如何使用 `axum-boot-core` 构建一个非 Web 应用程序 - 一个定时数据处理器。

## 🎯 应用场景

- **定时任务处理** - 从数据源定期获取和处理数据
- **批量数据处理** - 批量处理数据记录
- **后台服务** - 无Web界面的后台处理服务
- **CLI工具** - 命令行数据处理工具

## 🏗️ 架构设计

```
DataProcessorApplication
├── DataSourceService      # 数据源服务
├── DataProcessorService   # 数据处理服务  
├── ResultStorageService   # 结果存储服务
└── TaskSchedulerService   # 任务调度服务
```

## 🚀 核心特性

### 1. **纯依赖注入**
- 无Web框架依赖
- 清晰的服务分离
- 自动配置管理

### 2. **灵活配置系统**
- TOML配置文件
- 环境变量支持
- 类型安全的配置绑定

### 3. **统一错误处理**
- 非HTTP特定的错误类型
- 统一的错误传播机制

### 4. **优雅关闭**
- 信号处理
- 资源清理

## 🔧 配置说明

```toml
# application.toml

[app]
name = "数据处理器"        # 应用名称
version = "1.0.0"         # 版本
batch_size = 10           # 每批处理数量
interval_seconds = 30     # 处理间隔（秒）

[logging]
level = "info"            # 日志级别
format = "text"           # 日志格式
```

## 🚀 运行示例

```bash
# 进入示例目录
cd examples/data-processor

# 运行应用
cargo run

# 使用环境变量覆盖配置
APP_BATCH_SIZE=20 cargo run

# 调试模式
RUST_LOG=debug cargo run
```

## 📋 预期输出

```
INFO  启动数据处理应用程序
INFO  应用上下文初始化完成
INFO  应用配置加载完成: 数据处理器
INFO  所有服务初始化完成
INFO  启动定时数据处理任务，间隔: 30秒
INFO  开始数据处理循环
INFO  从数据源获取 10 条待处理记录
INFO  开始批量处理 10 条记录
INFO  批量处理完成，成功处理 10 条记录
INFO  保存 10 条处理结果
INFO  所有结果保存完成
INFO  数据处理循环完成
```

## 💡 扩展示例

这个基础框架可以轻松扩展为：

1. **ETL工具** - 数据提取、转换、加载
2. **消息队列消费者** - 处理队列中的消息
3. **日志分析器** - 分析和处理日志文件
4. **数据同步工具** - 同步不同数据源的数据
5. **报告生成器** - 定时生成各种报告

## 🔍 与Web应用的对比

| 功能 | data-processor | web应用 |
|------|----------------|---------|
| 依赖 | axum-boot-core | axum-boot-starter-web |
| 错误类型 | 通用Error | WebError + HTTP状态码 |
| 响应格式 | 无 | ApiResponse<T> |
| 启动方式 | 自定义run()实现 | 自动Web服务器 |
| 适用场景 | 后台任务、CLI | REST API、Web服务 |

这个示例充分展示了 `axum-boot-core` 作为纯核心框架的能力和灵活性。