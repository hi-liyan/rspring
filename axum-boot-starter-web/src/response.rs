use serde::{Deserialize, Serialize};

/// API 统一响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 响应代码
    pub code: i32,
    /// 响应消息
    pub message: String,
    /// 响应数据
    pub data: Option<T>,
    /// 响应时间戳
    pub timestamp: i64,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    /// 创建成功响应（无数据）
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse {
            code: 200,
            message: "success".to_string(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    /// 创建错误响应
    pub fn error(code: i32, message: impl Into<String>) -> ApiResponse<()> {
        ApiResponse {
            code,
            message: message.into(),
            data: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
    
    /// 创建业务错误响应
    pub fn business_error(message: impl Into<String>) -> ApiResponse<()> {
        Self::error(400, message)
    }
    
    /// 创建验证错误响应
    pub fn validation_error(message: impl Into<String>) -> ApiResponse<()> {
        Self::error(400, message)
    }
    
    /// 创建未授权错误响应
    pub fn unauthorized() -> ApiResponse<()> {
        Self::error(401, "未授权访问")
    }
    
    /// 创建禁止访问错误响应
    pub fn forbidden() -> ApiResponse<()> {
        Self::error(403, "禁止访问")
    }
    
    /// 创建资源未找到错误响应
    pub fn not_found(resource: impl Into<String>) -> ApiResponse<()> {
        Self::error(404, format!("资源未找到: {}", resource.into()))
    }
    
    /// 创建内部服务器错误响应
    pub fn internal_error() -> ApiResponse<()> {
        Self::error(500, "内部服务器错误")
    }
}

/// 分页参数
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    /// 页码（从0开始）
    pub page: u64,
    /// 每页大小
    pub size: u64,
}

impl Default for Page {
    fn default() -> Self {
        Self { page: 0, size: 20 }
    }
}

impl Page {
    /// 创建分页参数
    pub fn new(page: u64, size: u64) -> Self {
        Self { page, size }
    }
    
    /// 计算偏移量
    pub fn offset(&self) -> u64 {
        self.page * self.size
    }
}

/// 分页结果
#[derive(Debug, Serialize, Deserialize)]
pub struct PageResult<T> {
    /// 数据列表
    pub content: Vec<T>,
    /// 当前页码
    pub page: u64,
    /// 每页大小
    pub size: u64,
    /// 总记录数
    pub total: u64,
    /// 总页数
    pub total_pages: u64,
    /// 是否第一页
    pub first: bool,
    /// 是否最后一页
    pub last: bool,
    /// 是否为空
    pub empty: bool,
}

impl<T> PageResult<T> {
    /// 创建分页结果
    pub fn new(content: Vec<T>, page: u64, size: u64, total: u64) -> Self {
        let total_pages = if size > 0 { (total + size - 1) / size } else { 0 };
        let first = page == 0;
        let last = page >= total_pages.saturating_sub(1);
        let empty = content.is_empty();
        
        Self {
            content,
            page,
            size,
            total,
            total_pages,
            first,
            last,
            empty,
        }
    }
    
    /// 创建空分页结果
    pub fn empty(page: u64, size: u64) -> Self {
        Self::new(Vec::new(), page, size, 0)
    }
    
    /// 是否有下一页
    pub fn has_next(&self) -> bool {
        !self.last && self.page + 1 < self.total_pages
    }
    
    /// 是否有上一页
    pub fn has_previous(&self) -> bool {
        !self.first && self.page > 0
    }
    
    /// 获取下一页页码
    pub fn next_page(&self) -> Option<u64> {
        if self.has_next() {
            Some(self.page + 1)
        } else {
            None
        }
    }
    
    /// 获取上一页页码
    pub fn previous_page(&self) -> Option<u64> {
        if self.has_previous() {
            Some(self.page - 1)
        } else {
            None
        }
    }
}