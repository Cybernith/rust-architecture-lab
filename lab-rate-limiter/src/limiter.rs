use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientId(pub String);

impl ClientId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

#[derive(Debug)]
pub struct TokenBucket {
    capacity: u32,
    tokens: f64,
    refill_per_sec: f64,
    last_refill: Instant,
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_per_sec: f64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_per_sec,
            last_refill: Instant::now(),
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill);
        let seconds = elapsed.as_secs_f64();
        let added = seconds * self.refill_per_sec;
        self.tokens = (self.tokens + added).min(self.capacity as f64);
        self.last_refill = now;
    }

    pub fn allow(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn remaining_tokens(&mut self) -> f64 {
        self.refill();
        self.tokens
    }
}

pub struct RateLimiter {
    buckets: HashMap<ClientId, TokenBucket>,
    capacity: u32,
    refill_per_sec: f64,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_per_sec: f64) -> Self {
        Self {
            buckets: HashMap::new(),
            capacity,
            refill_per_sec,
        }
    }

    pub fn allow(&mut self, client_id: &ClientId) -> bool {
        let bucket = self
            .buckets
            .entry(client_id.clone())
            .or_insert_with(|| TokenBucket::new(self.capacity, self.refill_per_sec));

        bucket.allow()
    }
}
