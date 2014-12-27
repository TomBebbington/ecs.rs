/// Efficiently construct a bag with the given values in
#[macro_export]
macro_rules! bag[
    ($($value:expr),*) => ({
        use std::slice::BoxedSlice;
        let xs: ::std::boxed::Box<[_]> = box [$($value),*];
        xs.into_bag()
    });
];

/// Create a new entity with the given components in
#[macro_export]
macro_rules! entity(
    [$($component:ty),*] => (
        |&mut:entity:EntityRefMut| {
            entity$(.add_default::<$component>())+;
        }
    );
    [$($component:expr),*] => (
        |&mut:entity:EntityRefMut| {
            entity$(.add($component))+;
        }
    );
);

/// Define a component with the given fields and default values
#[macro_export]
macro_rules! comp(
    ($name:ident = {$($field:ident: $ty:ty),+}) => (
        #[deriving(Copy, Clone)]
        pub struct $name {
            $(pub $field: $ty),+
        }
    );
    ($name:ident = {$($field:ident: $ty:ty = $value:expr),+}) => (
        #[deriving(Copy, Clone)]
        pub struct $name {
            $(pub $field: $ty),+
        }
        impl ::std::default::Default for $name {
            fn default() -> $name {
                $name {
                    $($field: $value),+
                }
            }
        }
    );
);

/// Make an aspect from the given filter
#[macro_export]
macro_rules! aspect(
    ($($comp:ty)&+) => (
        box |mut entity:EntityRef| -> bool {
            $(entity.has::<$comp>())&&+
        }
    );
    ($($comp:ty)|+) => (
        box |mut entity:EntityRef| -> bool {
            $(entity.has::<$comp>())||+
        }
    );
);

/// Make a world from the given components and processors
#[macro_export]
macro_rules! world{
    () => (World::new());
    (components: [$($comp:ty),+]) => ({
        let mut world = World::new();
        $(world.register_component::<$comp>();)+
        world 
    });
    (components: [$($comp:ty),+], processors: [$($processor:expr for $aspect:expr),+]) => ({
        let mut world = World::new();
        $(world.register_component::<$comp>();)+
        $(world.register_processor(box $processor, $aspect);)+
        world 
    });
    (comp: [$($comp:ty),+], proc: [$($processor:expr for $aspect:expr),+]) => ({
        let mut world = World::new();
        $(world.register_component::<$comp>();)+
        $(world.register_processor(box $processor, $aspect);)+
        world 
    });
}