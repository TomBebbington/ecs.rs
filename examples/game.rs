#![feature(plugin)]
#[macro_use] #[plugin]
extern crate eccles;
use eccles::*;

#[derive(Copy)]
pub struct Pos(f64, f64);
#[derive(Copy)]
pub struct Vel(f64, f64);

world!{
	name: Game,
	components: {
		pos => Pos,
		vel => Vel
	},
	processors: {
		movement => Movement for [pos, vel]
	}
}
#[derive(Copy, Default)]
pub struct Movement;
impl Movement {
	fn run(&mut self, delta: f64, entities: &[Entity], pos: &mut Components<Pos>, vel: &Components<Vel>) {
		for &e in entities.iter() {
			let Vel(vx, vy) = vel[e];
			let &mut Pos(ref mut x, ref mut y) = &mut pos[e];
			*x += vx * delta;
			*y += vy * delta;
		}
	}
}
pub fn main() {
	let mut world:Game = World::new();
	let entity = world.create();
	world.pos.insert(entity, Pos(1.0, -2.0));
	world.pos.insert(entity, Pos(3.0, 2.0));
	world.update(1.0);
}