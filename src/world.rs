use entity::Entity;
use std::any::Any;
use system::System;
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