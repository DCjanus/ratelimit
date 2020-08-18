use std::sync::Mutex;

use crate::single_thread::Limiter as CoreLimiter;

/// The simplest multi-threaded rate limiter
///
/// No wake ordering guarantees.
pub struct Limiter {
    pub(crate) inner: Mutex<CoreLimiter>,
}

impl Limiter {
    pub fn wait_for(&self, count: u64) {
        self.inner.lock().unwrap().wait_for(count)
    }

    pub fn try_wait_for(&self, count: u64) -> Result<(), ()> {
        self.inner.try_lock().map_err(|_| ())?.try_wait_for(count)
    }
}
