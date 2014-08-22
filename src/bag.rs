use std::collections::treemap::{TreeMap, Entries};
use std::collections::{Collection, Mutable, Set, MutableSet, MutableMap, Map};
use std::hash::{Hash, hash};
use IMap = std::iter::Map;
use std::fmt::{Show, Formatter, Result};
/// A unique set of unordered items
pub struct Bag<T> {
	map: TreeMap<u64, T>
}
impl<T:Hash> Bag<T> {
    /// Make a new set
	pub fn new() -> Bag<T> {
		Bag {
			map: TreeMap::new()
		}
	}
    /// Iterate through the set
	pub fn iter(&self) -> IMap<'static, (&u64, &T), &T, Entries<u64, T>> {
		self.map.values()
	}
}
impl<T> Collection for Bag<T> {
	#[inline(always)]
	fn len(&self) -> uint {
		self.map.len()
	}
	#[inline(always)]
	fn is_empty(&self) -> bool {
		self.map.is_empty()
	}
}
impl<T> Mutable for Bag<T> {
    #[inline]
    fn clear(&mut self) {
    	self.map.clear()
    }
}
impl<T:Hash> Set<T> for Bag<T> {
    #[inline]
    fn contains(&self, value: &T) -> bool {
        self.map.contains_key(&hash(value))
    }
    fn is_disjoint(&self, _:&Bag<T>) -> bool {
    	false
    }
    fn is_subset(&self, _:&Bag<T>) -> bool {
    	false
    }
}
impl<T:Hash> MutableSet<T> for Bag<T> {
    #[inline]
    fn insert(&mut self, value: T) -> bool {
    	self.map.insert(hash(&value), value)
    }
    #[inline]
    fn remove(&mut self, value: &T) -> bool {
    	self.map.remove(&hash(value))
    }
}
impl<T:Hash + Show> Show for Bag<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        try!(write!(f, "{{"));
        for (i, x) in self.iter().enumerate() {
            if i != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", *x));
        }
        write!(f, "}}")
    }
}