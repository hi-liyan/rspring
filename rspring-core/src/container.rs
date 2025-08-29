use crate::error::Result;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Component: Send + Sync {
    fn component_name(&self) -> &'static str;
}

pub trait Service: Component {}
pub trait Repository: Component {}
pub trait Controller: Component {}

#[derive(Debug)]
pub struct Container {
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    singletons: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            singletons: HashMap::new(),
        }
    }
    
    pub fn register<T: 'static + Send + Sync>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.components.insert(type_id, Box::new(component));
    }
    
    pub fn register_singleton<T: 'static + Send + Sync>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        self.singletons.insert(type_id, Arc::new(component));
    }
    
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.components.get(&type_id)?.downcast_ref()
    }
    
    pub fn get_singleton<T: 'static>(&self) -> Option<Arc<T>> {
        let type_id = TypeId::of::<T>();
        let any_arc = self.singletons.get(&type_id)?;
        
        // Clone the Arc and try to downcast it
        let cloned = any_arc.clone();
        
        // Use unsafe downcast - this is safe if we ensure type consistency
        // In a production version, we might want to use a different approach
        unsafe {
            let raw = Arc::into_raw(cloned);
            if (*raw).type_id() == type_id {
                Some(Arc::from_raw(raw as *const T))
            } else {
                Arc::from_raw(raw); // Restore the Arc to avoid memory leak
                None
            }
        }
    }
    
    pub fn contains<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.components.contains_key(&type_id) || self.singletons.contains_key(&type_id)
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestService;
    impl Component for TestService {
        fn component_name(&self) -> &'static str {
            "TestService"
        }
    }
    
    #[test]
    fn test_container_register_and_get() {
        let mut container = Container::new();
        let service = TestService;
        
        container.register(service);
        
        let retrieved = container.get::<TestService>();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().component_name(), "TestService");
    }
}