extern crate test;
use bag::{Bag, IntoBag};
use world::World;
use entity::{EntityRef, EntityRefMut};
use processor::{Aspect, Processor};

#[test]
#[should_fail]
fn test_out_of_bounds_bag() {
    let bag = bag![42u, 56, 22112, 3223];
    bag.get::<uint>(5);
}

#[test]
fn test_bag_index() {
    let bag = bag![121u, 12u];
    assert_eq!(121u, bag.get(0));
    assert_eq!(12u, bag.get(1));
}

#[test]
fn test_bag_length() {
    let mut bag = Bag::with_capacity::<uint>(3);
    assert_eq!(3, bag.capacity());
    assert_eq!(0, bag.len());
    bag.add(42u);
    assert_eq!(1, bag.len());
    bag.add(43u);
    assert_eq!(2, bag.len());
    bag.add(44u);
    assert_eq!(3, bag.len());
    assert_eq!(3, bag.capacity());
}
#[test]
fn test_bag_macro() {
    let ref bag = bag![3u, 10u, 32u];
    assert_eq!(bag.get(0), 3u);
    assert_eq!(bag.get(1), 10u);
    assert_eq!(bag.get(2), 32u);
}
#[test]
fn test_into_bag() {
    let bag: Bag = vec![1u, 2u, 3u].into_bag();
    assert_eq!(3, bag.len());
    assert_eq!(1u, bag.get(0));
    assert_eq!(2u, bag.get(1));
    assert_eq!(3u, bag.get(2));
    assert_eq!(bag.into_vec(), vec![1u, 2u, 3u]);
}

#[test]
fn test_bag_alloc() {
    let ref mut bag = Bag::with_capacity::<uint>(2);
    let bad_index = bag.add(304u);
    bag.add(204u);
    assert_eq!(bag.len(), 2);
    assert_eq!(bag.capacity(), 2);
    bag.add(205u);
    assert_eq!(bag.get(bad_index), 304u);
    assert_eq!(bag.len(), 3);
    assert_eq!(bag.capacity(), 3);
}