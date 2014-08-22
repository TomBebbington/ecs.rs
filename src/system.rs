use entity::Entity;
use std::any::Any;

/// A system
pub trait System: Send {
	/// Process 
	fn run(&mut self, e:&Entity, i:&Any);
	/// Check if an entity can be processed
	fn can_process(&self, e:&Entity) -> bool;
}
