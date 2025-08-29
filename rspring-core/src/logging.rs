//! 日志系统模块
//! 
//! 提供基于 tracing 的统一日志功能

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use crate::config::LoggingConfig;
use crate::error::Result;

/// 初始化日志系统
/// 
/// # 参数
/// * `config` - 日志配置
/// 
/// # 错误
/// 当日志初始化失败时返回错误
pub fn init_logging(config: &LoggingConfig) -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.level));
    
    match config.format.as_str() {
        "json" => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        "compact" => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer().compact())
                .init();
        }
        "pretty" | _ => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer().pretty())
                .init();
        }
    }
    
    tracing::info!("日志系统已初始化，级别: {}, 格式: {}", config.level, config.format);
    
    Ok(())
}