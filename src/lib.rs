//! A token bucket rate limiter for rust which can be used by either a single
//! thread or shared across threads.

#![cfg_attr(feature = "cargo-clippy", deny(warnings))]

use std::sync::Mutex;
use std::time::{Duration, Instant};

pub mod multi_thread;
pub mod single_thread;

/// Maximum acceptable duration in this crate, which is `2^64` nanoseconds,
/// about 20 years.
pub const MAX_DURATION: Duration = Duration::from_nanos(u64::MAX);

/// A builder for a rate limiter.
#[derive(Clone)]
pub struct Builder {
    capacity: u64,
    quantum: u64,
    initial: Option<u64>,
    interval: Duration,
}

impl Builder {
    /// Creates a new Builder with the default config.
    pub fn new() -> Builder {
        Builder::default()
    }

    /// Build a single thread rate limiter.
    pub fn single_thread(self) -> crate::single_thread::Limiter {
        let Self {
            capacity,
            quantum,
            initial,
            interval,
        } = self;
        crate::single_thread::Limiter {
            capacity,
            quantum,
            available: initial.unwrap_or(capacity).min(capacity),
            interval,
            last_tick: Instant::now(),
        }
    }

    /// Build a simplest multi thread rate limiter.
    ///
    /// No wake ordering guarantees.
    pub fn multi_thread(self) -> crate::multi_thread::Limiter {
        let inner = self.single_thread();
        let inner = Mutex::new(inner);
        crate::multi_thread::Limiter { inner }
    }

    /// Sets the number of tokens to add per interval.
    ///
    /// Default value is 1
    pub fn quantum(mut self, quantum: u64) -> Self {
        assert!(quantum > 0);
        self.quantum = quantum;
        self
    }

    /// Sets the number of tokens that the bucket can hold.
    ///
    /// Default value is 1
    pub fn capacity(mut self, capacity: u64) -> Self {
        assert!(capacity > 0);
        self.capacity = capacity;
        self
    }

    /// Set the duration between token adds.
    ///
    /// Default value is `Duration::from_secs(1)`.
    pub fn interval(mut self, interval: Duration) -> Self {
        assert!(interval > Duration::from_secs(0));
        self.interval = interval;
        self
    }

    /// Set the available tokens in the beginning.
    ///
    /// Default value is `None`, which means same as `capacity`.
    pub fn initial(mut self, initial: Option<u64>) -> Self {
        self.initial = initial;
        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            capacity: 1,
            quantum: 1,
            interval: Duration::from_secs(1),
            initial: None,
        }
    }
}
