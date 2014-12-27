use comp::{Component, ComponentManager};
use entity::{EntityIter, Entity, EntityRef, EntityRefMut, EntityBuilder, EntityManager};
use processor::{Aspect, Processor, ProcessorManager};
use util;
/// Encapsulates the entities, the components they consist of and
/// the processors that operate upon them
pub struct World {
    /// Manages the data
    pub components: ComponentManager,
    /// Manages the processors of the data
    pub processors: ProcessorManager,
    /// Manages the unique identifiers of bits of data
    pub entities: EntityManager
}
impl World {
    /// Creates a new world
    pub fn new() -> World {
        debug!("Created a new world");
        World {
            components: ComponentManager::new(),
            processors: ProcessorManager::new(),
            entities: EntityManager::new()
        }
    }
    /// Creates a new world
    pub fn with_capacity(capacity: uint) -> World {
        debug!("Created a new world");
        World {
            components: ComponentManager::with_capacity(capacity),
            processors: ProcessorManager::new(),
            entities: EntityManager::new()
        }
    }
    #[inline(always)]
    /// Creates an entity in the world and returns it and the world
    pub fn new_entity(&mut self) -> EntityRefMut {
        debug!("Built new entity in world");
        EntityRefMut {
            entity: self.entities.create(),
            components: &mut self.components,
            entities: &mut self.entities
        }
    }
    #[inline(always)]
    /// Build an entity and enable it
    pub fn build_entity<B:EntityBuilder>(&mut self, mut builder: B) -> Entity {
        debug!("Built new entity with builder {} in world", util::get_type_name::<B>());
        let entity = builder.build(self.new_entity());
        self.entities.enable(entity);
        entity
    }
    #[inline(always)]
    /// Build a certain number of entities and enable them
    pub fn build_entities<B:EntityBuilder>(&mut self, count: uint, mut builder: B) -> Vec<Entity> {
        debug!("Built {} new entities with builder {} in world", count, util::get_type_name::<B>());
        let built_entities = Vec::from_fn(count, |_| self.entities.create());
        builder.build_some(self, built_entities.iter());
        built_entities
    }
    #[inline(always)]
    /// Removes an entity from the world
    pub fn remove_entity(&mut self, entity: Entity) {
        debug!("Removed {} from world", entity);
        self.components.clear_entity(entity);
        self.entities.remove(entity);
    }
    #[inline(always)]
    /// Get an immutable reference to an entity in the world
    pub fn get_entity(&self, entity: Entity) -> EntityRef {
        EntityRef {
            entity: entity,
            entities: &self.entities,
            components: &self.components
        }
    }
    #[inline(always)]
    /// Get a mutable reference to an entity in the world
    pub fn get_entity_mut(&mut self, entity: Entity) -> EntityRefMut {
        EntityRefMut {
            entity: entity,
            entities: &mut self.entities,
            components: &mut self.components
        }
    }
    /// Update all the processors with a delta, in seconds
    pub fn update(&mut self, delta: f64) {
        debug!("Updated processors by {} ", delta);
        for processor in self.processors.iter_mut() {
            processor.processor.run_all(&mut self.entities, &mut self.components, &*processor.entities.borrow(), delta);
        }
    }
    /// Begin a loop of updating the world
    pub fn begin_loop(mut self) {
        use time;
        let mut last = time::precise_time_s();
        loop {
            let now = time::precise_time_s();
            let delta = now - last;
            self.update(delta);
            last = now;
        }
    }
    /// Allocate a component list for the given type of component
    #[inline(always)]
    pub fn register_component<C:Component>(&mut self) {
        self.components.register::<C>();
    }
    #[inline(always)]
    /// Add a processor to the world
    pub fn register_processor(&mut self, processor: Box<Processor>, aspect: Box<Aspect>) {
        self.processors.add(processor, aspect);
    }

    /// Iterate through the entities
    #[inline(always)]
    pub fn entities(&self) -> EntityIter {
        self.entities.iter()
    }
}