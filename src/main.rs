#![link(plugin)]
extern crate ecs;
use ecs::Entity;
fn main() {
	let mut thing = Entity::new();
	thing.add(Position{ x: 10.0, y: 3.0});
	println!("{}", thing.get::<Position>().unwrap());
}
#[deriving(Show)]
pub struct Position {
	pub x: f32,
	pub y: f32
}