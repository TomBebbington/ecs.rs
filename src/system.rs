use entity::Entity;

/// A system
pub trait System: Send {
	/// Process a single entity
	fn run(&mut self, entity:&mut Entity, delta:f64);
	/// Process an iterator of compatible entities, ran once a frame
	fn run_all<It:Iterator<Entity>>(&mut self, mut iter:It, delta:f64) {
		for mut e in iter {
			self.run(&mut e, delta)
		}
	}
	/// Check if an entity can be processed
	fn can_process(&self, _:&Entity) -> bool {
		false
	}
}
