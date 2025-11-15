use lab_circuit_breaker::breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerError};
use std::thread;
use std::time::Duration;

fn failing_op() -> Result<(), &'static str> {
    Err("boom")
}

fn ok_op() -> Result<&'static str, &'static str> {
    Ok("ok")
}

#[test]
fn opens_after_threshold_failures() {
    let config = CircuitBreakerConfig::new(3, Duration::from_millis(200));
    let mut cb = CircuitBreaker::new(config);

    assert!(cb.is_closed());

    let _ = cb.call(|| failing_op());
    let _ = cb.call(|| failing_op());
    assert!(cb.is_closed());

    let _ = cb.call(|| failing_op());
    assert!(cb.is_open());

    let result = cb.call(|| ok_op());
    assert!(matches!(result, Err(CircuitBreakerError::Open)));
}

#[test]
fn transitions_to_half_open_after_timeout_and_closes_on_success() {
    let config = CircuitBreakerConfig::new(1, Duration::from_millis(200));
    let mut cb = CircuitBreaker::new(config);

    let _ = cb.call(|| failing_op());
    assert!(cb.is_open());

    thread::sleep(Duration::from_millis(250));

    let res = cb.call(|| ok_op());
    assert!(res.is_ok());
    assert!(cb.is_closed());
}

#[test]
fn half_open_failure_goes_back_to_open() {
    let config = CircuitBreakerConfig::new(1, Duration::from_millis(100));
    let mut cb = CircuitBreaker::new(config);

    let _ = cb.call(|| failing_op());
    assert!(cb.is_open());

    thread::sleep(Duration::from_millis(150));

    let res = cb.call(|| failing_op());
    match res {
        Err(CircuitBreakerError::Inner(_)) => {}
        _ => panic!("expected inner error"),
    }

    assert!(cb.is_open());
}
