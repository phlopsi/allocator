#![allow(unused_imports)]

#[cfg(test)]
mod tests;

use crate::align128::Align128;
use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release, SeqCst};

const INVALID_INDEX: isize = -1;

#[derive(Debug)]
struct Slot<T> {
    next: Align128<AtomicIsize>,
    data: UnsafeCell<MaybeUninit<T>>,
}

impl<T> Slot<T> {
    fn empty(next: isize) -> Self {
        Self {
            next: Align128(AtomicIsize::new(next)),
            data: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }
}

#[derive(Debug)]
pub struct Allocator<T> {
    storage: std::boxed::Box<[Slot<T>]>,
    free: AtomicIsize,
}

unsafe impl<T: Debug> Sync for Allocator<T> {}

impl<T: Debug> Allocator<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(1 <= capacity, capacity <= (isize::MAX as usize));
        let mut storage = Vec::with_capacity(capacity);

        for next in 1..capacity {
            storage.push(Slot::empty(next as isize));
        }

        storage.push(Slot::empty(INVALID_INDEX));
        let storage = storage.into_boxed_slice();
        debug_assert!(capacity == storage.len());

        Self {
            storage,
            free: AtomicIsize::new(0),
        }
    }

    #[track_caller]
    pub fn box_it(&self, value: T) -> Box<'_, T> {
        let mut head = self.free.load(SeqCst);

        loop {
            let slot = match self.storage.get(head as usize) {
                Some(s) => s,
                None => {
                    dbg!(head, &self);
                    panic!("out of reserved memory")
                }
            };

            let next = slot.next.load(SeqCst);

            match self
                .free
                .compare_exchange_weak(head, next, SeqCst, SeqCst)
            {
                Ok(head) => {
                    // for debugging
                    slot.next.store(-2, SeqCst);
                    unsafe { &mut *slot.data.get() }.write(value);

                    return Box {
                        allocator: self,
                        index: head as usize,
                    };
                }
                Err(new_head) => {
                    head = new_head;
                }
            }
        }
    }

    unsafe fn get_ref(&self, index: usize) -> &T {
        (&*(self.storage.get_unchecked(index).data.get()
            as *const MaybeUninit<T>))
            .assume_init_ref()
    }

    #[allow(clippy::mut_from_ref)]
    unsafe fn get_mut(&self, index: usize) -> &mut T {
        (&mut *self.storage.get_unchecked(index).data.get())
            .assume_init_mut()
    }

    unsafe fn deallocate(&self, index: usize) {
        let mut head = self.free.load(SeqCst);

        loop {
            self.storage.get_unchecked(index).next.store(head, SeqCst);

            match self.free.compare_exchange_weak(
                head,
                index as isize,
                SeqCst,
                SeqCst,
            ) {
                Ok(_) => break,
                Err(new_head) => {
                    head = new_head;
                }
            }
        }
    }

    unsafe fn drop_in_place(&self, index: usize) {
        (&mut *self.storage.get_unchecked(index).data.get())
            .assume_init_drop();
    }
}

pub struct Box<'a, T: Debug> {
    allocator: &'a Allocator<T>,
    index: usize,
}

impl<T: Debug> Deref for Box<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { self.allocator.get_ref(self.index) }
    }
}

impl<T: Debug> DerefMut for Box<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { self.allocator.get_mut(self.index) }
    }
}

impl<T: Debug> Drop for Box<'_, T> {
    fn drop(&mut self) {
        unsafe { self.allocator.drop_in_place(self.index) };
        unsafe { self.allocator.deallocate(self.index) };
    }
}
