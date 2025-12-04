use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

pub struct Mutex<T> {
    state: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Mutex {
            state: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    fn lock(&self) -> Option<T> {
        None
        
    }
}
