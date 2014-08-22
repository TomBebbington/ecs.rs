use anymap::AnyMap;

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