//! 错误处理模块
//! 
//! 提供统一的错误类型定义和处理机制，支持类型安全的错误处理

pub mod types;
pub mod handler;

// 重新导出常用类型和函数
pub use types::{Error, Result};
pub use handler::{ErrorHandler, ErrorResponse};

/// 全局错误处理器实例
/// 
/// 提供便捷的全局错误处理功能
pub static ERROR_HANDLER: once_cell::sync::Lazy<ErrorHandler> = 
    once_cell::sync::Lazy::new(|| ErrorHandler::new());

/// 便捷的错误处理函数
/// 
/// # 示例
/// ```rust
/// use rspring_core::error::handle_error;
/// 
/// let error = Error::validation("输入无效");
/// let response = handle_error(&error, Some("用户验证"));
/// ```
pub fn handle_error(error: &Error, context: Option<&str>) -> ErrorResponse {
    ERROR_HANDLER.handle_error(error, context)
}

/// 便捷的结果处理函数
/// 
/// # 示例
/// ```rust
/// use rspring_core::error::handle_result;
/// 
/// let result: Result<String> = Ok("success".to_string());
/// let handled = handle_result(result, Some("操作上下文"));
/// ```
pub fn handle_result<T>(result: Result<T>, context: Option<&str>) -> std::result::Result<T, ErrorResponse> {
    ERROR_HANDLER.handle_result(result, context)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试全局错误处理
    #[test]
    fn test_global_error_handling() {
        let error = Error::business("TEST_ERROR", "测试错误");
        let response = handle_error(&error, Some("全局测试"));
        
        assert_eq!(response.code, "TEST_ERROR");
        assert_eq!(response.message, "测试错误");
    }

    /// 测试全局结果处理
    #[test] 
    fn test_global_result_handling() {
        let success_result: Result<String> = Ok("success".to_string());
        let handled = handle_result(success_result, None);
        
        assert!(handled.is_ok());
        assert_eq!(handled.unwrap(), "success");
    }
}