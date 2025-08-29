//! 依赖注入模块
//! 
//! 实现自动依赖注入功能，包括构造函数注入、字段注入等

use crate::error::{Error, Result};
use crate::container::registry::{ComponentRegistry, ComponentLifecycle};
use std::any::TypeId;
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{debug, info, warn};

/// 依赖注入器
/// 
/// 负责解析组件依赖关系并执行依赖注入
pub struct DependencyInjector {
    /// 组件注册表
    registry: ComponentRegistry,
    /// 初始化顺序缓存
    initialization_order: Vec<TypeId>,
    /// 是否已经计算过初始化顺序
    order_calculated: bool,
}

impl DependencyInjector {
    /// 创建新的依赖注入器
    pub fn new() -> Self {
        info!("创建依赖注入器");
        
        Self {
            registry: ComponentRegistry::new(),
            initialization_order: Vec::new(),
            order_calculated: false,
        }
    }
    
    /// 使用现有的组件注册表创建依赖注入器
    pub fn with_registry(registry: ComponentRegistry) -> Self {
        info!("使用现有注册表创建依赖注入器");
        
        Self {
            registry,
            initialization_order: Vec::new(),
            order_calculated: false,
        }
    }
    
    /// 获取组件注册表的引用
    pub fn registry(&self) -> &ComponentRegistry {
        &self.registry
    }
    
    /// 获取组件注册表的可变引用
    pub fn registry_mut(&mut self) -> &mut ComponentRegistry {
        // 重置初始化顺序，因为注册表可能被修改
        self.order_calculated = false;
        &mut self.registry
    }
    
    /// 自动装配所有组件
    /// 
    /// 执行完整的依赖注入流程：
    /// 1. 检测循环依赖
    /// 2. 计算初始化顺序（拓扑排序）
    /// 3. 按顺序初始化组件
    pub fn auto_wire(&mut self) -> Result<()> {
        info!("开始自动装配过程");
        
        // 1. 检测循环依赖
        self.registry.detect_circular_dependencies()
            .map_err(|e| {
                Error::dependency_injection(format!("循环依赖检测失败: {}", e))
            })?;
        
        // 2. 计算初始化顺序
        self.calculate_initialization_order()?;
        
        // 3. 执行依赖注入
        self.inject_dependencies()?;
        
        info!("自动装配完成");
        Ok(())
    }
    
    /// 计算组件初始化顺序
    /// 
    /// 使用拓扑排序算法确定组件的初始化顺序，确保依赖组件先于被依赖组件初始化
    fn calculate_initialization_order(&mut self) -> Result<()> {
        if self.order_calculated {
            return Ok(());
        }
        
        debug!("计算组件初始化顺序");
        
        let mut in_degree: HashMap<TypeId, usize> = HashMap::new();
        let mut all_types = HashSet::new();
        
        // 收集所有类型和计算入度
        for metadata in self.registry.list_components() {
            let type_id = metadata.type_id;
            all_types.insert(type_id);
            in_degree.insert(type_id, 0);
        }
        
        // 计算每个节点的入度
        for &type_id in &all_types {
            for dep_type in self.registry.get_dependencies(&type_id) {
                if all_types.contains(&dep_type) {
                    *in_degree.entry(dep_type).or_insert(0) += 1;
                }
            }
        }
        
        // 拓扑排序
        let mut queue = VecDeque::new();
        let mut result = Vec::new();
        
        // 将入度为0的节点加入队列
        for (&type_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(type_id);
            }
        }
        
