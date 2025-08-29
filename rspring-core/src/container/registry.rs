//! 组件注册模块
//! 
//! 提供组件注册和管理功能，支持不同生命周期的组件管理

use crate::error::{Error, Result};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// 组件生命周期类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentLifecycle {
    /// 单例模式 - 整个应用生命周期内只有一个实例
    Singleton,
    /// 原型模式 - 每次获取都创建新实例（暂未实现）
    Prototype,
}

/// 组件元数据
#[derive(Debug, Clone)]
pub struct ComponentMetadata {
    /// 组件名称
    pub name: String,
    /// 组件类型 ID
    pub type_id: TypeId,
    /// 生命周期类型
    pub lifecycle: ComponentLifecycle,
    /// 注册时间
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// 描述信息
    pub description: Option<String>,
}

/// 组件注册表
/// 
/// 管理所有注册的组件，支持按类型查找和生命周期管理
#[derive(Debug)]
pub struct ComponentRegistry {
    /// 组件存储 - 普通组件
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    /// 单例组件存储
    singletons: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    /// 组件元数据
    metadata: HashMap<TypeId, ComponentMetadata>,
    /// 组件依赖关系图
    dependencies: HashMap<TypeId, Vec<TypeId>>,
}

impl ComponentRegistry {
    /// 创建新的组件注册表
    pub fn new() -> Self {
        info!("创建新的组件注册表");
        
        Self {
            components: HashMap::new(),
            singletons: HashMap::new(),
            metadata: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }
    
    /// 注册普通组件
    /// 
    /// # 参数
    /// * `component` - 组件实例
    /// * `name` - 组件名称（可选）
    /// 
    /// # 示例
    /// ```rust
    /// let mut registry = ComponentRegistry::new();
    /// registry.register(MyService::new(), Some("user_service".to_string()))?;
    /// ```
    pub fn register<T: 'static + Send + Sync>(
        &mut self, 
        component: T,
        name: Option<String>
    ) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let component_name = name.unwrap_or_else(|| {
            std::any::type_name::<T>().split("::").last().unwrap_or("Unknown").to_string()
        });
        
        debug!("注册组件: {} (类型: {})", component_name, std::any::type_name::<T>());
        
        // 检查是否已注册
        if self.components.contains_key(&type_id) || self.singletons.contains_key(&type_id) {
            return Err(Error::container(format!("组件 {} 已经注册", component_name)));
        }
        
        // 存储组件
        self.components.insert(type_id, Box::new(component));
        
        // 存储元数据
        let metadata = ComponentMetadata {
            name: component_name.clone(),
            type_id,
            lifecycle: ComponentLifecycle::Prototype,
            registered_at: chrono::Utc::now(),
            description: None,
        };
        self.metadata.insert(type_id, metadata);
        
