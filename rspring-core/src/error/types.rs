//! 错误类型定义模块
//! 
//! 定义了应用程序中使用的所有错误类型，支持统一的错误处理机制

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// 统一错误类型
/// 
/// 包含所有可能发生的错误类型，便于统一处理和分类
#[derive(Error, Debug)]
pub enum Error {
    /// 配置相关错误
    #[error("配置错误: {0}")]
    Configuration(#[from] config::ConfigError),
    
    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    /// 序列化错误 - JSON
    #[error("JSON 序列化错误: {0}")]
    JsonSerialization(#[from] serde_json::Error),
    
    /// 序列化错误 - YAML
    #[error("YAML 序列化错误: {0}")]
    YamlSerialization(#[from] serde_yaml::Error),
    
    /// 序列化错误 - TOML
    #[error("TOML 序列化错误: {0}")]
    TomlSerialization(#[from] toml::de::Error),
    
    /// 容器相关错误
    #[error("容器错误: {message}")]
    Container { message: String },
    
    /// 组件未找到错误
    #[error("组件未找到: {component}")]
    ComponentNotFound { component: String },
    
    /// 依赖注入错误
    #[error("依赖注入错误: {message}")]
    DependencyInjection { message: String },
    
    /// 验证错误
    #[error("验证错误: {message}")]
    Validation { message: String },
    
    /// 业务错误
    #[error("业务错误: {message} (错误码: {code})")]
    Business { message: String, code: String },
    
    /// 资源未找到
    #[error("资源未找到: {resource}")]
    NotFound { resource: String },
    
    /// 未授权访问
    #[error("未授权访问")]
    Unauthorized,
    
    /// 内部服务器错误
    #[error("内部服务器错误: {message}")]
    Internal { message: String },
    
    /// 应用程序错误
    #[error("应用程序错误: {message}")]
    Application { message: String },
    
    /// 运行时错误
    #[error("运行时错误: {message}")]
    Runtime { message: String },
}

impl Error {
    /// 创建容器错误
    /// 
    /// # 示例
    /// ```rust
    /// let error = Error::container("组件注册失败");
    /// ```
    pub fn container(message: impl Into<String>) -> Self {
        Self::Container { 
            message: message.into() 
        }
    }
    
    /// 创建组件未找到错误
    /// 
    /// # 参数
    /// * `component` - 组件名称或类型
    pub fn component_not_found(component: impl Into<String>) -> Self {
        Self::ComponentNotFound { 
            component: component.into() 
        }
    }
    
    /// 创建依赖注入错误
    /// 
    /// # 示例
    /// ```rust
    /// let error = Error::dependency_injection("循环依赖检测失败");
    /// ```
    pub fn dependency_injection(message: impl Into<String>) -> Self {
        Self::DependencyInjection { 
            message: message.into() 
        }
    }
    
    /// 创建验证错误
    /// 
    /// # 示例
    /// ```rust
    /// if username.is_empty() {
    ///     return Err(Error::validation("用户名不能为空"));
    /// }
    /// ```
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation { 
            message: message.into() 
        }
    }
    
    /// 创建业务错误
    /// 
    /// # 参数
    /// * `code` - 错误码，用于客户端判断
    /// * `message` - 错误描述
    /// 
    /// # 示例
    /// ```rust
    /// if user_exists {
    ///     return Err(Error::business("USER_EXISTS", "用户已存在"));
    /// }
    /// ```
    pub fn business(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Business { 
            code: code.into(), 
            message: message.into() 
        }
    }
    
    /// 创建未找到错误
    /// 
    /// # 示例
    /// ```rust
    /// let error = Error::not_found("用户");
    /// ```
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound { 
            resource: resource.into() 
        }
    }
    
    /// 创建内部错误
    /// 
    /// # 示例
    /// ```rust
    /// let error = Error::internal("数据库连接失败");
    /// ```
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal { 
            message: message.into() 
        }
    }
    
    /// 创建应用程序错误
    pub fn application(message: impl Into<String>) -> Self {
        Self::Application { 
            message: message.into() 
        }
    }
    
    /// 创建运行时错误
    pub fn runtime(message: impl Into<String>) -> Self {
        Self::Runtime { 
            message: message.into() 
        }
    }
    
    /// 检查是否为业务错误
    pub fn is_business_error(&self) -> bool {
        matches!(self, Self::Business { .. })
    }
    
    /// 检查是否为验证错误
    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::Validation { .. })
    }
    
    /// 获取错误码（如果是业务错误）
    pub fn error_code(&self) -> Option<&str> {
        match self {
            Self::Business { code, .. } => Some(code),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试错误创建方法
    #[test]
    fn test_error_creation() {
        let validation_error = Error::validation("测试验证错误");
        assert!(validation_error.is_validation_error());
        
        let business_error = Error::business("TEST_001", "测试业务错误");
        assert!(business_error.is_business_error());
        assert_eq!(business_error.error_code(), Some("TEST_001"));
        
        let not_found_error = Error::not_found("用户");
        assert!(matches!(not_found_error, Error::NotFound { .. }));
    }

    /// 测试错误消息格式
    #[test]
    fn test_error_display() {
        let error = Error::business("USER_EXISTS", "用户已存在");
        let error_message = error.to_string();
        
        assert!(error_message.contains("用户已存在"));
        assert!(error_message.contains("USER_EXISTS"));
    }
}