# Rust Architecture Lab 🦀
by `Soroosh Morshedi` 

This repository is a technical mirror of how I architect systems.

Every module here expresses a core software engineering principle:
financial consistency, rate limiting, matching engines, fault isolation, and more —
all distilled into small, executable Rust libraries.

I created this repo as a living portfolio of my engineering philosophy:
tight domain modeling, explicit invariants, deterministic behavior, and clean,
layered code that scales from prototype to production.

If you're reviewing my work for collaboration, hiring, or technical leadership,
start here - these labs represent exactly how I reason about systems.


that showcase how I think about:

- Domain modeling
- System design
- Resilience & infrastructure patterns
- Clean, testable code

Each folder is a standalone lab with:

- `problem.md` - problem framing and requirements
- `design.md` - domain & architecture design
- `src/` - implementation
- `tests/` - scenario-based tests

---

## Labs

### 1. `lab-wallet-ledger`

A minimal **event-sourced wallet ledger**:

- Multiple wallets identified by `WalletId`.
- Operations:
  - `deposit(amount)`
  - `withdraw(amount)`
- Invariants:
  - No negative balances
  - Amounts must be positive
- Every operation is stored as a `WalletEvent`.
- `WalletService` exposes an in-memory API to create wallets, move money, and query balance & events.

Use case: modeling financial flows and auditability in e-commerce / crypto systems.

---

### 2. `lab-rate-limiter`

A per-client **Token Bucket** rate limiter:

- Separate `TokenBucket` per `ClientId`.
- Configurable:
  - capacity
  - refill rate (tokens per second)
- API:
  - `allow(&client_id) -> bool`
- Uses real wall-clock time (`Instant`, `Duration`) to refill tokens.

Use case: API gateways, multi-tenant SaaS, abuse protection.

---

### 3. `lab-order-book`

A simplified **limit order book** domain:

- `Order { id, side, price, quantity }`
- `OrderBook` maintains:
  - `bids`: highest price first
  - `asks`: lowest price first
- Matching:
  - Buy orders match against best asks (`buy_price >= ask_price`)
  - Sell orders match against best bids (`sell_price <= bid_price`)
- Produces `Trade { buy_id, sell_id, price, quantity }`.

Use case: trading engines, matching logic, priority structures.

---

### 4. `lab-circuit-breaker`

A generic **Circuit Breaker** for resilient calls:

- Classic state machine:
  - `Closed` → `Open` → `HalfOpen`
- Config:
  - `failure_threshold`
  - `open_timeout`
- Generic API:
  - `call(&mut self, op: impl FnOnce() -> Result<T, E>) -> Result<T, CircuitBreakerError<E>>`
- Protects downstream dependencies from being hammered when they are unhealthy.

Use case: microservices, external APIs, databases, any unreliable IO.

---

## How to Run

From the workspace root:

```bash
# Run all tests
cargo test

# Or per lab
cargo test -p lab-wallet-ledger
cargo test -p lab-rate-limiter
cargo test -p lab-order-book
cargo test -p lab-circuit-breaker
```

---

## Tech & Style

- Rust 2021
- Strong separation of:
  - domain types
  - service layers
  - error models
- No external dependencies (by design) — pure standard library.

---

## Author

**Soroosh Morshedi (Cybernith)**  

- GitHub: [github.com/Cybernith](https://github.com/Cybernith)  
- Website: [sorooshmorshedi.ir](https://sorooshmorshedi.ir)