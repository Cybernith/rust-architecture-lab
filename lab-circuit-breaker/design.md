# Design: Circuit Breaker

## 1. State Machine

**States:**

- `Closed`:
  - All calls are allowed.
  - We count consecutive failures.
  - When `failures >= failure_threshold`, we transition to `Open`.

- `Open { opened_at }`:
  - Calls are rejected immediately.
  - If `now - opened_at >= open_timeout`, we transition to `HalfOpen`.

- `HalfOpen { attempts }`:
  - Only a limited number of "test" calls are allowed.
  - If a test call **succeeds**:
    - We reset counters and transition back to `Closed`.
  - If a test call **fails**:
    - We go back to `Open` and reset the timer.

For this lab, we keep the HalfOpen logic simple:

- First successful call in `HalfOpen` → immediately back to `Closed`.
- First failure in `HalfOpen` → back to `Open`.

---

## 2. Configuration and Types

```rust
struct CircuitBreakerConfig {
    failure_threshold: u32,
    open_timeout: Duration,
}
```

State:

```rust
enum BreakerState {
    Closed { failures: u32 },
    Open { opened_at: Instant },
    HalfOpen,
}
```

Main struct:

```rust
struct CircuitBreaker {
    state: BreakerState,
    config: CircuitBreakerConfig,
}
```

Generic error type:

```rust
enum CircuitBreakerError<E> {
    Open,
    Inner(E),
}
```

---

## 3. `call` API

Signature:

```rust
fn call<F, T, E>(&mut self, op: F) -> Result<T, CircuitBreakerError<E>>
where
    F: FnOnce() -> Result<T, E>;
```

Flow:

1. **Check state**:
   - If `Open`:
     - If timeout not passed → return `Err(CircuitBreakerError::Open)`.
     - If timeout passed → move to `HalfOpen` and continue.
2. **Execute operation**:
   - Run `op()`.
3. **On success**:
   - If `Closed`:
     - reset `failures = 0`.
   - If `HalfOpen`:
     - transition to `Closed` with `failures = 0`.
4. **On failure**:
   - If `Closed`:
     - increment failures; if `>= threshold` → move to `Open`.
   - If `HalfOpen`:
     - immediately go to `Open`.
   - Wrap downstream error as `CircuitBreakerError::Inner(e)`.

---

## 4. Testing Strategy

1. **Stays Closed for few failures**:
   - threshold = 3
   - 1–2 failures keep state in `Closed`.

2. **Transitions to Open after threshold**:
   - 3rd failure → state becomes `Open`.
   - Next call before timeout → `CircuitBreakerError::Open`.

3. **Transitions to HalfOpen then Closed on success**:
   - Put it in `Open`.
   - Wait for `open_timeout`.
   - Next call should execute and, if successful, return to `Closed`.

4. **HalfOpen failure goes back to Open**:
   - After timeout, in `HalfOpen`, a failed call should move breaker back to `Open`.

---

## 5. Extensibility

Possible future extensions:

- Track success rate instead of only consecutive failures.
- Add metrics: counts for opened/closed transitions, rejected calls.
- Different strategies per endpoint (configurable thresholds/timeouts).
- Async version (`async fn call` with `F: Future<Output = Result<T, E>>`).

The current design is intentionally compact but captures the essence of the Circuit Breaker pattern with a clear state machine and transition rules.

---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir