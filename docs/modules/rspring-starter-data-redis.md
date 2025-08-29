# rspring-starter-data-redis

[![crates.io](https://img.shields.io/crates/v/rspring-starter-data-redis.svg)](https://crates.io/crates/rspring-starter-data-redis)
[![docs.rs](https://img.shields.io/docsrs/rspring-starter-data-redis)](https://docs.rs/rspring-starter-data-redis)

**rspring-starter-data-redis** 是 RSpring 框架的 Redis Starter，集成 Rust 最常用的 Redis 客户端库并提供了简化的 Redis 操作。通过自动装配和丰富的配置选项，让 Redis 的使用变得更加简单。

## 🎯 核心功能

- **Redis 客户端集成** - 集成 Rust 最优秀的 Redis 客户端库
- **自动装配** - 自动配置 Redis 连接和连接池
- **简化操作** - 提供简洁的 API 封装，简化 Redis 操作
- **多数据结构** - 支持 String、Hash、List、Set、ZSet 等数据结构
- **序列化支持** - 内置 JSON、MessagePack 等序列化支持
- **连接池管理** - 高效的连接池管理和配置
- 📈 **性能监控** - 内置的 Redis 性能分析
- 🛡️ **故障恢复** - 自动重连和故障转移

### 📦 快速开始

添加依赖到 `Cargo.toml`：

```toml
[dependencies]
axum-boot-starter-data-redis = "0.1.0"
```

## 🏗️ 架构设计

### 核心组件架构

```
axum-boot-starter-data-redis
├── config/
│   ├── redis.rs           # Redis 配置
│   └── pool.rs           # 连接池管理
├── cache/
│   ├── annotations.rs     # 缓存注解
│   ├── manager.rs        # 缓存管理器
│   └── operations.rs     # 缓存操作
├── template/
│   ├── redis_template.rs  # Redis 操作模板
│   └── serializer.rs     # 序列化器
├── repository/
│   ├── crud.rs           # CRUD 操作接口
│   └── hash.rs           # Hash 操作
└── lib.rs                # 模块导出
```

### 模块依赖关系

```mermaid
graph TD
    A[axum-boot-starter-data-redis] --> B[axum-boot-core]
    A --> C[redis]
    A --> D[tokio]
    A --> E[serde]
    A --> F[serde_json]
    A --> G[rmp-serde]
    
    B --> H[axum-boot-macro]
    B --> I[tracing]
```

## 📚 API 参考

### 核心类型

#### RedisConfig

Redis 配置结构体：

```rust
/// Redis 配置
#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis 连接 URL
    /// 格式: redis://[:password]@host:port[/db]
    pub url: String,
    
    /// 连接池大小
    pub pool_size: u32,
    
    /// 连接超时时间（毫秒）
    pub connect_timeout: u64,
    
    /// 命令执行超时时间（毫秒）
    pub command_timeout: u64,
    
    /// 重试次数
    pub retry_attempts: u32,
    
    /// 重试间隔（毫秒）
    pub retry_interval: u64,
    
    /// 是否启用 Redis 日志
    pub enable_logging: bool,
    
    /// 默认 TTL（秒）
    pub default_ttl: u64,
    
    /// 序列化方式
    pub serialization: SerializationType,
}

/// 序列化类型
#[derive(Debug, Clone, Deserialize)]
pub enum SerializationType {
    Json,
    MessagePack,
    Bincode,
}
```

#### RedisTemplate

Redis 操作模板：

```rust
/// Redis 操作模板
#[derive(Debug, Clone)]
pub struct RedisTemplate {
    pool: Arc<RedisPool>,
    serializer: Arc<dyn Serializer>,
    default_ttl: Option<u64>,
}

impl RedisTemplate {
    /// 创建新的 RedisTemplate
    pub fn new(pool: Arc<RedisPool>, serializer: Arc<dyn Serializer>) -> Self;
    
    /// 设置值
    pub async fn set<T>(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize;
    
    /// 设置值（带 TTL）
    pub async fn set_ex<T>(&self, key: &str, value: &T, ttl: u64) -> Result<()>
    where
        T: Serialize;
    
    /// 获取值
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned;
    
    /// 删除键
    pub async fn del(&self, key: &str) -> Result<bool>;
    
    /// 检查键是否存在
    pub async fn exists(&self, key: &str) -> Result<bool>;
    
    /// 设置过期时间
    pub async fn expire(&self, key: &str, ttl: u64) -> Result<bool>;
    
    /// 获取 TTL
    pub async fn ttl(&self, key: &str) -> Result<i64>;
}
```

#### HashOperations

Hash 数据结构操作：

```rust
/// Hash 操作接口
pub struct HashOperations {
    template: Arc<RedisTemplate>,
}

impl HashOperations {
    /// 设置 Hash 字段
    pub async fn hset<T>(&self, key: &str, field: &str, value: &T) -> Result<()>
    where
        T: Serialize;
    
    /// 获取 Hash 字段
    pub async fn hget<T>(&self, key: &str, field: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned;
    
    /// 获取所有 Hash 字段
    pub async fn hgetall<T>(&self, key: &str) -> Result<HashMap<String, T>>
    where
        T: DeserializeOwned;
    
    /// 删除 Hash 字段
    pub async fn hdel(&self, key: &str, field: &str) -> Result<bool>;
    
    /// 检查 Hash 字段是否存在
    pub async fn hexists(&self, key: &str, field: &str) -> Result<bool>;
    
    /// 获取 Hash 长度
    pub async fn hlen(&self, key: &str) -> Result<i64>;
    
    /// 获取所有字段名
    pub async fn hkeys(&self, key: &str) -> Result<Vec<String>>;
}
```

#### ListOperations

List 数据结构操作：

```rust
/// List 操作接口
pub struct ListOperations {
    template: Arc<RedisTemplate>,
}

impl ListOperations {
    /// 左推入
    pub async fn lpush<T>(&self, key: &str, value: &T) -> Result<i64>
    where
        T: Serialize;
    
    /// 右推入
    pub async fn rpush<T>(&self, key: &str, value: &T) -> Result<i64>
    where
        T: Serialize;
    
    /// 左弹出
    pub async fn lpop<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned;
    
    /// 右弹出
    pub async fn rpop<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: DeserializeOwned;
    
    /// 获取列表范围
    pub async fn lrange<T>(&self, key: &str, start: i64, stop: i64) -> Result<Vec<T>>
    where
        T: DeserializeOwned;
    
    /// 获取列表长度
    pub async fn llen(&self, key: &str) -> Result<i64>;
    
    /// 按索引获取元素
    pub async fn lindex<T>(&self, key: &str, index: i64) -> Result<Option<T>>
    where
        T: DeserializeOwned;
}
```

### 缓存注解系统

#### @Cacheable

缓存查询结果：

```rust
/// 缓存注解
/// 
/// # 参数
/// - `key`: 缓存键（支持 SpEL 表达式）
/// - `ttl`: 过期时间（秒）
/// - `condition`: 缓存条件
/// - `unless`: 排除条件
/// 
/// # 示例
/// ```rust
/// impl UserService {
///     #[Cacheable(key = "user:#{id}", ttl = 3600)]
///     async fn get_user(&self, id: u64) -> Result<Option<User>>;
///     
///     #[Cacheable(key = "users:active", ttl = 1800, condition = "#{active == true}")]
///     async fn get_active_users(&self, active: bool) -> Result<Vec<User>>;
/// }
/// ```
#[proc_macro_attribute]
pub fn Cacheable(args: TokenStream, input: TokenStream) -> TokenStream;
```

#### @CachePut

更新缓存：

```rust
/// 缓存更新注解
/// 
/// # 示例
/// ```rust
/// impl UserService {
///     #[CachePut(key = "user:#{user.id}", ttl = 3600)]
///     async fn update_user(&self, user: &User) -> Result<User>;
/// }
/// ```
#[proc_macro_attribute]
pub fn CachePut(args: TokenStream, input: TokenStream) -> TokenStream;
```

#### @CacheEvict

删除缓存：

```rust
/// 缓存删除注解
/// 
/// # 参数
/// - `key`: 要删除的缓存键
/// - `all_entries`: 是否删除所有相关缓存
/// - `before_invocation`: 是否在方法执行前删除
/// 
/// # 示例
/// ```rust
/// impl UserService {
///     #[CacheEvict(key = "user:#{id}")]
///     async fn delete_user(&self, id: u64) -> Result<()>;
///     
///     #[CacheEvict(all_entries = true)]
///     async fn clear_all_users(&self) -> Result<()>;
/// }
/// ```
#[proc_macro_attribute]
pub fn CacheEvict(args: TokenStream, input: TokenStream) -> TokenStream;
```

### Repository 系统

#### RedisRepository

Redis 数据仓库接口：

```rust
/// Redis Repository trait
#[async_trait]
pub trait RedisRepository<T, ID>: Send + Sync
where
    T: Serialize + DeserializeOwned,
    ID: Display + Send + Sync,
{
    /// 保存实体
    async fn save(&self, entity: &T, id: &ID) -> Result<()>;
    
    /// 保存实体（带 TTL）
    async fn save_with_ttl(&self, entity: &T, id: &ID, ttl: u64) -> Result<()>;
    
    /// 根据 ID 查找
    async fn find_by_id(&self, id: &ID) -> Result<Option<T>>;
    
    /// 检查实体是否存在
    async fn exists_by_id(&self, id: &ID) -> Result<bool>;
    
    /// 根据 ID 删除
    async fn delete_by_id(&self, id: &ID) -> Result<bool>;
    
    /// 获取所有键
    async fn find_all_keys(&self) -> Result<Vec<String>>;
    
    /// 批量保存
    async fn save_all(&self, entities: Vec<(ID, T)>) -> Result<()>;
    
    /// 批量查找
    async fn find_all_by_ids(&self, ids: Vec<ID>) -> Result<HashMap<String, T>>;
}
```

### 序列化器

#### Serializer

序列化接口：

```rust
/// 序列化器 trait
#[async_trait]
pub trait Serializer: Send + Sync {
    /// 序列化
    async fn serialize<T>(&self, value: &T) -> Result<Vec<u8>>
    where
        T: Serialize + Send + Sync;
    
    /// 反序列化
    async fn deserialize<T>(&self, data: &[u8]) -> Result<T>
    where
        T: DeserializeOwned + Send;
}

/// JSON 序列化器
pub struct JsonSerializer;

/// MessagePack 序列化器
pub struct MessagePackSerializer;

/// Bincode 序列化器
pub struct BincodeSerializer;
```

## 🚀 使用指南

### 基础使用

#### 1. 配置 Redis

```toml
# application.toml
[redis]
url = "redis://localhost:6379"
pool_size = 10
connect_timeout = 5000
command_timeout = 3000
retry_attempts = 3
retry_interval = 1000
enable_logging = true
default_ttl = 3600
serialization = "json"

# 集群配置（可选）
[redis.cluster]
enabled = false
nodes = ["redis://node1:6379", "redis://node2:6379", "redis://node3:6379"]

# Sentinel 配置（可选）
[redis.sentinel]
enabled = false
service_name = "mymaster"
sentinels = ["redis://sentinel1:26379", "redis://sentinel2:26379"]
```

#### 2. 基础操作

```rust
use axum_boot_starter_data_redis::*;
use serde::{Deserialize, Serialize};

/// 用户缓存服务
#[derive(Service)]
pub struct UserCacheService {
    redis_template: Arc<RedisTemplate>,
}

impl UserCacheService {
    /// 缓存用户信息
    pub async fn cache_user(&self, user: &User) -> Result<()> {
        let key = format!("user:{}", user.id);
        self.redis_template.set_ex(&key, user, 3600).await
    }
    
    /// 获取缓存的用户信息
    pub async fn get_cached_user(&self, user_id: u64) -> Result<Option<User>> {
        let key = format!("user:{}", user_id);
        self.redis_template.get(&key).await
    }
    
    /// 删除用户缓存
    pub async fn evict_user_cache(&self, user_id: u64) -> Result<()> {
        let key = format!("user:{}", user_id);
        self.redis_template.del(&key).await?;
        Ok(())
    }
}
```

#### 3. 使用缓存注解

```rust
/// 用户服务（使用缓存注解）
#[derive(Service)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
    redis_template: Arc<RedisTemplate>,
}

impl UserService {
    /// 获取用户（自动缓存）
    #[Cacheable(key = "user:#{id}", ttl = 3600)]
    pub async fn get_user(&self, id: u64) -> Result<Option<User>> {
        // 这个方法的结果会自动缓存到 Redis
        self.user_repository.find_by_id(id).await
    }
    
    /// 更新用户（更新缓存）
    #[CachePut(key = "user:#{user.id}", ttl = 3600)]
    pub async fn update_user(&self, user: &User) -> Result<User> {
        // 更新数据库后自动更新缓存
        let updated_user = self.user_repository.save(user).await?;
        Ok(updated_user)
    }
    
    /// 删除用户（清除缓存）
    #[CacheEvict(key = "user:#{id}")]
    pub async fn delete_user(&self, id: u64) -> Result<()> {
        // 删除数据后自动清除缓存
        self.user_repository.delete_by_id(id).await
    }
    
    /// 获取活跃用户列表（条件缓存）
    #[Cacheable(
        key = "users:active:#{page}:#{size}", 
        ttl = 1800,
        condition = "#{page <= 10 && size <= 100}"
    )]
    pub async fn get_active_users(&self, page: u64, size: u64) -> Result<Vec<User>> {
        // 只有当页码 <= 10 且大小 <= 100 时才缓存
        self.user_repository.find_active_paged(page, size).await
    }
}
```

### 高级功能

#### 1. Hash 操作

```rust
/// 用户配置服务
#[derive(Service)]
pub struct UserConfigService {
    hash_ops: Arc<HashOperations>,
}

impl UserConfigService {
    /// 设置用户配置
    pub async fn set_user_config(&self, user_id: u64, key: &str, value: &str) -> Result<()> {
        let hash_key = format!("user_config:{}", user_id);
        self.hash_ops.hset(&hash_key, key, &value).await
    }
    
    /// 获取用户配置
    pub async fn get_user_config(&self, user_id: u64, key: &str) -> Result<Option<String>> {
        let hash_key = format!("user_config:{}", user_id);
        self.hash_ops.hget(&hash_key, key).await
    }
    
    /// 获取用户所有配置
    pub async fn get_all_user_configs(&self, user_id: u64) -> Result<HashMap<String, String>> {
        let hash_key = format!("user_config:{}", user_id);
        self.hash_ops.hgetall(&hash_key).await
    }
    
    /// 批量设置用户配置
    pub async fn set_user_configs(&self, user_id: u64, configs: HashMap<String, String>) -> Result<()> {
        let hash_key = format!("user_config:{}", user_id);
        
        for (key, value) in configs {
            self.hash_ops.hset(&hash_key, &key, &value).await?;
        }
        
        Ok(())
    }
}
```

#### 2. List 操作（消息队列）

```rust
/// 消息队列服务
#[derive(Service)]
pub struct MessageQueueService {
    list_ops: Arc<ListOperations>,
}

impl MessageQueueService {
    /// 发送消息到队列
    pub async fn send_message(&self, queue: &str, message: &Message) -> Result<()> {
        self.list_ops.rpush(queue, message).await?;
        Ok(())
    }
    
    /// 从队列接收消息
    pub async fn receive_message(&self, queue: &str) -> Result<Option<Message>> {
        self.list_ops.lpop(queue).await
    }
    
    /// 批量发送消息
    pub async fn send_batch_messages(&self, queue: &str, messages: Vec<Message>) -> Result<()> {
        for message in messages {
            self.list_ops.rpush(queue, &message).await?;
        }
        Ok(())
    }
    
    /// 获取队列长度
    pub async fn get_queue_size(&self, queue: &str) -> Result<i64> {
        self.list_ops.llen(queue).await
    }
    
    /// 查看队列中的消息（不移除）
    pub async fn peek_messages(&self, queue: &str, count: i64) -> Result<Vec<Message>> {
        self.list_ops.lrange(queue, 0, count - 1).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub content: String,
    pub timestamp: i64,
    pub retry_count: u32,
}
```

#### 3. Set 操作（标签系统）

```rust
/// 标签服务
#[derive(Service)]
pub struct TagService {
    set_ops: Arc<SetOperations>,
}

impl TagService {
    /// 为用户添加标签
    pub async fn add_user_tag(&self, user_id: u64, tag: &str) -> Result<()> {
        let key = format!("user_tags:{}", user_id);
        self.set_ops.sadd(&key, tag).await?;
        Ok(())
    }
    
    /// 移除用户标签
    pub async fn remove_user_tag(&self, user_id: u64, tag: &str) -> Result<()> {
        let key = format!("user_tags:{}", user_id);
        self.set_ops.srem(&key, tag).await?;
        Ok(())
    }
    
    /// 获取用户所有标签
    pub async fn get_user_tags(&self, user_id: u64) -> Result<Vec<String>> {
        let key = format!("user_tags:{}", user_id);
        self.set_ops.smembers(&key).await
    }
    
    /// 检查用户是否有指定标签
    pub async fn user_has_tag(&self, user_id: u64, tag: &str) -> Result<bool> {
        let key = format!("user_tags:{}", user_id);
        self.set_ops.sismember(&key, tag).await
    }
    
    /// 获取两个用户的共同标签
    pub async fn get_common_tags(&self, user_id1: u64, user_id2: u64) -> Result<Vec<String>> {
        let key1 = format!("user_tags:{}", user_id1);
        let key2 = format!("user_tags:{}", user_id2);
        self.set_ops.sinter(&[key1, key2]).await
    }
}
```

#### 4. 分布式锁

```rust
/// 分布式锁服务
#[derive(Service)]
pub struct DistributedLockService {
    redis_template: Arc<RedisTemplate>,
}

impl DistributedLockService {
    /// 获取分布式锁
    pub async fn acquire_lock(&self, key: &str, ttl: u64) -> Result<Option<String>> {
        let lock_key = format!("lock:{}", key);
        let lock_value = uuid::Uuid::new_v4().to_string();
        
        // 使用 SET NX EX 原子操作
        let result: bool = redis::cmd("SET")
            .arg(&lock_key)
            .arg(&lock_value)
            .arg("NX")
            .arg("EX")
            .arg(ttl)
            .query_async(&mut *self.redis_template.get_connection().await?)
            .await?;
        
        if result {
            Ok(Some(lock_value))
        } else {
            Ok(None)
        }
    }
    
    /// 释放分布式锁
    pub async fn release_lock(&self, key: &str, lock_value: &str) -> Result<bool> {
        let lock_key = format!("lock:{}", key);
        
        // 使用 Lua 脚本确保原子性
        let script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#;
        
        let result: i64 = redis::Script::new(script)
            .key(&lock_key)
            .arg(lock_value)
            .invoke_async(&mut *self.redis_template.get_connection().await?)
            .await?;
        
        Ok(result == 1)
    }
    
    /// 执行带锁的操作
    pub async fn with_lock<F, R>(&self, key: &str, ttl: u64, f: F) -> Result<Option<R>>
    where
        F: FnOnce() -> BoxFuture<'static, Result<R>>,
    {
        if let Some(lock_value) = self.acquire_lock(key, ttl).await? {
            let result = f().await;
            self.release_lock(key, &lock_value).await?;
            Ok(Some(result?))
        } else {
            Ok(None)
        }
    }
}
```

#### 5. Redis Repository 实现

```rust
/// 用户 Redis Repository
#[derive(Repository)]
pub struct UserRedisRepository {
    redis_template: Arc<RedisTemplate>,
}

#[async_trait]
impl RedisRepository<User, u64> for UserRedisRepository {
    async fn save(&self, user: &User, id: &u64) -> Result<()> {
        let key = format!("user:{}", id);
        self.redis_template.set(&key, user).await
    }
    
    async fn save_with_ttl(&self, user: &User, id: &u64, ttl: u64) -> Result<()> {
        let key = format!("user:{}", id);
        self.redis_template.set_ex(&key, user, ttl).await
    }
    
    async fn find_by_id(&self, id: &u64) -> Result<Option<User>> {
        let key = format!("user:{}", id);
        self.redis_template.get(&key).await
    }
    
    async fn exists_by_id(&self, id: &u64) -> Result<bool> {
        let key = format!("user:{}", id);
        self.redis_template.exists(&key).await
    }
    
    async fn delete_by_id(&self, id: &u64) -> Result<bool> {
        let key = format!("user:{}", id);
        self.redis_template.del(&key).await
    }
    
    async fn find_all_keys(&self) -> Result<Vec<String>> {
        // 注意：KEYS 命令在生产环境中应谨慎使用
        let pattern = "user:*";
        self.redis_template.keys(pattern).await
    }
    
    async fn save_all(&self, entities: Vec<(u64, User)>) -> Result<()> {
        // 使用 pipeline 提高性能
        let mut pipe = redis::pipe();
        
        for (id, user) in entities {
            let key = format!("user:{}", id);
            let serialized = self.redis_template.serializer.serialize(&user).await?;
            pipe.set(&key, serialized);
        }
        
        pipe.query_async(&mut *self.redis_template.get_connection().await?)
            .await?;
        
        Ok(())
    }
    
    async fn find_all_by_ids(&self, ids: Vec<u64>) -> Result<HashMap<String, User>> {
        let keys: Vec<String> = ids.iter().map(|id| format!("user:{}", id)).collect();
        
        if keys.is_empty() {
            return Ok(HashMap::new());
        }
        
        let values: Vec<Option<String>> = self.redis_template.mget(&keys).await?;
        let mut result = HashMap::new();
        
        for (key, value) in keys.into_iter().zip(values.into_iter()) {
            if let Some(data) = value {
                if let Ok(user) = self.redis_template.serializer.deserialize::<User>(data.as_bytes()).await {
                    result.insert(key, user);
                }
            }
        }
        
        Ok(result)
    }
}
```

## 🔧 配置参考

### 完整配置示例

```toml
# application.toml

[redis]
# 基础连接配置
url = "redis://[:password]@localhost:6379/0"
pool_size = 20
connect_timeout = 5000
command_timeout = 3000
retry_attempts = 3
retry_interval = 1000

# 序列化配置
serialization = "json"  # json, messagepack, bincode

# 缓存配置
default_ttl = 3600
enable_logging = true
log_slow_commands = true
slow_command_threshold = 100

# 连接池配置
[redis.pool]
min_idle = 5
max_idle = 10
max_open = 20
connection_lifetime = 3600

# 集群配置
[redis.cluster]
enabled = false
nodes = [
    "redis://node1.example.com:6379",
    "redis://node2.example.com:6379", 
    "redis://node3.example.com:6379"
]
read_from_replicas = true

# Sentinel 配置
[redis.sentinel]
enabled = false
service_name = "mymaster"
sentinels = [
    "redis://sentinel1.example.com:26379",
    "redis://sentinel2.example.com:26379",
    "redis://sentinel3.example.com:26379"
]
sentinel_password = "sentinel_pass"

# SSL/TLS 配置
[redis.tls]
enabled = false
cert_file = "client.crt"
key_file = "client.key"
ca_file = "ca.crt"
skip_verify = false

# 监控配置
[redis.monitoring]
enabled = true
metrics_interval = 30
export_connection_stats = true
export_command_stats = true
```

### 环境变量配置

```bash
# Redis 连接
REDIS_URL=redis://localhost:6379
REDIS_POOL_SIZE=20
REDIS_CONNECT_TIMEOUT=5000
REDIS_COMMAND_TIMEOUT=3000

# 序列化
REDIS_SERIALIZATION=json
REDIS_DEFAULT_TTL=3600

# 集群配置
REDIS_CLUSTER_ENABLED=false
REDIS_CLUSTER_NODES=redis://node1:6379,redis://node2:6379

# TLS 配置
REDIS_TLS_ENABLED=false
REDIS_TLS_CERT_FILE=client.crt
REDIS_TLS_KEY_FILE=client.key
```

## 🧪 测试支持

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum_boot_starter_data_redis::test::*;
    
    #[tokio::test]
    async fn test_redis_template() {
        // 创建测试 Redis 连接
        let redis_template = create_test_redis_template().await;
        
        // 测试基本操作
        let test_user = User {
            id: 1,
            name: "测试用户".to_string(),
            email: "test@example.com".to_string(),
        };
        
        // 设置值
        redis_template.set("test_user", &test_user).await.unwrap();
        
        // 获取值
        let cached_user: Option<User> = redis_template.get("test_user").await.unwrap();
        assert!(cached_user.is_some());
        assert_eq!(cached_user.unwrap().name, "测试用户");
        
        // 删除值
        let deleted = redis_template.del("test_user").await.unwrap();
        assert!(deleted);
        
        // 验证删除
        let deleted_user: Option<User> = redis_template.get("test_user").await.unwrap();
        assert!(deleted_user.is_none());
    }
    
    #[tokio::test]
    async fn test_hash_operations() {
        let hash_ops = create_test_hash_operations().await;
        
        // 测试 Hash 操作
        hash_ops.hset("test_hash", "field1", &"value1").await.unwrap();
        hash_ops.hset("test_hash", "field2", &"value2").await.unwrap();
        
        let value1: Option<String> = hash_ops.hget("test_hash", "field1").await.unwrap();
        assert_eq!(value1, Some("value1".to_string()));
        
        let all_values: HashMap<String, String> = hash_ops.hgetall("test_hash").await.unwrap();
        assert_eq!(all_values.len(), 2);
    }
}
```

### 集成测试

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use axum_boot_starter_data_redis::test::*;
    
    #[tokio::test]
    async fn test_cache_annotations() {
        let app = create_test_app()
            .await
            .with_service::<UserService>();
        
        let user_service = app.get_service::<UserService>();
        
        // 第一次调用，数据来自数据库
        let start = std::time::Instant::now();
        let user1 = user_service.get_user(1).await.unwrap();
        let first_call_time = start.elapsed();
        
        // 第二次调用，数据来自缓存
        let start = std::time::Instant::now();
        let user2 = user_service.get_user(1).await.unwrap();
        let second_call_time = start.elapsed();
        
        assert_eq!(user1, user2);
        // 缓存调用应该更快
        assert!(second_call_time < first_call_time);
    }
}
```

## 🎯 最佳实践

### 1. 键命名规范

```rust
/// ✅ 好的键命名规范
pub struct CacheKeyBuilder;

impl CacheKeyBuilder {
    /// 用户缓存键
    pub fn user_key(user_id: u64) -> String {
        format!("user:{}", user_id)
    }
    
    /// 用户会话键
    pub fn user_session_key(user_id: u64) -> String {
        format!("session:user:{}", user_id)
    }
    
    /// 分页数据键
    pub fn page_key(resource: &str, page: u64, size: u64) -> String {
        format!("page:{}:{}:{}", resource, page, size)
    }
    
    /// 分布式锁键
    pub fn lock_key(resource: &str) -> String {
        format!("lock:{}", resource)
    }
}

/// ❌ 避免的做法
// 键名不规范
"userdata"
"user_info_cache"
format!("user-{}-data", user_id)  // 混合分隔符
```

### 2. TTL 管理

```rust
/// ✅ 合理的 TTL 设置
pub struct CacheTTL;

impl CacheTTL {
    /// 用户基础信息（变化少，TTL 长）
    pub const USER_PROFILE: u64 = 3600 * 24; // 24小时
    
    /// 用户会话信息（安全相关，TTL 短）
    pub const USER_SESSION: u64 = 3600 * 2; // 2小时
    
    /// 热点数据（访问频繁，TTL 中等）
    pub const HOT_DATA: u64 = 3600; // 1小时
    
    /// 临时数据（如验证码）
    pub const TEMP_DATA: u64 = 300; // 5分钟
    
    /// 分布式锁
    pub const DISTRIBUTED_LOCK: u64 = 30; // 30秒
}

impl UserCacheService {
    pub async fn cache_user_profile(&self, user: &User) -> Result<()> {
        let key = CacheKeyBuilder::user_key(user.id);
        self.redis_template.set_ex(&key, user, CacheTTL::USER_PROFILE).await
    }
}
```

### 3. 错误处理和降级

```rust
/// ✅ 优雅的缓存降级
#[derive(Service)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
    redis_template: Arc<RedisTemplate>,
}

