//! Entity Component System
#![crate_name = "ecs"]
#![comment = "A lightweight Component Entity System"]
#![license = "MIT"]
#![crate_type = "lib"]
#![doc(
    html_favicon_url = "http://tombebbington.github.io/favicon.png"
  )]
#![experimental]
#![feature(globs)]
#![allow(missing_doc)]
#![deny(non_uppercase_statics, unnecessary_parens, unrecognized_lint,
	unreachable_code, unnecessary_allocation, unnecessary_typecast, unnecessary_allocation,
	uppercase_variables, non_camel_case_types, unused_must_use)]
extern crate anymap;
extern crate graphics;
extern crate opengl_graphics;
pub use bag::Bag;
pub use entity::Entity;
pub use world::World;
pub use system::System;
pub mod game;
mod bag;
mod entity;
mod system;
mod world;