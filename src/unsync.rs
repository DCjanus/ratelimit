//! Single-threaded rate limiter.

use std::time::{Duration, Instant};

use crate::utils::SaturatingCast;

/// Single-threaded rate limiter.
pub struct Limiter {
    interval_ms: u64,
    capacity: u64,
    quantum: u64,
    available: u64,
    last_tick: Instant,
}

impl Limiter {
    /// Create a new rate limiter instance.
    /// `capacity`: sets the number of tokens that bucket can hold.
    /// `quantum`: sets the number of tokens to add per interval.
    /// `interval_ms`: sets the duration between token adds, in milliseconds.
    ///
    /// # Note
    ///
    /// `interval_ms` must be greater than 0.
    /// `capacity` must be no less than `quantum`.
    pub fn new(capacity: u64, quantum: u64, interval_ms: u64) -> Limiter {
        assert!(interval_ms > 0);
        assert!(capacity >= quantum);
        Self {
            interval_ms,
            capacity,
            quantum,
            last_tick: Instant::now(),
            available: capacity,
        }
    }

    /// Returns `Ok` if no wait required, returns `Err` if wait would block.
    pub fn try_wait_for(&mut self, count: u64) -> Result<(), ()> {
        let VirtualWait {
            last_tick,
            available,
            sleep_for,
        } = self.virtual_wait_for(count);
        if sleep_for > Duration::from_secs(0) {
            return Err(());
        }
        self.last_tick = last_tick;
        self.available = available;
        Ok(())
    }

    /// Blocking wait for `count` tokens.
    pub fn wait_for(&mut self, count: u64) {
        let VirtualWait {
            last_tick,
            available,
            sleep_for,
        } = self.virtual_wait_for(count);
        self.last_tick = last_tick;
        self.available = available;
        if sleep_for != Duration::from_secs(0) {
            std::thread::sleep(sleep_for);
        }
    }

    fn virtual_wait_for(&self, count: u64) -> VirtualWait {
        let mut last_tick = self.last_tick;
        let mut available = self.available;
        let now = Instant::now();

        let elapsed: u64 = (now - last_tick).as_millis().saturating_cast(); // pretty sure saturating_cast is ok for this
        let term = elapsed / self.interval_ms;
        last_tick += Duration::from_millis(self.interval_ms * term);
        available = self
            .quantum
            .saturating_mul(term)
            .saturating_add(available)
            .min(self.capacity);

        if available >= count {
            return VirtualWait {
                last_tick,
                sleep_for: Duration::from_secs(0),
                available: available - count,
            };
        }

        let remain = count - available;
        let mut term = remain / self.quantum;
        if remain % self.quantum != 0 {
            available = self.quantum - (remain % self.quantum);
            term += 1;
        } else {
            available = 0;
        }
        let sleep_for = Duration::from_millis(self.interval_ms.saturating_mul(term));
        last_tick += sleep_for;
        VirtualWait {
            last_tick,
            available,
            sleep_for,
        }
    }
}

struct VirtualWait {
    pub last_tick: Instant,
    pub available: u64,
    pub sleep_for: Duration,
}
