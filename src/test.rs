use std::arch::asm;
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread::{scope, spawn};
use std::time::Duration;

pub fn t() -> usize {
    let n_threads = 5;
    let x: AtomicUsize = AtomicUsize::new(0);
    scope(|s| {
        for _ in 0..n_threads {
            s.spawn(|| {
                let v = unsafe { *x.inner.get() }.wrapping_add(1);
                x.store(v);
                // dbg!("VAL -> {:?}", unsafe{ *x.inner.get() });
            });
        }
    });
    let v = unsafe { *x.inner.get() };
    assert_eq!(v, n_threads);
    v
}
pub fn reader(a: &AtomicUsize) {
    while a.load() == 0 {}
    dbg!("A: {:?}", unsafe { *a.inner.get() });
}

pub fn writer(a: &AtomicUsize, v: usize) {
    a.store(v);
}

pub fn run_thread() {
    let shared_atomic = Arc::new(AtomicUsize::new(0));
    let shared_atomic_clone = shared_atomic.clone();
    let t_handle = spawn(move || reader(&shared_atomic_clone));
    std::thread::sleep(Duration::from_millis(50));
    writer(&shared_atomic, 1);
    t_handle.join().unwrap()
}

#[derive(Debug)]
pub struct AtomicUsize {
    inner: UnsafeCell<usize>,
}

unsafe impl Sync for AtomicUsize {}
unsafe impl Send for AtomicUsize {}

impl AtomicUsize {
    pub const fn new(v: usize) -> Self {
        Self {
            inner: UnsafeCell::new(v),
        }
    }

    pub fn load(&self) -> usize {
        unsafe { *self.inner.get() }
    }

    pub fn store(&self, v: usize) {
        unsafe {
            asm!(
                "lock; xchg [{address}], {v}",
                address = in(reg) self.inner.get(),
                v = in(reg) v
            );
        }
    }
}