impl UserService {
    pub async fn get_user(&self, id: u64) -> Result<Option<User>> {
        let cache_key = CacheKeyBuilder::user_key(id);
        
        // 尝试从缓存获取
        match self.redis_template.get::<User>(&cache_key).await {
            Ok(Some(user)) => {
                tracing::debug!("用户 {} 从缓存获取", id);
                return Ok(Some(user));
            },
            Ok(None) => {
                tracing::debug!("用户 {} 缓存未命中", id);
            },
            Err(e) => {
                tracing::warn!("获取用户 {} 缓存失败，降级到数据库: {}", id, e);
                // 缓存失败时不影响业务逻辑
            }
        }
        
        // 从数据库获取
        let user = self.user_repository.find_by_id(id).await?;
        
        // 异步更新缓存（不阻塞主流程）
        if let Some(ref user) = user {
            let redis_template = self.redis_template.clone();
            let cache_key = cache_key.clone();
            let user_for_cache = user.clone();
            
            tokio::spawn(async move {
                if let Err(e) = redis_template.set_ex(&cache_key, &user_for_cache, CacheTTL::USER_PROFILE).await {
                    tracing::warn!("异步更新用户缓存失败: {}", e);
                }
            });
        }
        
        Ok(user)
    }
}
```

### 4. 批量操作优化

```rust
/// ✅ 高效的批量操作
impl UserService {
    /// 批量获取用户（优化版本）
    pub async fn get_users_batch(&self, user_ids: Vec<u64>) -> Result<HashMap<u64, User>> {
        if user_ids.is_empty() {
            return Ok(HashMap::new());
        }
        
        // 1. 批量从缓存获取
        let cache_keys: Vec<String> = user_ids.iter()
            .map(|id| CacheKeyBuilder::user_key(*id))
            .collect();
        
        let cached_results = match self.redis_template.mget::<User>(&cache_keys).await {
            Ok(results) => results,
            Err(e) => {
                tracing::warn!("批量获取缓存失败，降级到数据库: {}", e);
                vec![None; user_ids.len()]
            }
        };
        
        // 2. 收集缓存命中和未命中的 ID
        let mut result = HashMap::new();
        let mut missing_ids = Vec::new();
        
        for (i, user_id) in user_ids.iter().enumerate() {
            if let Some(user) = &cached_results[i] {
                result.insert(*user_id, user.clone());
            } else {
                missing_ids.push(*user_id);
            }
        }
        
        // 3. 从数据库获取未命中的数据
        if !missing_ids.is_empty() {
            let db_users = self.user_repository.find_by_ids(missing_ids).await?;
            
            // 4. 异步批量更新缓存
            if !db_users.is_empty() {
                let redis_template = self.redis_template.clone();
                let users_for_cache = db_users.clone();
                
                tokio::spawn(async move {
                    let mut pipe = redis::pipe();
                    
                    for user in users_for_cache {
                        let key = CacheKeyBuilder::user_key(user.id);
                        if let Ok(serialized) = serde_json::to_string(&user) {
                            pipe.set_ex(&key, serialized, CacheTTL::USER_PROFILE);
                        }
                    }
                    
                    if let Err(e) = pipe.query_async(&mut *redis_template.get_connection().await.unwrap()).await {
                        tracing::warn!("批量更新缓存失败: {}", e);
                    }
                });
            }
            
            // 5. 合并结果
            for user in db_users {
                result.insert(user.id, user);
            }
        }
        
        Ok(result)
    }
}
```

### 5. 分布式锁最佳实践

```rust
/// ✅ 安全的分布式锁实现
impl DistributedLockService {
    /// 执行需要互斥的业务操作
    pub async fn execute_exclusively<F, R>(
        &self, 
        resource: &str, 
        operation: F
    ) -> Result<Option<R>>
    where
        F: FnOnce() -> BoxFuture<'static, Result<R>>,
    {
        let lock_key = format!("lock:{}", resource);
        let lock_timeout = 30; // 30秒锁超时
        let acquire_timeout = 5; // 5秒获取超时
        
        // 尝试获取锁（带超时）
        let lock_value = match timeout(
            Duration::from_secs(acquire_timeout),
            self.acquire_lock_with_retry(&lock_key, lock_timeout)
        ).await {
            Ok(Ok(Some(value))) => value,
            Ok(Ok(None)) => {
                tracing::warn!("获取锁失败: {}", resource);
                return Ok(None);
            },
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                tracing::warn!("获取锁超时: {}", resource);
                return Ok(None);
            }
        };
        
