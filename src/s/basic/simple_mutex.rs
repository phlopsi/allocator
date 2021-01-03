#[cfg(test)]
mod tests;

use simple_mutex::Mutex;
use simple_mutex::MutexGuard;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct Allocator<T> {
    storage: std::boxed::Box<[Mutex<Option<T>>]>,
}

impl<T> Allocator<T> {
    pub fn new(capacity: usize) -> Self {
        let mut storage = Vec::with_capacity(capacity);
        storage.resize_with(capacity, Default::default);

        Self {
            storage: storage.into_boxed_slice(),
        }
    }

    #[track_caller]
    pub fn box_it(&self, value: T) -> Box<'_, T> {
        let mut guard = self
            .storage
            .iter()
            .find_map(|mutex| mutex.try_lock())
            .expect("out of reserved memory");

        *guard = Some(value);

        Box { inner: guard }
    }
}

pub struct Box<'a, T> {
    inner: MutexGuard<'a, Option<T>>,
}

impl<T> Deref for Box<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.inner.deref() {
            Some(value) => value,
            None => unreachable!(),
        }
    }
}

impl<T> DerefMut for Box<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.inner.deref_mut() {
            Some(value) => value,
            None => unreachable!(),
        }
    }
}

impl<T> Drop for Box<'_, T> {
    fn drop(&mut self) {
        *self.inner = None;
    }
}
