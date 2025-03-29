use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}

unsafe impl<T> Send for Guard<'_, T> where T: Send {}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety:This Guard safely guarantees
        // we've exclusively locked the lock.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety:This Guard safely guarantees
        // we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> Guard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }

    /// Safety: The &mut T from lock() should be dropped!
    /// (Don't keep a reference to T around!)
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

pub fn run_spin_lock() {
    let l: SpinLock<Vec<_>> = SpinLock::new(Vec::<u32>::new());
    std::thread::scope(|s| {
        s.spawn(|| l.lock().push(1));
        s.spawn(|| {
            let mut g = l.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = l.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
