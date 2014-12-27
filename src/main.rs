#![feature(phase)]
#[phase(link, plugin)]
extern crate ecs;
use ecs::{World, EntityRef, EntityRefMut};

fn main() {
    let mut world = world!{
        components: [Position, Velocity],
        processors: [
            |mut entity:EntityRefMut, delta: f64| {
                let vel = entity.get::<Velocity>().unwrap();
                let pos = entity.borrow_mut::<Position>().unwrap();
                pos.x += vel.x * delta;
                pos.y += vel.y * delta;
            } for
                aspect!(Position & Velocity)
        ]
    };
    let entities = world.build_entities(10, entity![
        Position {
            x: 3.0,
            y: 15.0
        },
        Velocity {
            x: -21.0,
            y: 0.5
        }
    ]);
    println!("{}", world.entities().collect::<Vec<_>>());
    world.update(1.0);
    println!("{}", world.get_entity(entities[0]).get::<Position>().x);
}

comp!(Position = {
    x: f64 = 0.0,
    y: f64 = 0.0
});
comp!(Velocity = {
    x: f64 = 0.0,
    y: f64 = 0.0
});