        // 执行业务操作
        let result = match operation().await {
            Ok(r) => r,
            Err(e) => {
                // 确保释放锁
                let _ = self.release_lock(&lock_key, &lock_value).await;
                return Err(e);
            }
        };
        
        // 释放锁
        if let Err(e) = self.release_lock(&lock_key, &lock_value).await {
            tracing::warn!("释放锁失败: {}, error: {}", resource, e);
        }
        
        Ok(Some(result))
    }
    
    /// 带重试的锁获取
    async fn acquire_lock_with_retry(&self, key: &str, ttl: u64) -> Result<Option<String>> {
        const MAX_RETRIES: u32 = 3;
        const RETRY_INTERVAL: u64 = 100; // 100ms
        
        for attempt in 1..=MAX_RETRIES {
            match self.acquire_lock(key, ttl).await? {
                Some(value) => return Ok(Some(value)),
                None => {
                    if attempt < MAX_RETRIES {
                        tokio::time::sleep(Duration::from_millis(RETRY_INTERVAL)).await;
                    }
                }
            }
        }
        
        Ok(None)
    }
}
```

## 📋 常见问题

### Q: Redis 连接池满了怎么办？

A: 优化连接池配置和使用模式：

```toml
[redis.pool]
max_open = 50          # 增加最大连接数
max_idle = 10          # 设置合理的空闲连接数
connection_lifetime = 3600  # 设置连接生命周期
```

### Q: 如何处理 Redis 集群故障转移？

A: 使用 Sentinel 或集群配置：

```toml
[redis.sentinel]
enabled = true
service_name = "mymaster"
sentinels = ["redis://sentinel1:26379"]
```

### Q: 缓存穿透如何防护？

A: 使用布隆过滤器或空值缓存：

```rust
pub async fn get_user_safe(&self, id: u64) -> Result<Option<User>> {
    // 缓存空值防止穿透
    let cache_key = format!("user:{}", id);
    
    match self.redis_template.get::<String>(&cache_key).await? {
        Some(value) if value == "NULL" => return Ok(None),
        Some(_) => { /* 正常缓存值 */ },
        None => { /* 缓存未命中 */ }
    }
    
    let user = self.user_repository.find_by_id(id).await?;
    
    if user.is_none() {
        // 缓存空值
        self.redis_template.set_ex(&cache_key, &"NULL", 300).await?;
    }
    
    Ok(user)
}
```

## 🚀 下一步

- 📚 [核心概念](../guide/core-concepts.md) - 理解框架设计理念
- 🌐 [Web 开发](../guide/web-development.md) - 学习 Web API 开发
- 🗄️ [MySQL 数据库](axum-boot-starter-data-mysql.md) - 学习数据库操作
- 🔧 [配置系统](../guide/configuration.md) - 掌握配置管理