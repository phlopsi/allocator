#[cfg(test)]
mod tests;

use crate::align128::Align128;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{
    AcqRel, Acquire, Relaxed, Release, SeqCst,
};

#[derive(Default)]
struct Mutex<T: ?Sized> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        if self
            .locked
            .compare_exchange(false, true, Acquire, Relaxed)
            .is_ok()
        {
            Some(MutexGuard { mutex: self })
        } else {
            std::process::abort()
        }
    }
}

#[must_use = "if unused the Mutex will immediately unlock"]
struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, Release);
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(self.mutex.value.get() as *const T) }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

pub struct Allocator<T> {
    storage: std::boxed::Box<[Align128<Mutex<MaybeUninit<T>>>]>,
    indices: parking_lot::Mutex<std::boxed::Box<[u16]>>,
}

impl<T> Allocator<T> {
    pub fn new(capacity: usize) -> Self {
        let mut storage = Vec::with_capacity(capacity);

        storage.resize_with(capacity, || {
            Align128(Mutex::new(MaybeUninit::uninit()))
        });

        let storage = storage.into_boxed_slice();

        let mut indices = Vec::with_capacity(capacity);
        indices.resize_with(capacity, Default::default);

        let indices =
            parking_lot::Mutex::new(indices.into_boxed_slice());

        Self { storage, indices }
    }

    #[track_caller]
    pub fn box_it(&self, value: T) -> Box<'_, T> {
        self.box_it_with_index(value, 0)
    }

    #[track_caller]
    fn box_it_with_index(&self, value: T, index: usize) -> Box<'_, T> {
        let mut guard = self
            .storage
            .iter()
            .cycle()
            .skip(index)
            .find_map(|mutex| mutex.try_lock())
            .unwrap();

        guard.write(value);

        Box { guard }
    }

    pub fn thread_local(&self) -> AllocatorRef<'_, T> {
        let mut indices = self.indices.lock();

        let (index, count) = indices
            .iter_mut()
            .enumerate()
            .min_by_key(|(_, count)| **count)
            .unwrap();

        *count += 1;

        AllocatorRef {
            allocator: self,
            index,
        }
    }
}

pub struct AllocatorRef<'allocator, T> {
    allocator: &'allocator Allocator<T>,
    index: usize,
}

impl<T> AllocatorRef<'_, T> {
    #[track_caller]
    pub fn box_it(&self, value: T) -> Box<'_, T> {
        self.allocator.box_it_with_index(value, self.index)
    }
}

impl<T> Drop for AllocatorRef<'_, T> {
    fn drop(&mut self) {
        self.allocator.indices.lock()[self.index] -= 1;
    }
}

pub struct Box<'guard, T> {
    guard: MutexGuard<'guard, MaybeUninit<T>>,
}

impl<T> Deref for Box<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.guard.assume_init_ref() }
    }
}

impl<T> DerefMut for Box<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.guard.assume_init_mut() }
    }
}

impl<T> Drop for Box<'_, T> {
    fn drop(&mut self) {
        unsafe { self.guard.assume_init_drop() };
    }
}
