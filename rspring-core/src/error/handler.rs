//! 错误处理器模块
//! 
//! 提供统一的错误处理逻辑和错误响应格式化

use crate::error::types::{Error, Result};
use std::fmt;
use tracing::error;

/// 错误处理器
/// 
/// 提供统一的错误处理逻辑，包括日志记录、错误分类等功能
#[derive(Debug, Default)]
pub struct ErrorHandler;

impl ErrorHandler {
    /// 创建新的错误处理器实例
    pub fn new() -> Self {
        Self
    }
    
    /// 处理错误并记录日志
    /// 
    /// # 参数
    /// * `error` - 要处理的错误
    /// * `context` - 错误上下文信息（可选）
    /// 
    /// # 返回值
    /// 返回格式化后的错误响应
    pub fn handle_error(&self, error: &Error, context: Option<&str>) -> ErrorResponse {
        // 根据错误类型记录不同级别的日志
        match error {
            Error::Internal { message } => {
                error!(context = context, "内部错误: {}", message);
            }
            Error::Configuration(_) => {
                error!(context = context, "配置错误: {}", error);
            }
            Error::DependencyInjection { message } => {
                error!(context = context, "依赖注入错误: {}", message);
            }
            Error::Business { code, message } => {
                tracing::warn!(
                    context = context, 
                    error_code = code, 
                    "业务错误: {}", 
                    message
                );
            }
            Error::Validation { message } => {
                tracing::warn!(context = context, "验证错误: {}", message);
            }
            Error::NotFound { resource } => {
                tracing::warn!(context = context, "资源未找到: {}", resource);
            }
            _ => {
                error!(context = context, "未分类错误: {}", error);
            }
        }
        
        // 创建错误响应
        ErrorResponse::from_error(error)
    }
    
    /// 处理并返回结果
    /// 
    /// 如果是错误，会记录日志并返回格式化的错误响应
    pub fn handle_result<T>(&self, result: Result<T>, context: Option<&str>) -> std::result::Result<T, ErrorResponse> {
        match result {
            Ok(value) => Ok(value),
            Err(error) => Err(self.handle_error(&error, context)),
        }
    }
}

/// 标准化错误响应结构
/// 
/// 用于统一的错误响应格式，便于客户端处理
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorResponse {
    /// 错误码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 错误详情（可选）
    pub details: Option<String>,
    /// 时间戳
    pub timestamp: String,
}

impl ErrorResponse {
    /// 从错误创建响应
    pub fn from_error(error: &Error) -> Self {
        let (code, message, details) = match error {
            Error::Configuration(_) => (
                "CONFIG_ERROR".to_string(),
                "配置错误".to_string(),
                Some(error.to_string()),
            ),
            Error::Validation { message } => (
                "VALIDATION_ERROR".to_string(),
                message.clone(),
                None,
            ),
            Error::Business { code, message } => (
                code.clone(),
                message.clone(),
                None,
            ),
            Error::NotFound { resource } => (
                "NOT_FOUND".to_string(),
                format!("{}未找到", resource),
                None,
            ),
            Error::Unauthorized => (
                "UNAUTHORIZED".to_string(),
                "未授权访问".to_string(),
                None,
            ),
            Error::Container { message } => (
                "CONTAINER_ERROR".to_string(),
                "容器错误".to_string(),
                Some(message.clone()),
            ),
            Error::ComponentNotFound { component } => (
                "COMPONENT_NOT_FOUND".to_string(),
                format!("组件未找到: {}", component),
                None,
            ),
            Error::DependencyInjection { message } => (
                "DEPENDENCY_INJECTION_ERROR".to_string(),
                "依赖注入错误".to_string(),
                Some(message.clone()),
            ),
            _ => (
                "INTERNAL_ERROR".to_string(),
                "内部服务器错误".to_string(),
                Some(error.to_string()),
            ),
        };
        
        Self {
            code,
            message,
            details,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// 创建自定义错误响应
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// 添加详情信息
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)?;
        if let Some(ref details) = self.details {
            write!(f, " - {}", details)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试错误处理器
    #[test]
    fn test_error_handler() {
        let handler = ErrorHandler::new();
        let error = Error::validation("用户名不能为空");
        
        let response = handler.handle_error(&error, Some("用户注册"));
        
        assert_eq!(response.code, "VALIDATION_ERROR");
        assert_eq!(response.message, "用户名不能为空");
        assert!(response.details.is_none());
    }

    /// 测试错误响应创建
    #[test]
    fn test_error_response_creation() {
        let business_error = Error::business("USER_EXISTS", "用户已存在");
        let response = ErrorResponse::from_error(&business_error);
        
        assert_eq!(response.code, "USER_EXISTS");
        assert_eq!(response.message, "用户已存在");
        
        let not_found_error = Error::not_found("用户");
        let response = ErrorResponse::from_error(&not_found_error);
        
        assert_eq!(response.code, "NOT_FOUND");
        assert_eq!(response.message, "用户未找到");
    }

    /// 测试结果处理
    #[test]
    fn test_handle_result() {
        let handler = ErrorHandler::new();
        
        // 测试成功情况
        let success_result: Result<i32> = Ok(42);
        let handled = handler.handle_result(success_result, None);
        assert_eq!(handled.unwrap(), 42);
        
        // 测试错误情况
        let error_result: Result<i32> = Err(Error::validation("测试错误"));
        let handled = handler.handle_result(error_result, Some("测试上下文"));
        
        assert!(handled.is_err());
        let error_response = handled.unwrap_err();
        assert_eq!(error_response.code, "VALIDATION_ERROR");
    }
}