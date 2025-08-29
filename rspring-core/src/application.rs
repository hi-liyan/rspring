use crate::{
    config::{ConfigurationManager, ServerConfig, LoggingConfig},
    container::Container,
    error::Result,
    logging,
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct ApplicationContext {
    pub container: Arc<RwLock<Container>>,
    pub config: Arc<ConfigurationManager>,
}

impl ApplicationContext {
    pub fn new() -> Result<Self> {
        let config = Arc::new(ConfigurationManager::new()?);
        let container = Arc::new(RwLock::new(Container::new()));
        
        Ok(Self {
            container,
            config,
        })
    }
    
    pub async fn register<T: 'static + Send + Sync>(&self, component: T) {
        let mut container = self.container.write().await;
        container.register(component);
    }
    
    pub async fn register_singleton<T: 'static + Send + Sync>(&self, component: T) {
        let mut container = self.container.write().await;
        container.register_singleton(component);
    }
}

pub struct AxumBootApplication {
    context: ApplicationContext,
}

impl AxumBootApplication {
    pub fn new() -> Result<Self> {
        let context = ApplicationContext::new()?;
        Ok(Self { context })
    }
    
    pub async fn run(&self) -> Result<()> {
        // Initialize logging
        let logging_config = self.context.config.bind::<LoggingConfig>()
            .unwrap_or_default();
        logging::init_logging(&logging_config)?;
        
        tracing::info!("Starting AxumBoot application...");
        
        // Get server configuration
        let server_config = self.context.config.bind::<ServerConfig>()
            .unwrap_or_default();
        
        tracing::info!("Server will start on {}:{}", server_config.host, server_config.port);
        
        // TODO: Start web server when axum-boot-starter-web is integrated
        tracing::info!("AxumBoot application started successfully!");
        
        // Keep the application running
        tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl_c");
        tracing::info!("Shutting down AxumBoot application...");
        
        Ok(())
    }
    
    pub fn context(&self) -> &ApplicationContext {
        &self.context
    }
}

impl Default for AxumBootApplication {
    fn default() -> Self {
        Self::new().expect("Failed to create AxumBoot application")
    }
}