use std::cell::RefCell;
use std::time::Duration;
use std::slice::{Iter, IterMut};
use comp::{Component, ComponentManager};
use entity::{Entity, EntityManager, EntityRef, EntityRefMut};
use world::World;
/// Processes components of entities
/// With each Processor running continuously as if it has a private internal thread,
/// performing global actions on every Entity that possesses a Component of the same
/// aspect as the Processor
pub trait Processor: 'static {
    /// Process a single entity with a delta in seconds
    fn run(&mut self, entity: EntityRefMut, delta:f64);

    /// Process entities with a delta in seconds
    fn run_all(&mut self, entities: &mut EntityManager, components: &mut ComponentManager, entity: &Vec<Entity>, delta:f64) {
        for &entity in entity.iter() {
            self.run(EntityRefMut {
                entity: entity,
                entities: entities,
                components: components
            }, delta);
        }
    }
}
impl Processor for |EntityRefMut, f64|:'static {
    fn run(&mut self, entity: EntityRefMut, delta:f64) {
        (*self)(entity, delta)
    }
}
impl Processor for |f64|:'static {
    fn run(&mut self, _: EntityRefMut, _:f64) {
    }
    fn run_all(&mut self, _: &mut EntityManager, _: &mut ComponentManager, _: &Vec<Entity>, delta:f64) {
        (*self)(delta)
    }
}
impl<T> Processor for |&T, f64|:'static where T:Component {
    fn run(&mut self, entity: EntityRefMut, delta:f64) {
        (*self)(entity.borrow().unwrap(), delta)
    }
}
impl<T> Processor for |&mut T, f64|:'static where T:Component {
    fn run(&mut self, mut entity: EntityRefMut, delta:f64) {
        (*self)(entity.borrow_mut().unwrap(), delta)
    }
}
impl<A, B> Processor for |&A, &B, f64|:'static where A:Component, B:Component {
    fn run(&mut self, entity: EntityRefMut, delta:f64) {
        (*self)(entity.borrow().unwrap(), entity.borrow().unwrap(), delta)
    }
}
impl<A, B> Processor for |&mut A, B, f64|:'static where A:Component, B:Component+Copy {
    fn run(&mut self, mut entity: EntityRefMut, delta:f64) {
        let b = *entity.borrow().unwrap();
        (*self)(entity.borrow_mut().unwrap(), b, delta)
    }
}

/// A processor that runs every interval instead of every tick.
pub struct IntervalProcessor<S> {
    /// The interval, in seconds
    pub interval: f64,
    /// The number of seconds since the last update
    pub since_last: f64,
    /// The actual processor to run
    pub processor: S
}
impl<S> IntervalProcessor<S> where S:Processor {
    /// Makes a new interval processor from the given processor and interval
    pub fn new(processor: S, interval: Duration) -> IntervalProcessor<S> {
        IntervalProcessor {
            interval: interval.num_seconds() as f64 + 0.001 * interval.num_milliseconds() as f64,
            since_last: 0.0,
            processor: processor
        }
    }
}
impl<S> Processor for IntervalProcessor<S> where S:Processor {
    fn run(&mut self, entity: EntityRefMut, delta:f64) {
        self.processor.run(entity, delta)
    }

    fn run_all(&mut self, entities: &mut EntityManager, components: &mut ComponentManager, entity: &Vec<Entity>, delta:f64) {
        self.since_last += delta;
        if self.since_last > self.interval {
            self.since_last -= self.interval;
            let interval = self.interval;
            self.processor.run_all(entities, components, entity, interval);
        }
    }
}

/// Represents the processor and its aspect
pub struct MetaProcessor {
    /// The aspect to process
    pub aspect: RefCell<Box<Aspect>>,
    /// The processor
    pub processor: Box<Processor>,
    /// Relevant entities
    pub entities: RefCell<Vec<Entity>>
}
/// Encapsulates processors and their categories
pub struct ProcessorManager {
    processors: Vec<MetaProcessor>
}
impl ProcessorManager {
    /// Create a new processor manager
    pub fn new() -> ProcessorManager {
        ProcessorManager {
            processors : Vec::new()
        }
    }
    /// Update a processor
    pub fn update(&mut self, world: &World, index: uint) {
        let processor = &mut self.processors[index];
        let mut entities = processor.entities.borrow_mut();
        *entities = world.entities.iter().filter(|&entity| {
            let entity = EntityRef {
                entity: entity,
                entities: &world.entities,
                components: &world.components
            };
            entity.is_active() && processor.aspect.borrow_mut().check(entity)
        }).collect::<Vec<_>>();
    }
    /// Add a processor with an aspect into the world
    pub fn add(&mut self, processor: Box<Processor>, aspect: Box<Aspect>) {
        self.processors.push(MetaProcessor {
            processor: processor,
            aspect: RefCell::new(aspect),
            entities: RefCell::new(Vec::new())
        });
    }
    #[inline(always)]
    /// Iterate through the systems and their descriptors immutably
    pub fn iter(&self) -> Iter<MetaProcessor> {
        self.processors.iter()
    }
    #[inline(always)]
    /// Iterate through the systems and their descriptors mutably
    pub fn iter_mut(&mut self) -> IterMut<MetaProcessor> {
        self.processors.iter_mut()
    }
}

/// An Aspect is used by processors as a matcher against entities,
/// to check if a processor is interested in an entity
pub trait Aspect :'static {
    /// Check if the entity meets the criteria of the aspect
    fn check<'a>(&mut self, entity: EntityRef<'a>) -> bool;
}

impl Aspect for |EntityRef|:'static -> bool {
    fn check<'a>(&mut self, entity: EntityRef<'a>) -> bool {
        (*self)(entity)
    }
}