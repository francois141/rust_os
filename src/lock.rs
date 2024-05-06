use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    flag: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        Self {
            flag: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.flag.swap(true, Ordering::Acquire) {
            // TODO: We can probably first read non-atomically here
            while self.flag.load(Ordering::Relaxed) {}
        }
    }

    pub fn unlock(&self) {
        self.flag.store(false, Ordering::Release);
    }
}