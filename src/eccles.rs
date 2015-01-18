//! Eccles is a lightweight yet complete macro-based Entity Component System
//! that is extremely fast
//!
//! For examples, please look under the examples directory

extern crate time;
#[cfg(test)]
extern crate "test" as testy;
use std::collections::VecMap;
#[cfg(test)]
#[allow(unstable)]
mod test;

/// A single entity
pub type Entity = usize;

/// Some components
pub type Components<T> = VecMap<T>;

/// The world contains all the components
pub trait World {
	/// Make a new world
	fn new() -> Self;
	/// Create a new entity in the world
	fn create(&mut self) -> Entity;
	/// Delete an entity and its components from the world
	fn delete(&mut self, entity: Entity);
	/// Update the processors in the world
	fn update(&mut self, delta: f64);
	/// Enter a loop in the world
	fn enter_loop(&mut self) {
		use time::precise_time_s;
		let mut last = precise_time_s();
		loop {
			let now = precise_time_s();
			self.update(now - last);
			last = now;
		}
	}
}
#[macro_export]
macro_rules! world{
	(name: $name:ident,
		components: {$($cfield:ident => $component:ident),+},
		processors: {$($pfield:ident => $processor:ident for [$($cfield2:ident),+]),+}) => (
		pub struct $name {
			pub count: usize,
			pub free_spots: Vec<usize>,
			$(pub $cfield: Components<$component>,)+
			$(pub $pfield: $processor,)+
		}
		impl World for $name {
			fn new() -> $name {
				use std::collections::VecMap;
				use std::default::Default;
				$name {
					count: 0,
					free_spots: Vec::with_capacity(5),
					$($cfield: VecMap::new(),)+
					$($pfield: Default::default(),)+
				}
			}
			fn create(&mut self) -> Entity {
				if let Some(spot) = self.free_spots.pop() {
					spot
				} else {
					self.count += 1;
					self.count - 1
				}
			}
			fn delete(&mut self, entity: Entity) {
				self.count -= 1;
				self.free_spots.push(entity);
				$(
					self.$cfield.remove(&entity);
				)+
			}
			fn update(&mut self, dt: f64) {
				use std::iter;
				use std::mem;
				let entities:Vec<Entity> = iter::range(0, self.count)
					.filter(|id| !self.free_spots.iter().any(|oid| oid == id))
					.collect();
				$(
					self.$pfield.run(dt, entities.as_slice(), $(unsafe { mem::transmute(&self.$cfield2) }),+);
				)+
			}
		}
	)
}