        info!("成功注册组件: {}", component_name);
        Ok(())
    }
    
    /// 注册单例组件
    /// 
    /// # 参数
    /// * `component` - 组件实例
    /// * `name` - 组件名称（可选）
    /// 
    /// # 示例
    /// ```rust
    /// let mut registry = ComponentRegistry::new();
    /// registry.register_singleton(ConfigService::new(), None)?;
    /// ```
    pub fn register_singleton<T: 'static + Send + Sync>(
        &mut self, 
        component: T,
        name: Option<String>
    ) -> Result<()> {
        let type_id = TypeId::of::<T>();
        let component_name = name.unwrap_or_else(|| {
            std::any::type_name::<T>().split("::").last().unwrap_or("Unknown").to_string()
        });
        
        debug!("注册单例组件: {} (类型: {})", component_name, std::any::type_name::<T>());
        
        // 检查是否已注册
        if self.components.contains_key(&type_id) || self.singletons.contains_key(&type_id) {
            return Err(Error::container(format!("组件 {} 已经注册", component_name)));
        }
        
        // 存储单例组件
        self.singletons.insert(type_id, Arc::new(component));
        
        // 存储元数据
        let metadata = ComponentMetadata {
            name: component_name.clone(),
            type_id,
            lifecycle: ComponentLifecycle::Singleton,
            registered_at: chrono::Utc::now(),
            description: None,
        };
        self.metadata.insert(type_id, metadata);
        
        info!("成功注册单例组件: {}", component_name);
        Ok(())
    }
    
    /// 获取普通组件的引用
    /// 
    /// # 返回值
    /// 返回组件的不可变引用，如果组件不存在返回 None
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        
        debug!("获取组件: {}", std::any::type_name::<T>());
        
        self.components.get(&type_id)?.downcast_ref()
    }
    
    /// 获取单例组件的 Arc 智能指针
    /// 
    /// # 返回值
    /// 返回组件的 Arc 智能指针，如果组件不存在返回 None
    pub fn get_singleton<T: 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        
        debug!("获取单例组件: {}", std::any::type_name::<T>());
        
        let any_arc = self.singletons.get(&type_id)?;
        
        // 尝试安全地向下转型
        // 这里我们需要使用 unsafe，但我们通过 TypeId 确保了类型安全
        unsafe {
            let raw_ptr = Arc::into_raw(any_arc.clone());
            
            // 验证类型 ID 匹配
            if (*raw_ptr).type_id() == type_id {
                Some(Arc::from_raw(raw_ptr as *const T))
            } else {
                // 恢复 Arc 避免内存泄漏
                Arc::from_raw(raw_ptr);
                None
            }
        }
    }
    
    /// 检查是否包含指定类型的组件
    /// 
    /// # 参数
    /// * `T` - 要检查的组件类型
    /// 
    /// # 返回值
    /// 如果包含该类型的组件返回 true，否则返回 false
    pub fn contains<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.contains_key(&type_id) || self.singletons.contains_key(&type_id)
    }
    
    /// 检查是否包含指定类型 ID 的组件
    pub fn contains_type_id(&self, type_id: &TypeId) -> bool {
        self.components.contains_key(type_id) || self.singletons.contains_key(type_id)
    }
    
    /// 移除组件
    /// 
    /// # 参数
    /// * `T` - 要移除的组件类型
    /// 
    /// # 返回值
    /// 如果成功移除返回 true，如果组件不存在返回 false
    pub fn remove<T: 'static>(&mut self) -> bool {
        let type_id = TypeId::of::<T>();
        let component_name = std::any::type_name::<T>();
        
        debug!("移除组件: {}", component_name);
        
        let removed = self.components.remove(&type_id).is_some() 
            || self.singletons.remove(&type_id).is_some();
        
        if removed {
            self.metadata.remove(&type_id);
            self.dependencies.remove(&type_id);
            info!("成功移除组件: {}", component_name);
        } else {
            warn!("尝试移除不存在的组件: {}", component_name);
        }
        
        removed
    }
    
    /// 获取组件元数据
    pub fn get_metadata<T: 'static>(&self) -> Option<&ComponentMetadata> {
        let type_id = TypeId::of::<T>();
        self.metadata.get(&type_id)
    }
    
    /// 获取所有组件的元数据
    pub fn list_components(&self) -> Vec<&ComponentMetadata> {
        self.metadata.values().collect()
    }
    
    /// 获取组件数量统计
    pub fn stats(&self) -> ComponentStats {
        ComponentStats {
            total_components: self.components.len() + self.singletons.len(),
            prototype_components: self.components.len(),
            singleton_components: self.singletons.len(),
        }
    }
    
    /// 记录依赖关系
    /// 
    /// # 参数
    /// * `dependent` - 依赖者的类型 ID
    /// * `dependency` - 被依赖者的类型 ID
    pub fn add_dependency(&mut self, dependent: TypeId, dependency: TypeId) {
        self.dependencies
            .entry(dependent)
            .or_default()
            .push(dependency);
    }
    
    /// 获取组件的依赖列表
    pub fn get_dependencies(&self, type_id: &TypeId) -> Vec<TypeId> {
        self.dependencies.get(type_id).cloned().unwrap_or_default()
    }
    
    /// 检测循环依赖
    pub fn detect_circular_dependencies(&self) -> Result<()> {
        let mut visited = std::collections::HashSet::new();
        let mut path = std::collections::HashSet::new();
        
        for &type_id in self.dependencies.keys() {
            if !visited.contains(&type_id) {
                self.detect_cycle_dfs(type_id, &mut visited, &mut path)?;
            }
        }
        
        Ok(())
    }
    
    /// 深度优先搜索检测循环依赖
    fn detect_cycle_dfs(
        &self,
        current: TypeId,
        visited: &mut std::collections::HashSet<TypeId>,
        path: &mut std::collections::HashSet<TypeId>,
    ) -> Result<()> {
        if path.contains(&current) {
            return Err(Error::dependency_injection("检测到循环依赖"));
        }
        
        if visited.contains(&current) {
            return Ok(());
        }
        
        visited.insert(current);
        path.insert(current);
        
        if let Some(deps) = self.dependencies.get(&current) {
            for &dep in deps {
                self.detect_cycle_dfs(dep, visited, path)?;
            }
        }
        
        path.remove(&current);
        Ok(())
    }
    
    /// 清空所有组件
    pub fn clear(&mut self) {
        info!("清空组件注册表");
        
        self.components.clear();
        self.singletons.clear();
        self.metadata.clear();
        self.dependencies.clear();
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 组件统计信息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentStats {
    /// 总组件数
    pub total_components: usize,
    /// 原型组件数
    pub prototype_components: usize,
    /// 单例组件数
    pub singleton_components: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::Component;

    struct TestService;
    impl Component for TestService {
        fn component_name(&self) -> &'static str {
            "TestService"
        }
    }

    struct TestRepository {
        value: i32,
    }
    
    impl Component for TestRepository {
        fn component_name(&self) -> &'static str {
            "TestRepository"
        }
    }

    #[test]
    fn test_register_and_get_component() {
        let mut registry = ComponentRegistry::new();
        let service = TestService;
        
        registry.register(service, Some("test_service".to_string())).unwrap();
        
        let retrieved = registry.get::<TestService>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().component_name(), "TestService");
        
        // 检查元数据
        let metadata = registry.get_metadata::<TestService>();
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().name, "test_service");
        assert_eq!(metadata.unwrap().lifecycle, ComponentLifecycle::Prototype);
    }

    #[test]
    fn test_register_and_get_singleton() {
        let mut registry = ComponentRegistry::new();
        let repo = TestRepository { value: 42 };
        
        registry.register_singleton(repo, None).unwrap();
        
        let retrieved = registry.get_singleton::<TestRepository>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().value, 42);
        
        // 检查元数据
        let metadata = registry.get_metadata::<TestRepository>();
        assert!(metadata.is_some());
        assert_eq!(metadata.unwrap().lifecycle, ComponentLifecycle::Singleton);
    }

    #[test]
    fn test_duplicate_registration() {
        let mut registry = ComponentRegistry::new();
        let service1 = TestService;
        let service2 = TestService;
        
        registry.register(service1, None).unwrap();
        let result = registry.register(service2, None);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("已经注册"));
    }

    #[test]
    fn test_contains() {
        let mut registry = ComponentRegistry::new();
        
        assert!(!registry.contains::<TestService>());
        
        registry.register(TestService, None).unwrap();
        assert!(registry.contains::<TestService>());
    }

    #[test]
    fn test_remove() {
        let mut registry = ComponentRegistry::new();
        registry.register(TestService, None).unwrap();
        
        assert!(registry.contains::<TestService>());
        assert!(registry.remove::<TestService>());
        assert!(!registry.contains::<TestService>());
        assert!(!registry.remove::<TestService>()); // 二次移除应返回 false
    }

    #[test]
    fn test_stats() {
        let mut registry = ComponentRegistry::new();
        
        let initial_stats = registry.stats();
        assert_eq!(initial_stats.total_components, 0);
        
        registry.register(TestService, None).unwrap();
        registry.register_singleton(TestRepository { value: 1 }, None).unwrap();
        
        let stats = registry.stats();
        assert_eq!(stats.total_components, 2);
        assert_eq!(stats.prototype_components, 1);
        assert_eq!(stats.singleton_components, 1);
    }

    #[test]
    fn test_list_components() {
        let mut registry = ComponentRegistry::new();
        registry.register(TestService, Some("service1".to_string())).unwrap();
        registry.register_singleton(TestRepository { value: 1 }, Some("repo1".to_string())).unwrap();
        
        let components = registry.list_components();
        assert_eq!(components.len(), 2);
        
        let names: Vec<&String> = components.iter().map(|c| &c.name).collect();
        assert!(names.contains(&&"service1".to_string()));
        assert!(names.contains(&&"repo1".to_string()));
    }

    #[test]
    fn test_clear() {
        let mut registry = ComponentRegistry::new();
        registry.register(TestService, None).unwrap();
        registry.register_singleton(TestRepository { value: 1 }, None).unwrap();
        
        assert_eq!(registry.stats().total_components, 2);
        
        registry.clear();
        assert_eq!(registry.stats().total_components, 0);
        assert!(!registry.contains::<TestService>());
        assert!(!registry.contains::<TestRepository>());
    }

    #[test]
    fn test_dependency_tracking() {
        let mut registry = ComponentRegistry::new();
        let service_id = TypeId::of::<TestService>();
        let repo_id = TypeId::of::<TestRepository>();
        
        registry.add_dependency(service_id, repo_id);
        
        let deps = registry.get_dependencies(&service_id);
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], repo_id);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut registry = ComponentRegistry::new();
        let service_id = TypeId::of::<TestService>();
        let repo_id = TypeId::of::<TestRepository>();
        
        // 创建循环依赖：Service -> Repository -> Service
        registry.add_dependency(service_id, repo_id);
        registry.add_dependency(repo_id, service_id);
        
        let result = registry.detect_circular_dependencies();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("循环依赖"));
    }
}