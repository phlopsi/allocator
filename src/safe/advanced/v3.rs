#[cfg(test)]
mod tests;

use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::Ordering::SeqCst;
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
        let mut index;
        let mut slot_lock_result;

        while {
            index = free.load(SeqCst);
            assert_ne!(INVALID_INDEX, index, "out of reserved memory");
            slot_lock_result = storage[index as usize].inner.try_lock();
            slot_lock_result.is_err()
        } {}

        let mut slot_guard = slot_lock_result.unwrap();

        let next_free = match slot_guard.deref() {
            SlotInner::Empty(n) => *n,
            SlotInner::Filled(_) => unreachable!(),
        };

        free.store(next_free, SeqCst);
        *slot_guard = SlotInner::Filled(value);

        Box {
            free,
            index,
            inner: slot_guard,
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
        let Self { inner, free, index } = self;

        free.fetch_update(SeqCst, SeqCst, |prev_index| {
            **inner = SlotInner::Empty(prev_index);
            Some(*index)
        })
        .unwrap();
    }
}
