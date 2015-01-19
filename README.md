Eccles
======
Eccles is a simple implementation of an Entity Component System in Rust, inspired by `rust_ecs` and `ecs-rs`. It uses macros to make the component list fast and easy to access.

The macro
---------
You use the `world!` macro to construct a world to put entities, components and systems on. Its syntax is:

     world!{
        name: (The name of the world),
        components: {
            (list name for using as a field on the world) => (component type),
            positions => Position
        },
        processors: {
            (processor name for using as a field on the world) => (processor type with Default impl) for [(list names to give to the processors)],
            render => Render for [sprite, pos]
        }
     }