//! RSpring 应用程序模块
//! 
//! 提供应用程序生命周期管理和应用上下文功能

use crate::{
    config::{ConfigurationManager, AppConfig, ServerConfig, LoggingConfig},
    container::Container,
    error::{Error, Result},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug, error};

/// 应用上下文
/// 
/// 管理全局的组件容器和配置管理器
#[derive(Debug)]
pub struct ApplicationContext {
    /// 依赖注入容器
    pub container: Arc<RwLock<Container>>,
    /// 配置管理器
    pub config: Arc<ConfigurationManager>,
}

impl ApplicationContext {
    /// 创建新的应用上下文
    /// 
    /// # 错误
    /// 当配置管理器创建失败时返回错误
    pub fn new() -> Result<Self> {
        debug!("创建应用上下文");
        
        let config = Arc::new(ConfigurationManager::new()?);
        let container = Arc::new(RwLock::new(Container::new()));
        
        info!("应用上下文创建成功");
        
        Ok(Self {
            container,
            config,
        })
    }
    
    /// 注册组件到容器
    /// 
    /// # 参数
    /// * `component` - 要注册的组件实例
    pub async fn register<T: 'static + Send + Sync + crate::Component>(&self, component: T) {
        debug!("注册组件: {}", component.component_name());
        let mut container = self.container.write().await;
        if let Err(e) = container.register(component) {
            error!("组件注册失败: {}", e);
        }
    }
    
    /// 注册单例组件到容器
    /// 
    /// # 参数
    /// * `component` - 要注册的单例组件实例
    pub async fn register_singleton<T: 'static + Send + Sync + crate::Component>(&self, component: T) {
        debug!("注册单例组件: {}", component.component_name());
        let mut container = self.container.write().await;
        if let Err(e) = container.register_singleton(component) {
            error!("单例组件注册失败: {}", e);
        }
    }
    
    /// 获取组件实例
    pub async fn get<T: 'static>(&self) -> Option<Arc<T>> {
        let container = self.container.read().await;
        container.get_singleton::<T>()
    }
    
    /// 执行容器自动装配
    pub async fn auto_wire(&self) -> Result<()> {
        info!("开始执行容器自动装配");
        let mut container = self.container.write().await;
        container.auto_wire()
    }
    
    /// 获取配置管理器引用
    pub fn config_manager(&self) -> &Arc<ConfigurationManager> {
        &self.config
    }
    
    /// 获取容器引用
    pub fn container(&self) -> &Arc<RwLock<Container>> {
        &self.container
    }
}

/// RSpring 应用程序特征
/// 
/// 定义应用程序的基本接口
pub trait RSpringApplication {
    /// 运行应用程序
    async fn run() -> Result<()>;
}

/// RSpring 应用程序实现
/// 
/// 提供默认的应用程序实现，支持基本的配置管理和日志功能
pub struct RSpringApp {
    /// 应用上下文
    context: ApplicationContext,
}

impl RSpringApp {
    /// 创建新的应用程序实例
    /// 
    /// # 错误
    /// 当应用上下文创建失败时返回错误
    pub fn new() -> Result<Self> {
        let context = ApplicationContext::new()?;
        Ok(Self { context })
    }
    
    /// 运行应用程序
    /// 
    /// 执行完整的应用程序生命周期：
    /// 1. 初始化日志系统
    /// 2. 加载配置
    /// 3. 自动装配容器
    /// 4. 启动应用（等待关闭信号）
    pub async fn run(&self) -> Result<()> {
        // 1. 初始化日志系统
        self.init_logging().await?;
        
        info!("启动 RSpring 应用程序");
        
        // 2. 加载和验证配置
        self.load_configuration().await?;
        
        // 3. 执行自动装配
        self.context.auto_wire().await?;
        
        info!("RSpring 应用程序启动完成");
        
        // 4. 保持运行直到收到关闭信号
        self.await_shutdown().await?;
        
        info!("RSpring 应用程序已停止");
        Ok(())
    }
    
    /// 初始化日志系统
    async fn init_logging(&self) -> Result<()> {
        let logging_config = self.context.config
            .get_section::<LoggingConfig>("logging")
            .unwrap_or_else(|_| LoggingConfig::default());
        
        crate::logging::init_logging(&logging_config)?;
        info!("日志系统初始化完成");
        Ok(())
    }
    
    /// 加载和验证配置
    async fn load_configuration(&self) -> Result<()> {
        debug!("加载应用配置");
        
        // 尝试加载应用基本配置
        let app_config = self.context.config
            .get_section::<AppConfig>("app")
            .unwrap_or_else(|_| AppConfig::default());
        
        info!(
            "应用配置加载完成 - 名称: {}, 版本: {}, 调试模式: {}", 
            app_config.name, 
            app_config.version, 
            app_config.debug
        );
        
        // 加载服务器配置（如果存在）
        if self.context.config.contains_key("server") {
            let server_config = self.context.config
                .get_section::<ServerConfig>("server")
                .unwrap_or_else(|_| ServerConfig::default());
            
            info!(
                "服务器配置: {}:{}, 工作线程: {:?}",
                server_config.host,
                server_config.port,
                server_config.workers.unwrap_or_else(|| num_cpus::get())
            );
        }
        
        Ok(())
    }
    
    /// 等待应用程序关闭信号
    async fn await_shutdown(&self) -> Result<()> {
        info!("应用程序运行中，按 Ctrl+C 停止");
        
        tokio::signal::ctrl_c().await
            .map_err(|e| Error::runtime(format!("等待关闭信号失败: {}", e)))?;
        
        info!("收到关闭信号，正在停止应用程序");
        Ok(())
    }
    
    /// 获取应用上下文
    pub fn context(&self) -> &ApplicationContext {
        &self.context
    }
}

impl Default for RSpringApp {
    fn default() -> Self {
        Self::new().expect("Failed to create RSpring application")
    }
}

/// 为向后兼容保留的类型别名
pub type AxumBootApplication = RSpringApp;