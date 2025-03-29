use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    // Panics when trying to send more than one message.
    pub unsafe fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("Can't send more than one message!")
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Release);
    }

    // Will panic if called when no messages
    // are available, or after the messages are consumed!
    // Tip: Use is_ready to check first!
    pub unsafe fn recieve(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("No messages available!")
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

pub fn run_channels() {
    let channel: Channel<&str> = Channel::new();
    let t = std::thread::current();
    std::thread::scope(|s| {
        s.spawn(|| {
            unsafe {
                channel.send("Hey there friend!");
            };
            t.unpark()
        });
        while !channel.is_ready() {
            std::thread::park();
        }
        assert_eq!(unsafe { channel.recieve() }, "Hey there friend!");
    });
}
