use std::collections::{HashMap, VecMap};
use std::intrinsics::TypeId;
use std::ops::{Index, IndexMut};
use bag::Bag;
use entity::Entity;
use util;

/// All the data for one aspect of the object
pub trait Component:'static + Send + Sized {

}
impl<T:'static + Send + Sized> Component for T {

}

/// Identifies the type of component
#[deriving(Copy, Eq, PartialEq, Hash)]
pub struct ComponentType {
    id: u64
}
impl ComponentType {
    /// Get the component type of a given component
    #[inline(always)]
    pub fn of<T:Component>() -> ComponentType {
        ComponentType {
            id: TypeId::of::<T>().hash()
        }
    }
}
/// Stores the components in a bag and the entity mappings in a map
pub struct ComponentList {
    components: Bag,
    recyclable_indices: Vec<uint>,
    entities: VecMap<uint>
}
impl<C:Component> Index<Entity, C> for ComponentList {
    fn index<'a>(&'a self, entity: &Entity) -> &'a C {
        let &index = self.entities.get(entity).unwrap();
        &self.components[index]
    }
}
impl<C:Component> IndexMut<Entity, C> for ComponentList {
    fn index_mut<'a>(&'a mut self, entity: &Entity) -> &'a mut C {
        let &index = self.entities.get_mut(entity).unwrap();
        &mut self.components[index]
    }
}
impl ComponentList {
    pub fn with_capacity<T:Component>(capacity: uint) -> ComponentList {
        ComponentList {
            components: Bag::with_capacity::<T>(capacity),
            entities: VecMap::with_capacity(capacity),
            recyclable_indices: Vec::new()
        }
    }
    #[inline(always)]
    pub fn has(&self, entity: Entity) -> bool {
        self.entities.contains_key(&entity)
    }
    #[inline(always)]
    pub fn add<T:Component>(&mut self, entity: Entity, component: T) {
        let index = match self.recyclable_indices.pop() {
            Some(index) => index,
            None => self.components.add(component)
        };
        self.entities.insert(entity, index);
    }
    #[inline(always)]
    pub fn set<T:Component>(&mut self, entity: Entity, component: T) {
        self[entity] = component
    }
    #[inline(always)]
    pub fn remove(&mut self, ref entity: Entity) {
        if let Some(&index) = self.entities.get(entity) {
            self.recyclable_indices.push(index);
            self.entities.remove(entity);
        }
    }
}

/// Attaches component types to the component lists
///
/// Components are stored as described in
/// [this article](http://cbpowell.wordpress.com/2012/12/05/entity-component-game-programming-using-jruby-and-libgdx-part-2/)
/// But they're stored in the ComponentManager here because that makes much more sense
pub struct ComponentManager {
    components: HashMap<ComponentType, ComponentList>
}

impl<T:Component> Index<Entity, T> for ComponentManager {
    fn index<'a>(&'a self, entity: &Entity) -> &'a T {
        let list = self.components.get(&ComponentType::of::<T>()).unwrap();
        &list[*entity]
    }
}
impl<T:Component> IndexMut<Entity, T> for ComponentManager {
    fn index_mut<'a>(&'a mut self, entity: &Entity) -> &'a mut T {
        let list = self.components.get_mut(&ComponentType::of::<T>()).unwrap();
        &mut list[*entity]
    }
}

impl ComponentManager {
    /// Create a new component manager
    pub fn new() -> ComponentManager {
        ComponentManager {
            components: HashMap::new()
        }
    }
    /// Create a new component manager
    pub fn with_capacity(capacity: uint) -> ComponentManager {
        ComponentManager {
            components: HashMap::with_capacity(capacity)
        }
    }
    /// Check if the components has a component attached to the entity
    #[inline(always)]
    pub fn has_id(&self, entity: Entity, id: &ComponentType) -> bool {
        self.components.get(id).unwrap().has(entity)
    }
    /// Check if the components has a component attached to the entity
    #[inline(always)]
    pub fn has<T: Component>(&self, entity: Entity) -> bool {
        self.has_id(entity, &ComponentType::of::<T>())
    }
    /// Add a component attached to the entity
    pub fn add<T: Component>(&mut self, entity: Entity, component: T) {
        debug!("Added {} to {}", util::get_type_name::<T>(), entity);
        let list = self.components.get_mut(&ComponentType::of::<T>()).unwrap();
        list.add(entity, component);
    }

    /// Set the component in the components list, and attach it to the entity
    #[inline(always)]
    pub fn set<T: Component>(&mut self, entity: Entity, component: T) {
        debug!("Set {} to {}", util::get_type_name::<T>(), entity);
        let list = self.components.get_mut(&ComponentType::of::<T>()).unwrap();
        list.set(entity, component);
    }

    /// Borrow immutably the component from the components list attached to this entity
    pub fn borrow<T: Component>(&self, entity: Entity) -> Option<&T> {
        debug!("Borrowed {} from {}", util::get_type_name::<T>(), entity);
        self.components.get(&ComponentType::of::<T>()).map(|list|
            &list[entity])
    }

    /// Borrow mutably the component from the components list attached to this entity
    pub fn borrow_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        debug!("Borrowed {} from {} mutably", util::get_type_name::<T>(), entity);
        self.components.get_mut(&ComponentType::of::<T>()).map(|list|
            &mut list[entity])
    }

    /// Get the components list for a component
    #[inline(always)]
    pub fn get_list<T: Component>(&mut self) -> &mut ComponentList {
        debug!("Getting list of {}", util::get_type_name::<T>());
        self.components.get_mut(&ComponentType::of::<T>()).unwrap()
    }

    /// Clone the component from the components list attached to this entity
    #[inline(always)]
    pub fn get<T: Component + Clone>(&self, entity: Entity) -> Option<T> {
        debug!("Copied {} from {}", util::get_type_name::<T>(), entity);
        self.borrow::<T>(entity).map(|v| v.clone())
    }

    /// Remove the component in the components list, and attach it to the entity
    pub fn remove<T: Component>(&mut self, entity: Entity) {
        debug!("Removed {} from {}", util::get_type_name::<T>(), entity);
        match self.components.get_mut(&ComponentType::of::<T>()) {
            Some(entities) => {
                entities.remove(entity);
            },
            None => ()
        };
    }
    
    /// Remove all the components in the component lists attached to the entity
    pub fn clear_entity(&mut self, entity: Entity) {
        debug!("Removed all from {}", entity);
        for (_, list) in self.components.iter_mut() {
            list.remove(entity);
        }
    }

    /// Clear all the components in a component list
    #[inline(always)]
    pub fn clear<T: Component>(&mut self) {
        self.components.remove(&ComponentType::of::<T>());
    }

    /// Register a component
    #[inline(always)]
    pub fn register<T: Component>(&mut self) {
        self.components.insert(ComponentType::of::<T>(), ComponentList::with_capacity::<T>(5));
    }
}