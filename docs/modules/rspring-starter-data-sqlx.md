# rspring-starter-data-sqlx

[![crates.io](https://img.shields.io/crates/v/rspring-starter-data-sqlx.svg)](https://crates.io/crates/rspring-starter-data-sqlx)
[![docs.rs](https://img.shields.io/docsrs/rspring-starter-data-sqlx)](https://docs.rs/rspring-starter-data-sqlx)

**rspring-starter-data-sqlx** 是 RSpring 框架的数据访问 Starter，集成 SQLx 库并提供数据源自动配置。支持通过 feature 选择启用的数据源，目前仅实现 MySQL 支持。

## 🎯 核心功能

- **SQLx 集成** - 无缝集成 SQLx 库，享受编译时检查的 SQL 操作
- **自动装配** - 自动配置数据源和连接池
- **Feature 选择** - 通过 Cargo feature 选择数据库类型（目前支持 MySQL）
- **连接池管理** - 智能的数据库连接池配置和管理
- **事务支持** - 简化的事务管理和声明式事务
- **配置简化** - 通过配置文件完成数据源设置
- 📈 **性能监控** - 内置的 SQL 性能分析
- 🛡️ **SQL 防注入** - 参数化查询保护

### 📦 快速开始

添加依赖到 `Cargo.toml`：

```toml
[dependencies]
axum-boot-starter-data-mysql = "0.1.0"
```

## 🏗️ 架构设计

### 核心组件架构

```
axum-boot-starter-data-mysql
├── config/
│   ├── database.rs        # 数据库配置
│   └── connection.rs      # 连接池管理
├── repository/
│   ├── annotations.rs     # Repository 注解
│   ├── crud.rs           # CRUD 操作接口
│   └── query.rs          # 查询构建器
├── transaction/
│   ├── manager.rs        # 事务管理器
│   └── annotations.rs    # 事务注解
├── migration/
│   ├── runner.rs         # 数据库迁移
│   └── schema.rs         # 表结构管理
└── lib.rs                # 模块导出
```

### 模块依赖关系

```mermaid
graph TD
    A[axum-boot-starter-data-mysql] --> B[axum-boot-core]
    A --> C[sqlx]
    A --> D[sqlx-mysql]
    A --> E[sea-query]
    A --> F[chrono]
    A --> G[uuid]
    
    B --> H[axum-boot-macro]
    B --> I[serde]
    B --> J[tokio]
```

## 📚 API 参考

### 核心类型

#### DatabaseConfig

数据库配置结构体：

```rust
/// MySQL 数据库配置
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库连接 URL
    /// 格式: mysql://username:password@host:port/database
    pub url: String,
    
    /// 连接池最大连接数
    pub max_connections: u32,
    
    /// 连接池最小连接数
    pub min_connections: u32,
    
    /// 连接超时时间（秒）
    pub connect_timeout: u64,
    
    /// 空闲连接超时时间（秒）
    pub idle_timeout: u64,
    
    /// 连接最大生存时间（秒）
    pub max_lifetime: u64,
    
    /// 是否启用 SQL 日志
    pub enable_logging: bool,
    
    /// 慢查询阈值（毫秒）
    pub slow_query_threshold: u64,
    
    /// 是否自动运行迁移
    pub auto_migrate: bool,
    
    /// 迁移文件目录
    pub migration_dir: String,
}
```

#### DatabasePool

数据库连接池：

```rust
/// MySQL 连接池包装器
#[derive(Debug, Clone)]
pub struct DatabasePool {
    pool: MySqlPool,
    config: DatabaseConfig,
}

impl DatabasePool {
    /// 创建新的连接池
    pub async fn new(config: DatabaseConfig) -> Result<Self>;
    
    /// 获取连接
    pub async fn acquire(&self) -> Result<PoolConnection<MySql>>;
    
    /// 开始事务
    pub async fn begin(&self) -> Result<Transaction<MySql>>;
    
    /// 执行查询
    pub async fn execute(&self, query: &str) -> Result<MySqlQueryResult>;
    
    /// 获取连接池统计信息
    pub fn stats(&self) -> PoolStats;
}
```

#### Entity

实体基础 trait：

```rust
/// 实体 trait
pub trait Entity: Send + Sync + 'static {
    /// 实体名称
    fn table_name() -> &'static str;
    
    /// 主键字段名
    fn primary_key() -> &'static str {
        "id"
    }
}

/// 实体派生宏
/// 
/// # 示例
/// ```rust
/// #[derive(Entity, sqlx::FromRow, Serialize, Deserialize)]
/// #[table_name = "users"]
/// pub struct User {
///     pub id: u64,
///     pub name: String,
///     pub email: String,
///     pub created_at: chrono::DateTime<chrono::Utc>,
/// }
/// ```
#[proc_macro_derive(Entity, attributes(table_name, primary_key))]
pub fn entity_derive(input: TokenStream) -> TokenStream;
```

### Repository 系统

#### CrudRepository

基础 CRUD 操作接口：

```rust
/// CRUD Repository trait
#[async_trait]
pub trait CrudRepository<T, ID>: Send + Sync
where
    T: Entity,
    ID: Send + Sync,
{
    /// 保存实体
    async fn save(&self, entity: &T) -> Result<T>;
    
    /// 根据 ID 查找
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    
    /// 查找所有实体
    async fn find_all(&self) -> Result<Vec<T>>;
    
    /// 检查实体是否存在
    async fn exists_by_id(&self, id: ID) -> Result<bool>;
    
    /// 根据 ID 删除
    async fn delete_by_id(&self, id: ID) -> Result<()>;
    
    /// 删除实体
    async fn delete(&self, entity: &T) -> Result<()>;
    
    /// 计算实体总数
    async fn count(&self) -> Result<i64>;
}
```

#### PagingRepository

分页查询接口：

```rust
/// 分页 Repository trait
#[async_trait]
pub trait PagingRepository<T>: CrudRepository<T, u64>
where
    T: Entity,
{
    /// 分页查询
    async fn find_all_paged(&self, page: u64, size: u64) -> Result<Page<T>>;
    
    /// 排序分页查询
    async fn find_all_sorted(&self, page: u64, size: u64, sort: Sort) -> Result<Page<T>>;
}

/// 排序配置
#[derive(Debug, Clone)]
pub struct Sort {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}
```

#### Repository 注解

```rust
/// Repository 组件注解
/// 
/// # 示例
/// ```rust
/// #[derive(Repository)]
/// pub struct UserRepository {
///     pool: Arc<DatabasePool>,
/// }
/// ```
#[proc_macro_derive(Repository)]
pub fn repository_derive(input: TokenStream) -> TokenStream;

/// 查询方法注解
/// 
/// # 示例
/// ```rust
/// impl UserRepository {
///     #[Query("SELECT * FROM users WHERE email = ?")]
///     async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
///     
///     #[Query("SELECT * FROM users WHERE name LIKE ?")]
///     async fn find_by_name_containing(&self, name: &str) -> Result<Vec<User>>;
/// }
/// ```
#[proc_macro_attribute]
pub fn Query(args: TokenStream, input: TokenStream) -> TokenStream;

/// 更新操作注解
/// 
/// # 示例
/// ```rust
/// #[Modifying]
/// #[Query("UPDATE users SET last_login = NOW() WHERE id = ?")]
/// async fn update_last_login(&self, user_id: u64) -> Result<u64>;
/// ```
#[proc_macro_attribute]
pub fn Modifying(args: TokenStream, input: TokenStream) -> TokenStream;
```

### 事务管理

#### Transaction 注解

```rust
/// 事务注解
/// 
/// # 参数
/// - `propagation`: 事务传播行为
/// - `isolation`: 事务隔离级别
/// - `read_only`: 是否只读事务
/// - `timeout`: 事务超时时间
/// 
/// # 示例
/// ```rust
/// impl UserService {
///     #[Transactional]
///     async fn create_user_with_profile(&self, request: CreateUserRequest) -> Result<User>;
///     
///     #[Transactional(read_only = true)]
///     async fn get_user_stats(&self) -> Result<UserStats>;
/// }
/// ```
#[proc_macro_attribute]
pub fn Transactional(args: TokenStream, input: TokenStream) -> TokenStream;
```

#### TransactionManager

```rust
/// 事务管理器
pub struct TransactionManager {
    pool: Arc<DatabasePool>,
}

impl TransactionManager {
    /// 执行事务
    pub async fn execute_in_transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Transaction<MySql>) -> BoxFuture<'_, Result<R>>,
    {
        let mut tx = self.pool.begin().await?;
        
        match f(&mut tx).await {
            Ok(result) => {
                tx.commit().await?;
                Ok(result)
            },
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}
```

### 查询构建器

#### QueryBuilder

类型安全的查询构建：

```rust
/// 查询构建器
pub struct QueryBuilder<T> 
where 
    T: Entity,
{
    table: String,
    conditions: Vec<Condition>,
    order_by: Vec<OrderBy>,
    limit: Option<u64>,
    offset: Option<u64>,
}

impl<T> QueryBuilder<T> 
where 
    T: Entity,
{
    /// 创建查询构建器
    pub fn new() -> Self;
    
    /// 添加 WHERE 条件
    pub fn where_eq<V>(self, field: &str, value: V) -> Self
    where 
        V: Send + Sync + 'static;
    
    /// 添加 LIKE 条件
    pub fn where_like(self, field: &str, pattern: &str) -> Self;
    
    /// 添加 IN 条件
    pub fn where_in<V>(self, field: &str, values: Vec<V>) -> Self
    where 
        V: Send + Sync + 'static;
    
    /// 添加排序
    pub fn order_by(self, field: &str, direction: SortDirection) -> Self;
    
    /// 设置限制
    pub fn limit(self, limit: u64) -> Self;
    
    /// 设置偏移
    pub fn offset(self, offset: u64) -> Self;
    
    /// 构建查询
    pub fn build(&self) -> (String, Vec<Box<dyn Send + Sync>>);
    
    /// 执行查询
    pub async fn fetch_all(&self, pool: &DatabasePool) -> Result<Vec<T>>;
    
    /// 执行分页查询
    pub async fn fetch_page(&self, pool: &DatabasePool, page: u64, size: u64) -> Result<Page<T>>;
}
```

### 数据库迁移

#### Migration

数据库迁移系统：

```rust
/// 迁移接口
#[async_trait]
pub trait Migration: Send + Sync {
    /// 迁移名称
    fn name(&self) -> &str;
    
    /// 迁移版本
    fn version(&self) -> i64;
    
    /// 执行迁移
    async fn up(&self, pool: &DatabasePool) -> Result<()>;
    
    /// 回滚迁移
    async fn down(&self, pool: &DatabasePool) -> Result<()>;
}

/// 迁移管理器
pub struct MigrationManager {
    pool: Arc<DatabasePool>,
    migrations: Vec<Box<dyn Migration>>,
}

impl MigrationManager {
    /// 运行所有待执行的迁移
    pub async fn migrate(&self) -> Result<()>;
    
    /// 回滚到指定版本
    pub async fn rollback_to(&self, version: i64) -> Result<()>;
    
    /// 获取迁移状态
    pub async fn status(&self) -> Result<Vec<MigrationStatus>>;
}
```

## 🚀 使用指南

### 基础使用

#### 1. 配置数据库

```toml
# application.toml
[database]
url = "mysql://root:password@localhost:3306/myapp"
max_connections = 10
min_connections = 2
connect_timeout = 30
idle_timeout = 300
max_lifetime = 3600
enable_logging = true
slow_query_threshold = 1000
auto_migrate = true
migration_dir = "migrations"
```

#### 2. 定义实体

```rust
use axum_boot_starter_data_mysql::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 用户实体
#[derive(Entity, sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户资料实体
#[derive(Entity, sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user_profiles"]
pub struct UserProfile {
    pub id: u64,
    pub user_id: u64,
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

#### 3. 创建 Repository

```rust
/// 用户 Repository
#[derive(Repository)]
pub struct UserRepository {
    pool: Arc<DatabasePool>,
}

#[async_trait]
impl CrudRepository<User, u64> for UserRepository {
    async fn save(&self, user: &User) -> Result<User> {
        let query = if user.id == 0 {
            // 插入新用户
            sqlx::query_as!(
                User,
                r#"
                INSERT INTO users (name, email, password_hash, created_at, updated_at) 
                VALUES (?, ?, ?, NOW(), NOW())
                "#,
                user.name,
                user.email,
                user.password_hash
            )
        } else {
            // 更新现有用户
            sqlx::query_as!(
                User,
                r#"
                UPDATE users 
                SET name = ?, email = ?, password_hash = ?, updated_at = NOW() 
                WHERE id = ?
                "#,
                user.name,
                user.email,
                user.password_hash,
                user.id
            )
        };
        
        query.fetch_one(&*self.pool.pool).await.map_err(Error::Database)
    }
    
    async fn find_by_id(&self, id: u64) -> Result<Option<User>> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_optional(&*self.pool.pool)
            .await
            .map_err(Error::Database)
    }
    
    async fn find_all(&self) -> Result<Vec<User>> {
        sqlx::query_as!(User, "SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&*self.pool.pool)
            .await
            .map_err(Error::Database)
    }
    
    async fn exists_by_id(&self, id: u64) -> Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&*self.pool.pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(count.0 > 0)
    }
    
    async fn delete_by_id(&self, id: u64) -> Result<()> {
        sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&*self.pool.pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(())
    }
    
    async fn delete(&self, user: &User) -> Result<()> {
        self.delete_by_id(user.id).await
    }
    
    async fn count(&self) -> Result<i64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
            .fetch_one(&*self.pool.pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(count.0)
    }
}

/// 扩展查询方法
impl UserRepository {
    /// 根据邮箱查找用户
    #[Query("SELECT * FROM users WHERE email = ?")]
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_optional(&*self.pool.pool)
            .await
            .map_err(Error::Database)
    }
    
    /// 根据名称模糊查找用户
    #[Query("SELECT * FROM users WHERE name LIKE ?")]
    pub async fn find_by_name_containing(&self, name: &str) -> Result<Vec<User>> {
        let pattern = format!("%{}%", name);
        sqlx::query_as!(User, "SELECT * FROM users WHERE name LIKE ?", pattern)
            .fetch_all(&*self.pool.pool)
            .await
            .map_err(Error::Database)
    }
    
    /// 更新最后登录时间
    #[Modifying]
    #[Query("UPDATE users SET updated_at = NOW() WHERE id = ?")]
    pub async fn update_last_login(&self, user_id: u64) -> Result<u64> {
        let result = sqlx::query!("UPDATE users SET updated_at = NOW() WHERE id = ?", user_id)
            .execute(&*self.pool.pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(result.rows_affected())
    }
}
```

#### 4. 使用事务

```rust
/// 用户服务（使用事务）
#[derive(Service)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
    user_profile_repository: Arc<UserProfileRepository>,
    transaction_manager: Arc<TransactionManager>,
}

impl UserService {
    /// 创建用户和资料（事务）
    #[Transactional]
    pub async fn create_user_with_profile(
        &self,
        request: CreateUserRequest,
    ) -> Result<UserWithProfile> {
        // 这个方法会自动在事务中执行
        
        // 1. 创建用户
        let user = User {
            id: 0,
            name: request.name.clone(),
            email: request.email.clone(),
            password_hash: hash_password(&request.password)?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let saved_user = self.user_repository.save(&user).await?;
        
        // 2. 创建用户资料
        let profile = UserProfile {
            id: 0,
            user_id: saved_user.id,
            first_name: request.first_name,
            last_name: request.last_name,
            bio: request.bio,
            avatar_url: None,
            created_at: Utc::now(),
        };
        
        let saved_profile = self.user_profile_repository.save(&profile).await?;
        
        Ok(UserWithProfile {
            user: saved_user,
            profile: saved_profile,
        })
    }
    
    /// 手动事务控制
    pub async fn complex_user_operation(&self, request: ComplexRequest) -> Result<()> {
        self.transaction_manager.execute_in_transaction(|tx| Box::pin(async move {
            // 在事务中执行多个操作
            
            // 操作 1: 更新用户信息
            sqlx::query!("UPDATE users SET name = ? WHERE id = ?", request.name, request.user_id)
                .execute(tx)
                .await?;
            
            // 操作 2: 插入操作日志
            sqlx::query!(
                "INSERT INTO user_logs (user_id, operation, created_at) VALUES (?, ?, NOW())",
                request.user_id,
                "update_profile"
            )
            .execute(tx)
            .await?;
            
            // 操作 3: 更新统计信息
            sqlx::query!("UPDATE user_stats SET update_count = update_count + 1 WHERE user_id = ?", request.user_id)
                .execute(tx)
                .await?;
            
            Ok(())
        })).await
    }
}
```

### 高级功能

#### 1. 查询构建器

```rust
impl UserService {
    /// 使用查询构建器进行复杂查询
    pub async fn search_users(&self, criteria: SearchCriteria) -> Result<Page<User>> {
        let mut query = QueryBuilder::<User>::new();
        
        // 动态添加条件
        if let Some(name) = &criteria.name {
            query = query.where_like("name", &format!("%{}%", name));
        }
        
        if let Some(email) = &criteria.email {
            query = query.where_eq("email", email);
        }
        
        if !criteria.ids.is_empty() {
            query = query.where_in("id", criteria.ids.clone());
        }
        
        // 添加排序
        query = query.order_by("created_at", SortDirection::Desc);
        
        // 执行分页查询
        query.fetch_page(&self.pool, criteria.page, criteria.size).await
    }
}
```

#### 2. 批量操作

```rust
impl UserRepository {
    /// 批量插入用户
    pub async fn save_batch(&self, users: Vec<User>) -> Result<Vec<User>> {
        let mut tx = self.pool.begin().await?;
        let mut saved_users = Vec::new();
        
        for user in users {
            let saved_user = sqlx::query_as!(
                User,
                "INSERT INTO users (name, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, NOW(), NOW())",
                user.name,
                user.email,
                user.password_hash
            )
            .fetch_one(&mut tx)
            .await
            .map_err(Error::Database)?;
            
            saved_users.push(saved_user);
        }
        
        tx.commit().await.map_err(Error::Database)?;
        Ok(saved_users)
    }
    
    /// 批量更新
    pub async fn update_batch(&self, updates: Vec<UserUpdate>) -> Result<u64> {
        let mut tx = self.pool.begin().await?;
        let mut total_affected = 0u64;
        
        for update in updates {
            let result = sqlx::query!(
                "UPDATE users SET name = ?, email = ?, updated_at = NOW() WHERE id = ?",
                update.name,
                update.email,
                update.id
            )
            .execute(&mut tx)
            .await
            .map_err(Error::Database)?;
            
            total_affected += result.rows_affected();
        }
        
        tx.commit().await.map_err(Error::Database)?;
        Ok(total_affected)
    }
}
```

#### 3. 数据库迁移

```rust
/// 创建用户表迁移
pub struct CreateUsersTable;

#[async_trait]
impl Migration for CreateUsersTable {
    fn name(&self) -> &str {
        "create_users_table"
    }
    
    fn version(&self) -> i64 {
        20240301000001
    }
    
    async fn up(&self, pool: &DatabasePool) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE users (
                id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL,
                INDEX idx_users_email (email),
                INDEX idx_users_created_at (created_at)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci
            "#
        )
        .execute(&*pool.pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
    
    async fn down(&self, pool: &DatabasePool) -> Result<()> {
        sqlx::query!("DROP TABLE users")
            .execute(&*pool.pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(())
    }
}

/// 添加用户资料表迁移
pub struct CreateUserProfilesTable;

#[async_trait]
impl Migration for CreateUserProfilesTable {
    fn name(&self) -> &str {
        "create_user_profiles_table"
    }
    
    fn version(&self) -> i64 {
        20240301000002
    }
    
    async fn up(&self, pool: &DatabasePool) -> Result<()> {
        sqlx::query!(
            r#"
            CREATE TABLE user_profiles (
                id BIGINT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
                user_id BIGINT UNSIGNED NOT NULL,
                first_name VARCHAR(100) NOT NULL,
                last_name VARCHAR(100) NOT NULL,
                bio TEXT,
                avatar_url VARCHAR(255),
                created_at DATETIME NOT NULL,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
                INDEX idx_user_profiles_user_id (user_id)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci
            "#
        )
        .execute(&*pool.pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
    
    async fn down(&self, pool: &DatabasePool) -> Result<()> {
        sqlx::query!("DROP TABLE user_profiles")
            .execute(&*pool.pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(())
    }
}
```

## 🔧 配置参考

### 完整配置示例

```toml
# application.toml

[database]
# 连接配置
url = "mysql://root:password@localhost:3306/myapp"
max_connections = 20
min_connections = 5
connect_timeout = 30
idle_timeout = 300
max_lifetime = 3600

# 日志配置
enable_logging = true
log_level = "info"
slow_query_threshold = 1000

# 迁移配置
auto_migrate = true
migration_dir = "migrations"
migration_table = "_migrations"

# SSL 配置
ssl_mode = "preferred"  # disabled, preferred, required
ssl_ca = "ca-cert.pem"
ssl_cert = "client-cert.pem"
ssl_key = "client-key.pem"

# 字符集配置
charset = "utf8mb4"
collation = "utf8mb4_unicode_ci"

# 时区配置
timezone = "UTC"

# 连接池监控
pool_monitoring = true
pool_metrics_interval = 30

# 只读副本配置（可选）
[database.read_replica]
enabled = true
url = "mysql://readonly:password@replica.example.com:3306/myapp"
max_connections = 10
```

### 环境变量配置

```bash
# 数据库配置
DATABASE_URL=mysql://root:password@localhost:3306/myapp
DATABASE_MAX_CONNECTIONS=20
DATABASE_MIN_CONNECTIONS=5
DATABASE_CONNECT_TIMEOUT=30

# SSL 配置
DATABASE_SSL_MODE=preferred
DATABASE_SSL_CA=ca-cert.pem
DATABASE_SSL_CERT=client-cert.pem
DATABASE_SSL_KEY=client-key.pem

# 迁移配置
DATABASE_AUTO_MIGRATE=true
DATABASE_MIGRATION_DIR=migrations
```

## 🧪 测试支持

### Repository 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum_boot_starter_data_mysql::test::*;
    
    #[tokio::test]
    async fn test_user_repository() {
        // 创建测试数据库池
        let pool = create_test_pool().await;
        let repository = UserRepository::new(pool);
        
        // 测试保存用户
        let user = User {
            id: 0,
            name: "测试用户".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed_password".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let saved_user = repository.save(&user).await.unwrap();
        assert!(saved_user.id > 0);
        assert_eq!(saved_user.name, "测试用户");
        
        // 测试查找用户
        let found_user = repository.find_by_id(saved_user.id).await.unwrap();
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().email, "test@example.com");
        
        // 测试删除用户
        repository.delete_by_id(saved_user.id).await.unwrap();
        let deleted_user = repository.find_by_id(saved_user.id).await.unwrap();
        assert!(deleted_user.is_none());
    }
}
```

### 事务测试

```rust
#[tokio::test]
async fn test_transaction_rollback() {
    let pool = create_test_pool().await;
    let transaction_manager = TransactionManager::new(pool.clone());
    
    // 测试事务回滚
    let result = transaction_manager.execute_in_transaction(|tx| Box::pin(async move {
        // 插入一个用户
        sqlx::query!("INSERT INTO users (name, email, password_hash, created_at, updated_at) VALUES (?, ?, ?, NOW(), NOW())",
            "事务测试用户",
            "transaction@example.com",
            "hashed"
        )
        .execute(tx)
        .await?;
        
        // 故意抛出错误以测试回滚
        Err::<(), Error>(Error::business("TEST_ERROR", "测试错误"))
    })).await;
    
    assert!(result.is_err());
    
    // 验证用户未被插入（事务已回滚）
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = ?")
        .bind("transaction@example.com")
        .fetch_one(&*pool.pool)
        .await
        .unwrap();
    
    assert_eq!(count.0, 0);
}
```

## 🎯 最佳实践

### 1. 实体设计

```rust
/// ✅ 好的实体设计
#[derive(Entity, sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: u64,                              // 主键
    pub name: String,                         // 业务字段
    pub email: String,
    pub password_hash: String,                // 敏感数据已加密
    pub status: UserStatus,                   // 使用枚举
    pub created_at: DateTime<Utc>,            // 审计字段
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
}
```

### 2. Repository 设计

```rust
/// ✅ 职责清晰的 Repository
#[derive(Repository)]
pub struct UserRepository {
    pool: Arc<DatabasePool>,  // 只依赖数据库连接池
}

impl UserRepository {
    /// 基础 CRUD 操作
    pub async fn save(&self, user: &User) -> Result<User> { /* ... */ }
    pub async fn find_by_id(&self, id: u64) -> Result<Option<User>> { /* ... */ }
    
    /// 业务相关的查询方法
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> { /* ... */ }
    pub async fn find_active_users(&self) -> Result<Vec<User>> { /* ... */ }
    
    /// 批量操作
    pub async fn find_by_ids(&self, ids: Vec<u64>) -> Result<Vec<User>> { /* ... */ }
}

/// ❌ 避免在 Repository 中包含业务逻辑
impl UserRepository {
    // ❌ 不要在 Repository 中验证业务规则
    pub async fn create_user_if_email_unique(&self, user: User) -> Result<User> {
        // 业务逻辑应该在 Service 层
    }
}
```

### 3. 事务管理

```rust
/// ✅ 合理的事务边界
#[derive(Service)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
    transaction_manager: Arc<TransactionManager>,
}

impl UserService {
    /// 事务边界应该包含完整的业务操作
    #[Transactional]
    pub async fn create_user_with_welcome_email(&self, request: CreateUserRequest) -> Result<User> {
        // 1. 创建用户（数据库操作）
        let user = self.user_repository.save(&User::from(request)).await?;
        
        // 2. 发送欢迎邮件（外部服务调用）
        self.email_service.send_welcome_email(&user.email).await?;
        
        // 3. 记录用户注册事件（数据库操作）
        self.event_repository.save(&UserRegisteredEvent::from(&user)).await?;
        
        Ok(user)
    }
    
    /// ✅ 只读操作使用只读事务
    #[Transactional(read_only = true)]
    pub async fn get_user_with_stats(&self, id: u64) -> Result<UserWithStats> {
        let user = self.user_repository.find_by_id(id).await?;
        let stats = self.user_stats_repository.find_by_user_id(id).await?;
        
        Ok(UserWithStats { user, stats })
    }
}
```

### 4. 性能优化

```rust
/// ✅ 批量操作优化
impl UserRepository {
    /// 使用 IN 查询而不是循环查询
    pub async fn find_by_ids(&self, ids: Vec<u64>) -> Result<Vec<User>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!("SELECT * FROM users WHERE id IN ({})", placeholders);
        
        let mut query = sqlx::query_as::<_, User>(&sql);
        for id in ids {
            query = query.bind(id);
        }
        
        query.fetch_all(&*self.pool.pool).await.map_err(Error::Database)
    }
    
    /// 分页查询优化
    pub async fn find_users_paged(&self, page: u64, size: u64) -> Result<Page<User>> {
        let offset = (page - 1) * size;
        
        // 并行执行数据查询和计数查询
        let (users_future, count_future) = tokio::join!(
            sqlx::query_as!(User, "SELECT * FROM users ORDER BY created_at DESC LIMIT ? OFFSET ?", size, offset)
                .fetch_all(&*self.pool.pool),
            sqlx::query!("SELECT COUNT(*) as count FROM users")
                .fetch_one(&*self.pool.pool)
        );
        
        let users = users_future.map_err(Error::Database)?;
        let count_result = count_future.map_err(Error::Database)?;
        
        Ok(Page::new(users, count_result.count as u64, page, size))
    }
}
```

## 📋 常见问题

### Q: 如何处理数据库连接池满的情况？

A: 配置合适的连接池参数并监控连接使用：

```toml
[database]
max_connections = 20
min_connections = 5
connect_timeout = 30
idle_timeout = 300
```

### Q: 如何实现读写分离？

A: 配置读副本：

```toml
[database.read_replica]
enabled = true
url = "mysql://readonly:password@replica.example.com:3306/myapp"
```

### Q: 如何处理数据库迁移？

A: 实现 Migration trait 并配置自动迁移：

```rust
#[async_trait]
impl Migration for CreateUsersTable {
    async fn up(&self, pool: &DatabasePool) -> Result<()> {
        // 迁移逻辑
    }
}
```

### Q: 如何优化大数据量查询？

A: 使用流式查询和分页：

```rust
pub async fn stream_users(&self) -> impl Stream<Item = Result<User>> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch(&*self.pool.pool)
        .map_err(Error::Database)
}
```

## 🚀 下一步

- 📚 [核心概念](../guide/core-concepts.md) - 理解框架设计理念
- 🌐 [Web 开发](../guide/web-development.md) - 学习 Web API 开发
- 🔧 [配置系统](../guide/configuration.md) - 掌握配置管理
- 📊 [Redis 缓存](axum-boot-starter-data-redis.md) - 学习缓存操作