        // 执行拓扑排序
        while let Some(current) = queue.pop_front() {
            result.push(current);
            
            // 处理当前节点的依赖
            for dep_type in self.registry.get_dependencies(&current) {
                if let Some(degree) = in_degree.get_mut(&dep_type) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dep_type);
                    }
                }
            }
        }
        
        // 检查是否所有节点都被处理（循环依赖检测）
        if result.len() != all_types.len() {
            return Err(Error::dependency_injection(
                "拓扑排序失败，可能存在循环依赖".to_string()
            ));
        }
        
        self.initialization_order = result;
        self.order_calculated = true;
        
        debug!("初始化顺序计算完成，共 {} 个组件", self.initialization_order.len());
        Ok(())
    }
    
    /// 执行依赖注入
    /// 
    /// 按照计算出的顺序初始化组件并注入依赖
    fn inject_dependencies(&mut self) -> Result<()> {
        debug!("开始执行依赖注入");
        
        let mut injected_count = 0;
        
        // 按初始化顺序处理组件
        for &type_id in &self.initialization_order.clone() {
            // 获取组件元数据
            if let Some(metadata) = self.registry.metadata.get(&type_id).cloned() {
                debug!("处理组件依赖注入: {}", metadata.name);
                
                // 检查组件的依赖是否都已经可用
                let dependencies = self.registry.get_dependencies(&type_id);
                for dep_type_id in dependencies {
                    if !self.registry.contains_type_id(&dep_type_id) {
                        return Err(Error::dependency_injection(format!(
                            "组件 {} 的依赖组件未找到",
                            metadata.name
                        )));
                    }
                }
                
                injected_count += 1;
                debug!("成功注入组件: {}", metadata.name);
            }
        }
        
        info!("依赖注入完成，共处理 {} 个组件", injected_count);
        Ok(())
    }
    
    /// 注册组件并添加到依赖图中
    /// 
    /// # 参数
    /// * `component` - 组件实例
    /// * `name` - 组件名称
    /// * `dependencies` - 依赖的类型 ID 列表
    pub fn register_with_dependencies<T: 'static + Send + Sync>(
        &mut self,
        component: T,
        name: Option<String>,
        dependencies: Vec<TypeId>,
    ) -> Result<()> {
        let type_id = TypeId::of::<T>();
        
        // 注册组件
        self.registry.register(component, name)?;
        
        // 添加依赖关系
        for dep_type_id in dependencies {
            self.registry.add_dependency(type_id, dep_type_id);
        }
        
        // 重置初始化顺序
        self.order_calculated = false;
        
        Ok(())
    }
    
    /// 注册单例组件并添加到依赖图中
    pub fn register_singleton_with_dependencies<T: 'static + Send + Sync>(
        &mut self,
        component: T,
        name: Option<String>,
        dependencies: Vec<TypeId>,
    ) -> Result<()> {
        let type_id = TypeId::of::<T>();
        
        // 注册单例组件
        self.registry.register_singleton(component, name)?;
        
        // 添加依赖关系
        for dep_type_id in dependencies {
            self.registry.add_dependency(type_id, dep_type_id);
        }
        
        // 重置初始化顺序
        self.order_calculated = false;
        
        Ok(())
    }
    
    /// 获取组件实例
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.registry.get::<T>()
    }
    
    /// 获取单例组件实例
    pub fn get_singleton<T: 'static>(&self) -> Option<std::sync::Arc<T>> {
        self.registry.get_singleton::<T>()
    }
    
    /// 验证依赖图的完整性
    /// 
    /// 检查所有组件的依赖是否都能被满足
    pub fn validate_dependencies(&self) -> Result<()> {
        debug!("验证依赖图完整性");
        
        for metadata in self.registry.list_components() {
            let type_id = metadata.type_id;
            let dependencies = self.registry.get_dependencies(&type_id);
            
            for dep_type_id in dependencies {
                if !self.registry.contains_type_id(&dep_type_id) {
                    return Err(Error::dependency_injection(format!(
                        "组件 {} 依赖的组件未找到 (TypeId: {:?})",
                        metadata.name, dep_type_id
                    )));
                }
            }
        }
        
        info!("依赖图验证通过");
        Ok(())
    }
    
    /// 获取初始化顺序
    pub fn get_initialization_order(&mut self) -> Result<&[TypeId]> {
        if !self.order_calculated {
            self.calculate_initialization_order()?;
        }
        Ok(&self.initialization_order)
    }
    
    /// 获取依赖注入统计信息
    pub fn get_injection_stats(&self) -> InjectionStats {
        let registry_stats = self.registry.stats();
        let total_dependencies: usize = self.registry.metadata
            .keys()
            .map(|type_id| self.registry.get_dependencies(type_id).len())
            .sum();
        
        InjectionStats {
            total_components: registry_stats.total_components,
            singleton_components: registry_stats.singleton_components,
            prototype_components: registry_stats.prototype_components,
            total_dependencies,
            initialization_order_calculated: self.order_calculated,
        }
    }
}

impl Default for DependencyInjector {
    fn default() -> Self {
        Self::new()
    }
}

/// 依赖注入统计信息
#[derive(Debug, Clone)]
pub struct InjectionStats {
    /// 总组件数
    pub total_components: usize,
    /// 单例组件数
    pub singleton_components: usize,
    /// 原型组件数
    pub prototype_components: usize,
    /// 总依赖关系数
    pub total_dependencies: usize,
    /// 是否已计算初始化顺序
    pub initialization_order_calculated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::Component;
    
