use lab_rate_limiter::limiter::{ClientId, RateLimiter};
use std::thread;
use std::time::Duration;

#[test]
fn rate_limiter_allows_up_to_capacity() {
    let mut limiter = RateLimiter::new(3, 1.0);
    let client = ClientId::new("client-1");

    assert!(limiter.allow(&client));
    assert!(limiter.allow(&client));
    assert!(limiter.allow(&client));
    assert!(!limiter.allow(&client));
}

#[test]
fn rate_limiter_refills_over_time() {
    let mut limiter = RateLimiter::new(2, 1.0);
    let client = ClientId::new("client-2");

    assert!(limiter.allow(&client));
    assert!(limiter.allow(&client));
    assert!(!limiter.allow(&client));

    thread::sleep(Duration::from_secs(2));
    assert!(limiter.allow(&client));
}

#[test]
fn separate_clients_have_separate_buckets() {
    let mut limiter = RateLimiter::new(1, 0.5);
    let a = ClientId::new("A");
    let b = ClientId::new("B");

    assert!(limiter.allow(&a));
    assert!(limiter.allow(&b));

    assert!(!limiter.allow(&a));
    assert!(!limiter.allow(&b));
}
