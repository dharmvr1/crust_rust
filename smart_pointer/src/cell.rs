use std::{
    cell::{Cell, UnsafeCell},
    sync,
};

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}
// impl<T> !sync for Cell<T> {}
//  not sync  very important concept

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[test]
fn bad() {
    let x = Cell::new(3);
    let first = x.get();
    x.set(5);
    assert_eq!(5, first);
}
