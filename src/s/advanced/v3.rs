#![allow(unused_imports)]

#[cfg(test)]
mod tests;

use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release, SeqCst};
use std::sync::Mutex;
use std::sync::MutexGuard;

const INVALID_INDEX: isize = -1;

enum SlotInner<T> {
    Filled(T),
    Empty(isize),
}

struct Slot<T> {
    inner: Mutex<SlotInner<T>>,
}

impl<T> Slot<T> {
    fn new(next_free_slot_index: isize) -> Self {
        Self {
            inner: Mutex::new(SlotInner::Empty(next_free_slot_index)),
        }
    }
}

pub struct Allocator<T> {
    storage: std::boxed::Box<[Slot<T>]>,
    free: std::sync::atomic::AtomicIsize,
}

impl<T> Allocator<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(1 <= capacity, capacity <= (isize::MAX as usize));
        let mut storage = Vec::with_capacity(capacity);

        for next_free_slot_index in 1..capacity {
            storage.push(Slot::new(next_free_slot_index as isize))
        }

        storage.push(Slot::new(INVALID_INDEX));
        let storage = storage.into_boxed_slice();
        debug_assert!(capacity == storage.len());

        Self {
            storage,
            free: std::sync::atomic::AtomicIsize::new(0),
        }
    }

    #[track_caller]
    pub fn box_it(&self, value: T) -> Box<'_, T> {
        let Self { storage, free } = &self;

        loop {
            let index = free.load(Acquire);

            match storage
                .get(index as usize)
                .expect("out of reserved memory")
                .inner
                .try_lock()
            {
                Ok(mut guard) => {
                    let next_free = match *guard {
                        SlotInner::Empty(n) => n,
                        SlotInner::Filled(_) => unreachable!(),
                    };

                    if free
                        .compare_exchange_weak(
                            index, next_free, AcqRel, Acquire,
                        )
                        .is_ok()
                    {
                        *guard = SlotInner::Filled(value);

                        return Box {
                            free,
                            index,
                            inner: guard,
                        };
                    }

                    std::sync::atomic::spin_loop_hint();
                }
                Err(std::sync::TryLockError::WouldBlock) => {
                    std::thread::yield_now();
                }
                Err(std::sync::TryLockError::Poisoned(e)) => {
                    panic!("{}", e)
                }
            }
        }
    }
}

pub struct Box<'a, T> {
    inner: MutexGuard<'a, SlotInner<T>>,
    free: &'a std::sync::atomic::AtomicIsize,
    index: isize,
}

impl<T> Deref for Box<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match self.inner.deref() {
            SlotInner::Filled(value) => value,
            SlotInner::Empty(_) => unreachable!(),
        }
    }
}

impl<T> DerefMut for Box<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.inner.deref_mut() {
            SlotInner::Filled(value) => value,
            SlotInner::Empty(_) => unreachable!(),
        }
    }
}

impl<T> Drop for Box<'_, T> {
    fn drop(&mut self) {
        *self.inner =
            SlotInner::Empty(self.free.swap(self.index, AcqRel));
    }
}
