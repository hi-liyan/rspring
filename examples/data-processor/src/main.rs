use axum_boot_core::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// 应用配置
#[derive(Debug, Deserialize, Configuration)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub batch_size: usize,
    pub interval_seconds: u64,
}

/// 数据模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataRecord {
    pub id: u64,
    pub content: String,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 数据源服务 - 模拟从某个数据源读取数据
#[derive(Service)]
pub struct DataSourceService {
    config: Arc<AppConfig>,
}

impl DataSourceService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
    
    /// 批量获取待处理数据
    pub async fn fetch_pending_data(&self) -> Result<Vec<DataRecord>> {
        // 模拟从数据库、消息队列或文件中读取数据
        tracing::info!("从数据源获取 {} 条待处理记录", self.config.batch_size);
        
        let records = (1..=self.config.batch_size)
            .map(|i| DataRecord {
                id: i as u64,
                content: format!("数据记录 {}", i),
                processed_at: None,
            })
            .collect();
        
        // 模拟IO延迟
        sleep(Duration::from_millis(100)).await;
        
        Ok(records)
    }
}

/// 数据处理服务 - 处理业务逻辑
#[derive(Service)]
pub struct DataProcessorService;

impl DataProcessorService {
    /// 处理单条数据记录
    pub async fn process_record(&self, mut record: DataRecord) -> Result<DataRecord> {
        tracing::debug!("处理数据记录: {}", record.id);
        
        // 模拟数据处理逻辑
        record.content = format!("已处理: {}", record.content);
        record.processed_at = Some(chrono::Utc::now());
        
        // 模拟处理时间
        sleep(Duration::from_millis(50)).await;
        
        Ok(record)
    }
    
    /// 批量处理数据
    pub async fn process_batch(&self, records: Vec<DataRecord>) -> Result<Vec<DataRecord>> {
        tracing::info!("开始批量处理 {} 条记录", records.len());
        
        let mut processed_records = Vec::new();
        
        for record in records {
            match self.process_record(record).await {
                Ok(processed) => processed_records.push(processed),
                Err(e) => tracing::error!("处理记录失败: {}", e),
            }
        }
        
        tracing::info!("批量处理完成，成功处理 {} 条记录", processed_records.len());
        Ok(processed_records)
    }
}

/// 结果存储服务 - 保存处理结果
#[derive(Service)]
pub struct ResultStorageService;

impl ResultStorageService {
    /// 保存处理结果
    pub async fn save_results(&self, records: Vec<DataRecord>) -> Result<()> {
        tracing::info!("保存 {} 条处理结果", records.len());
        
        // 模拟保存到数据库、文件或消息队列
        for record in records {
            tracing::debug!("保存记录: {} -> {}", record.id, record.content);
        }
        
        // 模拟IO延迟
        sleep(Duration::from_millis(200)).await;
        
        tracing::info!("所有结果保存完成");
        Ok(())
    }
}

/// 数据处理任务调度器 - 协调各个服务
#[derive(Service)]
pub struct TaskSchedulerService {
    data_source: Arc<DataSourceService>,
    processor: Arc<DataProcessorService>,
    storage: Arc<ResultStorageService>,
    config: Arc<AppConfig>,
}

impl TaskSchedulerService {
    pub fn new(
        data_source: Arc<DataSourceService>,
        processor: Arc<DataProcessorService>,
        storage: Arc<ResultStorageService>,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            data_source,
            processor,
            storage,
            config,
        }
    }
    
    /// 执行一次数据处理循环
    pub async fn run_cycle(&self) -> Result<()> {
        tracing::info!("开始数据处理循环");
        
        // 1. 获取待处理数据
        let pending_data = self.data_source.fetch_pending_data().await?;
        
        if pending_data.is_empty() {
            tracing::info!("没有待处理数据，跳过本次循环");
            return Ok(());
        }
        
        // 2. 处理数据
        let processed_data = self.processor.process_batch(pending_data).await?;
        
        // 3. 保存结果
        self.storage.save_results(processed_data).await?;
        
        tracing::info!("数据处理循环完成");
        Ok(())
    }
    
    /// 启动定时任务
    pub async fn start_scheduled_tasks(&self) -> Result<()> {
        tracing::info!("启动定时数据处理任务，间隔: {}秒", self.config.interval_seconds);
        
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.interval_seconds));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.run_cycle().await {
                tracing::error!("数据处理循环出错: {}", e);
                // 继续下一次循环，不退出
            }
        }
    }
}

/// CLI应用程序入口 - 展示 axum-boot-core 的非Web使用场景
#[axum_boot_application]
pub struct DataProcessorApplication;

#[tokio::main]
async fn main() -> Result<()> {
    DataProcessorApplication::run().await
}