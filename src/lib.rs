//! An Entity Component Processor implemented in Rust
//! 
//! Based on concepts detailed at [this wiki](http://entity-processors.wikidot.com/)
#![crate_name = "eccles"]
#![crate_type = "lib"]
#![doc(
    html_favicon_url = "http://tombebbington.github.io/favicon.png"
  )]
#![experimental]
#![feature(globs, phase, macro_rules, default_type_params)]
#![deny(missing_docs, unused_parens, unknown_lints,
    unreachable_code, unused_allocation, unused_allocation,
    non_camel_case_types, unused_must_use)]
#[cfg(bench)]
extern crate extra;
extern crate time;
pub use bag::{Bag, IntoBag};
pub use entity::{Entity, EntityRef, EntityRefMut, EntityBuilder, EntityManager};
pub use comp::{Component, ComponentType, ComponentManager};
pub use world::World;
pub use processor::{ProcessorManager, Processor, Aspect, IntervalProcessor};
/// A debug macro to use internally
macro_rules! debug(
    ($($value:expr),+) => (
        if cfg!(debug) {
            println!($($value),+)
        }
    )
);
/// Constructs a vtable from a struct and a trait
macro_rules! vtable(
    ($ty:ty as $trait_ty:ty) => (
        unsafe {
            let value = box mem::uninitialized::<$ty>() as Box<$trait_ty>;
            let value_obj:TraitObject = mem::transmute(&*value);
            value_obj.vtable
        }
    );
);

macro_rules! cast(
    ($value: expr, $ty:ty, $vtable: expr) => (
        unsafe {
            let value:TraitObject = TraitObject {
                data: mem::transmute($value),
                vtable: $vtable
            };
            mem::transmute::<_, &mut $ty>(value)
        }
    );
);

mod bag;
mod comp;
mod entity;
mod processor;
mod world;
mod util;
#[macro_escape]
mod macros;
#[cfg(test)]
mod test;