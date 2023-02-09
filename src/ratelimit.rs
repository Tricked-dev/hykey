use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use parking_lot::Mutex;

// API keys currently have a default rate limit of 60 requests per minute. We are also currently undergoing a transition period that it is worth being aware of, you can read more about this here.

// Endpoints which require the use of an API key will also respond with headers to assist with managing the rate limit:

// 'RateLimit-Limit' - The limit of requests per minute for the provided API key.
// 'RateLimit-Remaining' - The remaining amount of requests allowed for the current minute.
// 'RateLimit-Reset' - The amount of seconds until the next minute and the reset of the API key usages.
// If you require a higher limit please contact us via our support desk with an explanation for the increased limit and your Minecraft Account UUID. Please note that we do not give increased limits for future expansion, please only request an increase once you need it.

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
