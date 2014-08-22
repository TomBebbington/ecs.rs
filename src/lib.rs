//! Entity Component System
#![crate_name = "ecs"]
#![comment = "A lightweight Component Entity System"]
#![license = "MIT"]
#![crate_type = "lib"]
#![doc(
    html_favicon_url = "http://tombebbington.github.io/favicon.png"
  )]
#![experimental]
#![deny(non_uppercase_statics, missing_doc, unnecessary_parens, unrecognized_lint,
	unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation,
	uppercase_variables, non_camel_case_types, unused_must_use)]
extern crate anymap;
use anymap::AnyMap;
use std::any::Any;
pub use bag::Bag;
mod bag;
/// An entity
pub struct Entity {
	components: AnyMap
}
impl Entity {
	/// Make a new component	
	pub fn new() -> Entity {
		Entity {
			components: AnyMap::new()
		}
	}
	/// Add a component to it
	pub fn add<C:Send>(&mut self, component: C) {
		self.components.insert(component);
	}
	/// Remove a component
	pub fn remove<C:Send>(&mut self) {
		 self.components.remove::<C>()
	}
	/// Get a component in it
	pub fn get<C:Send>(&self) -> Option<&C> {
		self.components.find::<C>()
	}
	/// Get a mutable component in it
	pub fn get_mut<C:Send>(&mut self) -> Option<&mut C> {
		self.components.find_mut::<C>()
	}
	/// Check if it has a value
	pub fn has<C:Send>(&self) -> bool {
		self.components.contains::<C>()
	}
}
/// A system
pub trait System: Send {
	/// Process 
	fn run(&mut self, e:&Entity, i:&Any);
	/// Check if an entity can be processed
	fn can_process(&self, e:&Entity) -> bool;
}
/// A world
pub struct World {
	entities: Vec<Entity>,
	systems: Vec<Box<System>>
}
impl World {
	/// Make a new world
	pub fn new() -> World {
		World {
			entities: Vec::new(),
			systems: Vec::new()
		}
	}
	/// Make a new entity
	pub fn make_entity(&mut self) -> &mut Entity {
		self.entities.push(Entity::new());
		let last = self.entities.len() - 1;
		self.entities.get_mut(last)
	}
	/// Run a generation on all the entities and stuff
	pub fn run(&mut self, i:&Any) {
		for entity in self.entities.mut_iter() {
			for system in self.systems.mut_iter() {
				if system.can_process(entity) {
					system.run(entity, i)
				}
			}
		}
	}
	/// Iterate through all the world's entities
	pub fn entities(&mut self) -> ::std::slice::Items<Entity> {
		self.entities.iter()
	}
}