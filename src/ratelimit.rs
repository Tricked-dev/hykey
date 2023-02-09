use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use parking_lot::Mutex;

#[derive(Clone)]
pub struct HySmartLimiter {
    pub remaining: Arc<AtomicU64>,
    pub reset: Arc<Mutex<Instant>>,
}

impl HySmartLimiter {
    pub fn new(limit: u32) -> Self {
        Self {
            remaining: Arc::new(AtomicU64::new(limit as u64)),
            reset: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub fn set_remaining(&self, remaining: u64) {
        self.remaining.store(remaining, Ordering::Relaxed);
    }
}
