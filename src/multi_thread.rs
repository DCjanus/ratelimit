use std::sync::Mutex;

use crate::single_thread::Limiter as CoreLimiter;

/// The simplest multi-threaded rate limiter
///
/// No wake ordering guarantees.
///
/// # Example
///
/// ```ignore
/// use std::time::Duration;
/// use std::sync::Arc;
///
/// use ratelimit::Builder;
///
/// let limiter = Builder::new().capacity(10).quantum(5).interval(Duration::from_secs(1)).multi_thread();
/// let outer_limiter = Arc::new(limiter);
/// let inner_limiter = outer_limiter.clone();
///
/// let begin = std::time::Instant::now();
/// let inner = std::thread::spawn(move ||{
///     for _ in 0..10 {
///         inner_limiter.wait_for(1);
///     }
/// });
/// for _ in 0..10 {
///     outer_limiter.wait_for(1);
/// }
/// inner.join();
///
/// assert_eq!(begin.elapsed().as_secs(), 2);
///
/// ```
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
