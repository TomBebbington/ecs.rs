//! Eccles is a lightweight yet complete macro-based Entity Component System
//! that is extremely fast
//!
//! For examples, please look under the examples directory
#![allow(unused_features)]
#![feature(core, test)]

extern crate clock_ticks;
#[cfg(test)]
extern crate "test" as testy;
use std::collections::VecMap;
#[cfg(test)]
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
}
pub struct WorldIterator<W> where W:World {
	last_time: f64,
	world: W
}
impl<W> Iterator for WorldIterator<W> where W:World {
	type Item = f64;
	fn next(&mut self) -> Option<f64> {
		use clock_ticks::precise_time_s;
		let current = precise_time_s();
		let delta = current - self.last_time;
		self.world.update(delta);
		Some(delta)
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
		impl ::std::iter::IntoIterator for $name {
			type Iter = WorldIterator<$name>;
			fn into_iter(self) -> WorldIterator<$name> {
				use clock_ticks::precise_time_s;
				WorldIterator {
					last_time: precise_time_s(),
					world: self
				}
			}
		}
	)
}