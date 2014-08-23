use entity::Entity;

/// A system
pub trait System: Send {
	/// Process 
	fn run(&mut self, e:&mut Entity, delta:f64);
	/// Check if an entity can be processed
	fn can_process(&self, e:&Entity) -> bool;
}
