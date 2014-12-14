use std::any::Any;
use std::mem;
use std::raw::{Slice, TraitObject};
use std::ops::{Index, IndexMut};
use std::iter::FromIterator;
use std::iter;

/// Collection type a bit like Vec that avoids generics yet is still type-safe
/// 
/// It has a vector of bytes and allocates values to it on-the-fly.
/// It stores the vtable of the type so it can drop values when they are removed.
/// 
/// # Examples
///
/// ```
/// let mut bag = Bag::new();
/// bag.add(1i);
/// bag.add(2i);
/// assert_eq!(vec.len(), 2);
/// assert_eq!(vec[0], 1);
/// ```
///
/// There is a `bag!` macro to streamline bag initilisation. You can use it like so:
/// ```
/// let bag = bag![1u, 2u, 3u, 5u, 7u, 11u];
/// assert_eq(bag.len(), 6);
/// ```
pub struct Bag {
    vtable: *mut (),
    list: Vec<i8>,
    size: uint
}

impl<T:'static> Index<uint, T> for Bag {
    #[inline(always)]
    fn index<'a>(&'a self, &index: &uint) -> &'a T {
        debug_assert_eq!(self.size, mem::size_of::<T>());
        &self.as_slice()[index]
    }
}
impl<T:'static> IndexMut<uint, T> for Bag {
    #[inline(always)]
    fn index_mut<'a>(&'a mut self, &index: &uint) -> &'a mut T {
        debug_assert_eq!(self.size, mem::size_of::<T>());
        &mut self.as_mut_slice()[index]
    }
}
impl<T:'static> FromIterator<T> for Bag {
    fn from_iter<I: Iterator<T>>(iterator: I) -> Bag {
        let (size, _) = iterator.size_hint();
        let mut bag = Bag::with_capacity::<T>(size);
        bag.list.grow(bag.size * size, 0);
        for (index, item) in iterator.enumerate() {
            bag[index] = item;
        }
        bag
    }
}
impl Bag {
    #[inline(always)]
    /// Make a new bag of items
    pub fn new<T:'static>() -> Bag {
        Bag::with_capacity::<T>(0)
    }
    /// Make a bag of items allocated to allow capacity of `T` before
    /// requiring re-allocation.
    pub fn with_capacity<T:'static>(capacity: uint) -> Bag {
        let size = mem::size_of::<T>();
        Bag {
            vtable: vtable!(T as Any),
            list: Vec::with_capacity(capacity * size),
            size: size
        }
    }

    /// Construct a bag by filling it with `length` copies of `value`
    pub fn from_elem<T:'static + Clone>(length: uint, value: T) -> Bag {
        let mut bag = Bag::with_capacity::<T>(length);
        for index in iter::range(0, length) {
            bag[index] = value.clone();
        }
        bag
    }
    /// Get the copy of the value at a given index
    pub fn get<T:Copy+'static>(&self, index: uint) -> T {
        self.as_slice()[index]
    }
    #[inline(always)]
    /// Get the maximum number of items this bag can hold before reallocating
    pub fn capacity(&self) -> uint {
        self.list.capacity() / self.size
    }

    #[inline(always)]
    /// Get the number of items in this bag
    pub fn len(&self) -> uint {
        self.list.len() / self.size
    }

    #[inline(always)]
    /// Check if this bag is empty
    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    /// Clear this bag of all the items in it
    pub fn clear(&mut self) {
        debug!("Clearing bag");
        for index in range(0, self.len()) {
            let val = cast!(&mut self.list[index * self.size], Any, self.vtable);
            mem::drop(val);
        }
        self.list.clear();
    }

    #[inline(always)]
    /// Return this bag as a slice
    pub fn as_slice<T:'static>(&self) -> &[T] {
        unsafe {
            mem::transmute(Slice {
                data: self.list.as_ptr(),
                len: self.len()
            })
        }
    }

    #[inline]
    /// Return this bag as a mutable slice
    fn as_mut_slice<T:'static>(&mut self) -> &mut [T] {
        unsafe {
            mem::transmute(Slice {
                data: self.list.as_ptr(),
                len: self.len()
            })
        }
    }
    fn allocate_index(&mut self) -> uint {
        let index = self.list.len();
        self.list.grow(self.size, 0);
        index / self.size
    }
    /// Add a component to the bag
    pub fn add<T:'static>(&mut self, component: T) -> uint {
        debug_assert_eq!(self.size, mem::size_of::<T>());
        let index = self.allocate_index();
        self[index] = component;
        index
    }
    /// Convert the  bag into a vector of items
    pub fn into_vec<T:'static>(self) -> Vec<T> {
        debug_assert_eq!(self.size, mem::size_of::<T>());
        unsafe {
            let pointer: *mut T = mem::transmute(self.list.as_ptr());
            Vec::from_raw_parts(pointer, self.list.len() / self.size, self.list.capacity())
        }
    }
}
impl Drop for Bag {
    fn drop(&mut self) {
        self.clear();
    }
}

/// Provides efficient conversion into a bag
pub trait IntoBag {
    /// Convert this value into a bag
    fn into_bag(self) -> Bag;
}
impl<T:'static> IntoBag for Vec<T> {
    fn into_bag(self) -> Bag {
        let size = mem::size_of::<T>();
        let vec = unsafe {
            Vec::from_raw_parts(mem::transmute(self.as_ptr()), self.len() * size, self.capacity() * size)
        };
        unsafe {
            mem::forget(self);
        }
        Bag {
            vtable: vtable!(T as Any),
            list: vec,
            size: size
        }
    }
}
impl<T:'static> IntoBag for Box<[T]> {
    fn into_bag(self) -> Bag {
        let size = mem::size_of::<T>();
        let vec = unsafe {
            Vec::from_raw_parts(mem::transmute(self.as_ptr()), self.len() * size, self.len() * size)
        };
        unsafe {
            mem::forget(self);
        }
        Bag {
            vtable: vtable!(T as Any),
            list: vec,
            size: size
        }
    }
}