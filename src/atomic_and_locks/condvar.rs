use crate::atomic_and_locks::{Mutex, MutexGuard};
use atomic_wait::{wait, wake_all, wake_one};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering::Relaxed};

pub struct CondVar {
    counter: AtomicU32,
    num_waiter: AtomicUsize,
}

impl CondVar {
    fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
            num_waiter: AtomicUsize::new(0),
        }
    }

    pub fn notify_one(&self) {
        if self.num_waiter.load(Relaxed) > 0 {
            self.counter.fetch_add(1, Relaxed);
            wake_one(&self.counter);
        }
    }

    pub fn notify_all(&self) {
        if self.num_waiter.load(Relaxed) > 0 {
            self.counter.fetch_add(1, Relaxed);
            wake_all(&self.counter);
        }
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        self.num_waiter.fetch_add(1, Relaxed);
        let counter_val = self.counter.load(Relaxed);
        let mutex = guard.mutex;
        drop(guard);
        wait(&self.counter, counter_val);
        self.num_waiter.fetch_sub(1, Relaxed);
        mutex.lock()
    }
}

#[test]
fn test_condvar() {
    let mutex = Mutex::new(0);
    let condvar = CondVar::new();

    let mut wakeups = 0;

    std::thread::scope(|s| {
        s.spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(1));
            *mutex.lock() = 123;
            condvar.notify_one();
        });

        let mut m = mutex.lock();
        while *m < 100 {
            m = condvar.wait(m);
            wakeups += 1
        }
        assert_eq!(*m, 123)
    });
    assert!(wakeups < 10);
}
