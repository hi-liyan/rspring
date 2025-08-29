use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("配置错误: {0}")]
    Configuration(#[from] config::ConfigError),
    
    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("YAML序列化错误: {0}")]
    YamlSerialization(#[from] serde_yaml::Error),
    
    #[error("TOML序列化错误: {0}")]
    TomlSerialization(#[from] toml::de::Error),
    
    #[error("容器错误: {message}")]
    Container { message: String },
    
    #[error("组件未找到: {component}")]
    ComponentNotFound { component: String },
    
    #[error("依赖注入错误: {message}")]
    DependencyInjection { message: String },
    
    #[error("验证错误: {message}")]
    Validation { message: String },
    
    #[error("业务错误: {message} (错误码: {code})")]
    Business { message: String, code: String },
    
    #[error("资源未找到: {resource}")]
    ResourceNotFound { resource: String },
    
    #[error("应用程序错误: {message}")]
    Application { message: String },
    
    #[error("运行时错误: {message}")]
    Runtime { message: String },
}

impl Error {
    /// 创建容器错误
    pub fn container(message: impl Into<String>) -> Self {
        Self::Container { message: message.into() }
    }
    
    /// 创建组件未找到错误
    pub fn component_not_found(component: impl Into<String>) -> Self {
        Self::ComponentNotFound { component: component.into() }
    }
    
    /// 创建依赖注入错误
    pub fn dependency_injection(message: impl Into<String>) -> Self {
        Self::DependencyInjection { message: message.into() }
    }
    
    /// 创建验证错误
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation { message: message.into() }
    }
    
    /// 创建业务错误
    pub fn business(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Business { 
            code: code.into(), 
            message: message.into() 
        }
    }
    
    /// 创建资源未找到错误
    pub fn resource_not_found(resource: impl Into<String>) -> Self {
        Self::ResourceNotFound { resource: resource.into() }
    }
    
    /// 创建应用程序错误
    pub fn application(message: impl Into<String>) -> Self {
        Self::Application { message: message.into() }
    }
    
    /// 创建运行时错误
    pub fn runtime(message: impl Into<String>) -> Self {
        Self::Runtime { message: message.into() }
    }
}