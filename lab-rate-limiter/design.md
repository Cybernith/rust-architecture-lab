# Design: Rate Limiter (Token Bucket)

## 1. Core Model

**ClientId**

- Newtype: `struct ClientId(String)`
- Identifies each logical client (user, API key, IP, etc.).

**TokenBucket**

- Fields:
  - `capacity: u32` — maximum number of tokens
  - `tokens: f64` — current tokens (fractional for smoother refill)
  - `refill_per_sec: f64` — how many tokens to add per second
  - `last_refill: Instant` — when we last updated this bucket

- Methods:
  - `new(capacity, refill_per_sec) -> TokenBucket`
  - `refill(&mut self)`:
    - compute elapsed time
    - add `elapsed_seconds * refill_per_sec` to `tokens`
    - cap at `capacity`
    - update `last_refill`
  - `allow(&mut self) -> bool`:
    - call `refill()`
    - if `tokens >= 1.0` → decrement → `true`
    - else → `false`

Using `f64` for tokens avoids discrete “step” refill and allows smoother rate behavior.

---

## 2. Service Layer

**RateLimiter**

- Fields:
  - `buckets: HashMap<ClientId, TokenBucket>`
  - `capacity: u32`
  - `refill_per_sec: f64`

- Methods:
  - `new(capacity, refill_per_sec) -> RateLimiter`
  - `allow(&mut self, client_id: &ClientId) -> bool`:
    - `entry(client_id)` into `buckets`
    - if not present → create a new `TokenBucket` with global config
    - call `allow()` on that bucket

This keeps the per-client state local while sharing configuration for the whole limiter.

---

## 3. Testing Strategy

We will test:

1. **Burst up to capacity:**

   - capacity = 3, refill = 1/sec
   - call `allow` 3 times → expect `true, true, true`
   - 4th call → expect `false`

2. **Refill over time:**

   - after exhausting tokens
   - sleep 2 seconds
   - call `allow` again → expect `true` (bucket refilled)

3. **Multiple clients:**

   - client A and client B share same limiter
   - each one has an independent bucket
   - draining A’s bucket must not affect B’s tokens

---

## 4. Extensibility

From here, we can extend to:

- Different capacity/refill per client
- Multi-level limits:
  - per-client
  - per-IP
  - global system limit
- Pluggable time source (for deterministic tests)
- Persistence or distributed store (Redis etc.) for real-world setups

The lab focuses on **a clean, composable core** that illustrates how I design time-based control mechanisms.
---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir