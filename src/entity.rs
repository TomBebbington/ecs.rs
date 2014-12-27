use std::default::Default;
use std::collections::HashSet;
use std::collections::hash_set::Iter;
use std::iter::Iterator;
use comp::{Component, ComponentType, ComponentManager};
use world::World;

/// A unique ID that tags each game object as a separate item
/// Implementations typically use a plain integer for this, as is used here

pub type Entity = uint;

/// Manages entity allocation and deletion
pub struct EntityManager {
    count: uint,
    max_index: uint,
    free_indices: Vec<Entity>,
    active: HashSet<Entity>
}
impl EntityManager {
    /// Create a new entity manager
    pub fn new() -> EntityManager {
        EntityManager {
            count: 0,
            max_index: 0,
            free_indices: Vec::with_capacity(10),
            active: HashSet::with_capacity(20)
        }
    }
    
    /// Create a new entity
    #[inline(always)]
    pub fn create(&mut self) -> Entity {
        self.count += 1;
        match self.free_indices.pop() {
            Some(index) => {
                index
            },
            None => {
                self.max_index += 1;
                self.count - 1
            }
        }
    }

    /// Enable an entity to be processed
    #[inline(always)]
    pub fn enable(&mut self, entity: Entity) {
        self.active.insert(entity);
    }
    /// Disable an entity from being processed
    #[inline(always)]
    pub fn disable(&mut self, entity: Entity) {
        self.active.remove(&entity);
    }
    /// Check if an entity is active
    #[inline(always)]
    pub fn is_active(&self, entity: Entity) -> bool {
        self.active.contains(&entity)
    }

    /// Check if an entity exists
    #[inline(always)]
    pub fn exists(&self, entity: Entity) -> bool {
        entity < self.count && !self.free_indices.iter().any(|&other| entity == other)
    }

    /// Delete an entity
    #[inline(always)]
    pub fn remove(&mut self, entity: Entity) {
        self.count -= 1;
        self.free_indices.push(entity);
    }

    /// Iterate through all the active entities
    #[inline(always)]
    pub fn iter(&self) -> EntityIter {
        EntityIter {
            iter: self.active.iter()
        }
    }
}
pub struct EntityIter<'a> {
    iter: Iter<'a, Entity>
}
impl<'a> Iterator<Entity> for EntityIter<'a> {
    fn next(&mut self) -> Option<Entity> {
        self.iter.next().map(|&e| e)
    }
}

/// An immutable reference into a world with an entity
/// This is pretty much equivalent to a MetaEntity in most ECS implementations
pub struct EntityRef<'a> {
    /// The components of the world the entity resides in
    pub components: &'a ComponentManager,
    /// The entities of the world the entity resides in
    pub entities: &'a EntityManager,
    /// The entity, a UUID
    pub entity: Entity
}

impl<'a> EntityRef<'a> {
    /// Check if the entity is active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.entities.is_active(self.entity)
    }
    /// Clone the component of the given type attached to this entity
    #[inline(always)]
    pub fn get<T:Component + Clone>(&mut self) -> T {
        let component:&T = &self.components[self.entity];
        component.clone()
    }
    /// Check if any components of the given type are attached to this entity
    #[inline(always)]
    pub fn has<T:Component>(&mut self) -> bool {
        self.components.has::<T>(self.entity)
    }
    /// Check if any components of the given type are attached to this entity
    #[inline(always)]
    pub fn has_id(&mut self, id: &ComponentType) -> bool {
        self.components.has_id(self.entity, id)
    }
    /// Borrow the component of the given type attached to this entity
    #[inline(always)]
    pub fn borrow<T:Component>(&mut self) -> Option<&'a T> {
        self.components.borrow(self.entity)
    }

}
/// An immutable reference into a world with an entity
pub struct EntityRefMut<'a> {
    /// The components of the world the entity resides in
    pub components: &'a mut ComponentManager,
    /// The entities of the world the entity resides in
    pub entities: &'a mut EntityManager,
    /// The entity, a UUID
    pub entity: Entity
}
impl<'a> EntityRefMut<'a> {
    /// Check if the entity is active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.entities.is_active(self.entity)
    }

    /// Enable the entity
    #[inline(always)]
    pub fn enable(self) -> EntityRefMut<'a> {
        self.entities.enable(self.entity);
        self
    }

    /// Disable the entity
    #[inline(always)]
    pub fn disable(self) -> EntityRefMut<'a> {
        self.entities.disable(self.entity);
        self
    }

    /// Add the component to the components list, and attach it to the entity
    #[inline(always)]
    pub fn add<T:Component>(self, component: T) -> EntityRefMut<'a> {
        self.components.add(self.entity, component);
        self
    }
    
    /// Add the component to the components list, and attach it to the entity
    #[inline(always)]
    pub fn add_default<T:Component + Default>(self) -> EntityRefMut<'a> {
        let component : T = Default::default();
        self.components.add(self.entity, component);
        self
    }
    /// Clone the component of the given type attached to this entity
    #[inline(always)]
    pub fn get<T:Component + Clone>(&self) -> Option<T> {
        self.components.get(self.entity)
    }
    
    #[inline(always)]
    /// Check if any components of the given type are attached to this entity
    pub fn has<T:Component>(&mut self) -> bool {
        self.components.has::<T>(self.entity)
    }
    #[inline(always)]
    /// Set the component in the components list, and attach it to the entity
    pub fn set<T:Component>(self, component: T) -> EntityRefMut<'a> {
        self.components.set(self.entity, component);
        self
    }
    #[inline(always)]
    /// Borrow immutably the component from the components list attached to this entity
    pub fn borrow<T:Component>(&self) -> Option<&T> {
        self.components.borrow(self.entity)
    }
    #[inline(always)]
    /// Borrow mutably the component from the components list attached to this entity
    pub fn borrow_mut<T:Component>(&mut self) -> Option<&mut T> {
        self.components.borrow_mut(self.entity)
    }
    #[inline(always)]
    /// Remove the component from the component list attached to this entity
    pub fn remove<T:Component>(self) -> EntityRefMut<'a> {
        self.components.remove::<T>(self.entity);
        self
    }
    #[inline(always)]
    /// Delete the entity
    pub fn delete(self) {
        self.components.clear_entity(self.entity)
    }
}

/// A closure capably of building an entity
pub trait EntityBuilder {
    /// Build a single entity and return it
    fn build(&mut self, entity: EntityRefMut) -> Entity;

    /// Build multiple entities
    fn build_some<'a, I:Iterator<&'a Entity>>(&mut self, world: &mut World, mut iter: I) {
        for &entity in iter {
            let entity_ref = world.get_entity_mut(entity);
            self.build(entity_ref);
        }
    }
}
impl<'a> EntityBuilder for |EntityRefMut|:'a {
    #[inline(always)]
    fn build(&mut self, mut entity: EntityRefMut) -> Entity {
        let entity_id = entity.entity;
        entity = entity.enable();
        (*self)(entity);
        entity_id
    }
}