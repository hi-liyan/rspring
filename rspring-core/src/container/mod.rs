//! 依赖注入容器模块
//! 
//! 提供类似 Spring IoC 容器的功能，支持：
//! - 组件自动注册和发现
//! - 依赖自动注入
//! - 生命周期管理
//! - 循环依赖检测

pub mod registry;
pub mod injection;

// 重新导出主要类型
pub use registry::{ComponentRegistry, ComponentMetadata, ComponentLifecycle, RegistryStats};
pub use injection::{DependencyInjector, InjectionStats};

use std::any::Any;

/// 依赖注入容器
/// 
/// 整合注册表和注入器功能的高级容器
pub struct Container {
    /// 依赖注入器
    injector: DependencyInjector,
}

impl Container {
    /// 创建新的容器实例
    pub fn new() -> Self {
        Self {
            injector: DependencyInjector::new(),
        }
    }
    
    /// 注册组件
    pub fn register<T: 'static + Send + Sync + Component>(&mut self, component: T) -> crate::Result<()> {
        self.injector.registry_mut().register(component, None)
    }
    
    /// 注册带名称的组件
    pub fn register_named<T: 'static + Send + Sync + Component>(
        &mut self, 
        component: T, 
        name: String
    ) -> crate::Result<()> {
        self.injector.registry_mut().register(component, Some(name))
    }
    
    /// 注册单例组件
    pub fn register_singleton<T: 'static + Send + Sync + Component>(&mut self, component: T) -> crate::Result<()> {
        self.injector.registry_mut().register_singleton(component, None)
    }
    
    /// 注册带名称的单例组件
    pub fn register_singleton_named<T: 'static + Send + Sync + Component>(
        &mut self, 
        component: T, 
        name: String
    ) -> crate::Result<()> {
        self.injector.registry_mut().register_singleton(component, Some(name))
    }
    
    /// 获取组件实例
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.injector.get::<T>()
    }
    
    /// 获取单例组件实例
    pub fn get_singleton<T: 'static>(&self) -> Option<std::sync::Arc<T>> {
        self.injector.get_singleton::<T>()
    }
    
    /// 检查是否包含指定类型的组件
    pub fn contains<T: 'static>(&self) -> bool {
        self.injector.registry().contains::<T>()
    }
    
    /// 执行自动装配
    pub fn auto_wire(&mut self) -> crate::Result<()> {
        self.injector.auto_wire()
    }
    
    /// 验证依赖完整性
    pub fn validate(&self) -> crate::Result<()> {
        self.injector.validate_dependencies()
    }
    
    /// 获取容器统计信息
    pub fn stats(&self) -> ContainerStats {
        let injection_stats = self.injector.get_injection_stats();
        ContainerStats {
            total_components: injection_stats.total_components,
            singleton_components: injection_stats.singleton_components,
            prototype_components: injection_stats.prototype_components,
            total_dependencies: injection_stats.total_dependencies,
            auto_wired: injection_stats.initialization_order_calculated,
        }
    }
    
    /// 获取依赖注入器的引用（用于高级操作）
    pub fn injector(&self) -> &DependencyInjector {
        &self.injector
    }
    
    /// 获取依赖注入器的可变引用（用于高级操作）
    pub fn injector_mut(&mut self) -> &mut DependencyInjector {
        &mut self.injector
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

/// 容器统计信息
#[derive(Debug, Clone)]
pub struct ContainerStats {
    /// 总组件数
    pub total_components: usize,
    /// 单例组件数
    pub singleton_components: usize,
    /// 原型组件数
    pub prototype_components: usize,
    /// 总依赖关系数
    pub total_dependencies: usize,
    /// 是否已执行自动装配
    pub auto_wired: bool,
}

/// 组件特征
/// 
/// 所有注册到容器的组件都必须实现此特征
pub trait Component: Send + Sync {
    /// 获取组件名称
    /// 
    /// 用于日志记录和调试
    fn component_name(&self) -> &'static str;
}

/// 服务组件标记特征
/// 
/// 用于标记业务逻辑组件
pub trait Service: Component {}

/// 仓储组件标记特征
/// 
/// 用于标记数据访问组件
pub trait Repository: Component {}

/// 控制器组件标记特征
/// 
/// 基础的控制器标记，具体 Web 功能由其他模块提供
pub trait Controller: Component {}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestService {
        name: String,
    }

    impl Component for TestService {
        fn component_name(&self) -> &'static str {
            "TestService"
        }
    }

    impl Service for TestService {}

    impl TestService {
        fn new(name: String) -> Self {
            Self { name }
        }
        
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_container_basic_operations() {
        let mut container = Container::new();
        
        let service = TestService::new("test".to_string());
        container.register(service).unwrap();
        
        container.auto_wire().unwrap();
        
        let retrieved = container.get::<TestService>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get_name(), "test");
        
        let stats = container.stats();
        assert_eq!(stats.total_components, 1);
        assert_eq!(stats.prototype_components, 1);
        assert!(stats.auto_wired);
    }

    #[test]
    fn test_singleton_registration() {
        let mut container = Container::new();
        
        let service = TestService::new("singleton".to_string());
        container.register_singleton(service).unwrap();
        
        container.auto_wire().unwrap();
        
        let singleton = container.get_singleton::<TestService>();
        assert!(singleton.is_some());
        assert_eq!(singleton.unwrap().get_name(), "singleton");
        
        let stats = container.stats();
        assert_eq!(stats.singleton_components, 1);
    }

    #[test]
    fn test_named_component_registration() {
        let mut container = Container::new();
        
        let service = TestService::new("named".to_string());
        container.register_named(service, "my_service".to_string()).unwrap();
        
        container.auto_wire().unwrap();
        
        let retrieved = container.get::<TestService>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().get_name(), "named");
    }
}