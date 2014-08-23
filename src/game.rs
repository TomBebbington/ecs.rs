use entity::Entity;
use system::System;
use graphics::*;
use opengl_graphics::{Gl, Texture};
/// A 2D position
pub struct Position2d {
	pub x: f64,
	pub y: f64
}
/// A 2D velocity
#[deriving(Clone)]
pub struct Velocity2d {
	pub vx: f64,
	pub vy: f64
}
/// A 3D position
pub struct Position3d {
	pub x: f64,
	pub y: f64,
	pub z: f64
}
/// A 3D velocity
#[deriving(Clone)]
pub struct Velocity3d {
	pub vx: f64,
	pub vy: f64,
	pub vz: f64
}
pub struct VelocitySystem2d;
impl System for VelocitySystem2d {
	fn can_process(&self, e:&Entity) -> bool {
		e.has::<Position2d>() && e.has::<Velocity2d>()
	}
	fn run(&mut self, e:&mut Entity, delta: f64) {
		let velocity = e.get_mut::<Velocity2d>().unwrap().clone();
		let position = e.get_mut::<Position2d>().unwrap();
		position.x += velocity.vx * delta;
		position.y += velocity.vy * delta;
	}
}
pub struct Sprite {
	pub texture:Texture
}
/// A graphics system
pub struct GraphicsSystem {
	pub context: Context,
	pub backend: Gl
}
impl GraphicsSystem {
	pub fn new<Back:BackEnd<IS>, IS:ImageSize>(width: f64, height:f64) -> GraphicsSystem {
		GraphicsSystem {
			context: Context::abs(width, height),
			backend: Gl::new()
		}
	}
}
impl System for GraphicsSystem {
	fn can_process(&self, e:&Entity) -> bool {
		e.has::<Position2d>() && e.has::<Sprite>()
	}
	fn run(&mut self, e:&mut Entity, _: f64) {
		let position = e.get::<Position2d>().unwrap();
		let texture = &e.get::<Sprite>().unwrap().texture;
		self.context.trans(position.x, position.y).image(texture).draw(&mut self.backend);
	}
}