    struct ServiceA;
    impl Component for ServiceA {
        fn component_name(&self) -> &'static str {
            "ServiceA"
        }
    }
    
    struct ServiceB;
    impl Component for ServiceB {
        fn component_name(&self) -> &'static str {
            "ServiceB"
        }
    }
    
    struct ServiceC;
    impl Component for ServiceC {
        fn component_name(&self) -> &'static str {
            "ServiceC"
        }
    }

    #[test]
    fn test_basic_auto_wiring() {
        let mut injector = DependencyInjector::new();
        
        // 注册没有依赖的组件
        injector.registry_mut().register(ServiceA, Some("service_a".to_string())).unwrap();
        
        // 执行自动装配
        let result = injector.auto_wire();
        assert!(result.is_ok());
        
        // 验证组件可以获取
        assert!(injector.get::<ServiceA>().is_some());
    }

    #[test]
    fn test_dependency_injection_with_order() {
        let mut injector = DependencyInjector::new();
        
        let service_a_id = TypeId::of::<ServiceA>();
        let service_b_id = TypeId::of::<ServiceB>();
        let service_c_id = TypeId::of::<ServiceC>();
        
        // ServiceC 依赖 ServiceB，ServiceB 依赖 ServiceA
        injector.register_with_dependencies(
            ServiceA, 
            Some("service_a".to_string()), 
            vec![]
        ).unwrap();
        
        injector.register_with_dependencies(
            ServiceB, 
            Some("service_b".to_string()), 
            vec![service_a_id]
        ).unwrap();
        
        injector.register_with_dependencies(
            ServiceC, 
            Some("service_c".to_string()), 
            vec![service_b_id]
        ).unwrap();
        
        // 执行自动装配
        let result = injector.auto_wire();
        assert!(result.is_ok());
        
        // 验证所有组件都可以获取
        assert!(injector.get::<ServiceA>().is_some());
        assert!(injector.get::<ServiceB>().is_some());
        assert!(injector.get::<ServiceC>().is_some());
        
        // 验证初始化顺序
        let order = injector.get_initialization_order().unwrap();
        
        // ServiceA 应该在 ServiceB 之前，ServiceB 应该在 ServiceC 之前
        let a_pos = order.iter().position(|&id| id == service_a_id).unwrap();
        let b_pos = order.iter().position(|&id| id == service_b_id).unwrap();
        let c_pos = order.iter().position(|&id| id == service_c_id).unwrap();
        
        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut injector = DependencyInjector::new();
        
        let service_a_id = TypeId::of::<ServiceA>();
        let service_b_id = TypeId::of::<ServiceB>();
        
        // 创建循环依赖：A -> B -> A
        injector.register_with_dependencies(
            ServiceA, 
            Some("service_a".to_string()), 
            vec![service_b_id]
        ).unwrap();
        
        injector.register_with_dependencies(
            ServiceB, 
            Some("service_b".to_string()), 
            vec![service_a_id]
        ).unwrap();
        
        // 自动装配应该失败
        let result = injector.auto_wire();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("循环依赖"));
    }

    #[test]
    fn test_missing_dependency() {
        let mut injector = DependencyInjector::new();
        
        let non_existent_id = TypeId::of::<ServiceB>(); // ServiceB 未注册
        
        // 注册 ServiceA 但声明依赖于未注册的 ServiceB
        injector.register_with_dependencies(
            ServiceA, 
            Some("service_a".to_string()), 
            vec![non_existent_id]
        ).unwrap();
        
        // 验证依赖应该失败
        let result = injector.validate_dependencies();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("未找到"));
    }

    #[test]
    fn test_singleton_registration() {
        let mut injector = DependencyInjector::new();
        
        injector.register_singleton_with_dependencies(
            ServiceA, 
            Some("singleton_service".to_string()), 
            vec![]
        ).unwrap();
        
        let result = injector.auto_wire();
        assert!(result.is_ok());
        
        // 验证可以获取单例组件
        let singleton = injector.get_singleton::<ServiceA>();
        assert!(singleton.is_some());
        
        // 获取统计信息
        let stats = injector.get_injection_stats();
        assert_eq!(stats.singleton_components, 1);
        assert_eq!(stats.total_components, 1);
    }

    #[test]
    fn test_injection_stats() {
        let mut injector = DependencyInjector::new();
        
        let service_a_id = TypeId::of::<ServiceA>();
        
        injector.register_with_dependencies(ServiceA, None, vec![]).unwrap();
        injector.register_singleton_with_dependencies(
            ServiceB, 
            None, 
            vec![service_a_id]
        ).unwrap();
        
        injector.auto_wire().unwrap();
        
        let stats = injector.get_injection_stats();
        assert_eq!(stats.total_components, 2);
        assert_eq!(stats.singleton_components, 1);
        assert_eq!(stats.prototype_components, 1);
        assert_eq!(stats.total_dependencies, 1); // ServiceB 依赖 ServiceA
        assert!(stats.initialization_order_calculated);
    }
}