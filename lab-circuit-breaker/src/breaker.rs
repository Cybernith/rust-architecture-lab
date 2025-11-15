use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub open_timeout: Duration,
}

impl CircuitBreakerConfig {
    pub fn new(failure_threshold: u32, open_timeout: Duration) -> Self {
        Self {
            failure_threshold,
            open_timeout,
        }
    }
}

#[derive(Debug)]
enum BreakerState {
    Closed { failures: u32 },
    Open { opened_at: Instant },
    HalfOpen,
}

#[derive(Debug)]
pub struct CircuitBreaker {
    state: BreakerState,
    config: CircuitBreakerConfig,
}

#[derive(Debug)]
pub enum CircuitBreakerError<E> {
    Open,
    Inner(E),
}

impl<E: std::fmt::Display> std::fmt::Display for CircuitBreakerError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CircuitBreakerError::Open => write!(f, "circuit breaker is open"),
            CircuitBreakerError::Inner(e) => write!(f, "inner error: {}", e),
        }
    }
}

impl<E: std::fmt::Debug + std::fmt::Display> std::error::Error for CircuitBreakerError<E> {}

impl CircuitBreaker {
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            state: BreakerState::Closed { failures: 0 },
            config,
        }
    }

    pub fn is_closed(&self) -> bool {
        matches!(self.state, BreakerState::Closed { .. })
    }

    pub fn is_open(&self) -> bool {
        matches!(self.state, BreakerState::Open { .. })
    }

    pub fn is_half_open(&self) -> bool {
        matches!(self.state, BreakerState::HalfOpen)
    }

    fn check_and_transition_from_open(&mut self) {
        if let BreakerState::Open { opened_at } = self.state {
            let now = Instant::now();
            if now.duration_since(opened_at) >= self.config.open_timeout {
                self.state = BreakerState::HalfOpen;
            }
        }
    }

    pub fn call<F, T, E>(&mut self, op: F) -> Result<T, CircuitBreakerError<E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match self.state {
            BreakerState::Open { .. } => {
                self.check_and_transition_from_open();

                if self.is_open() {
                    return Err(CircuitBreakerError::Open);
                }
            }
            _ => {}
        }

        let result = op();

        match result {
            Ok(value) => {
                match self.state {
                    BreakerState::Closed { .. } => {
                        self.state = BreakerState::Closed { failures: 0 };
                    }
                    BreakerState::HalfOpen => {
                        self.state = BreakerState::Closed { failures: 0 };
                    }
                    BreakerState::Open { .. } => {
                    }
                }
                Ok(value)
            }
            Err(e) => {
                match &mut self.state {
                    BreakerState::Closed { failures } => {
                        *failures += 1;
                        if *failures >= self.config.failure_threshold {
                            self.state = BreakerState::Open {
                                opened_at: Instant::now(),
                            };
                        }
                    }
                    BreakerState::HalfOpen => {
                        self.state = BreakerState::Open {
                            opened_at: Instant::now(),
                        };
                    }
                    BreakerState::Open { .. } => {
                    }
                }

                Err(CircuitBreakerError::Inner(e))
            }
        }
    }
}
