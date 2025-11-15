# Lab: Circuit Breaker (Resilient Calls)

## 1. Problem Statement

We want to design a reusable **Circuit Breaker** component that protects a system
from repeatedly calling a failing dependency (e.g. external API, DB, microservice).

The goals:

- Model the classic **Closed → Open → Half-Open** state machine.
- Stop hammering a broken downstream service.
- Automatically probe and recover when things get healthy again.
- Make the component generic over any `Result<T, E>`-returning operation.

---

## 2. Functional Requirements

- States:
  - `Closed`:
    - All calls are allowed.
    - Failures are counted.
  - `Open`:
    - Calls are **rejected immediately** without touching the downstream.
    - After a timeout, the breaker moves into `HalfOpen`.
  - `HalfOpen`:
    - A limited number of test calls are allowed.
    - If a test call **succeeds**, the breaker goes back to `Closed`.
    - If a test call **fails**, the breaker goes back to `Open`.

- Configuration:
  - `failure_threshold`: how many consecutive failures trigger `Open`.
  - `open_timeout`: how long we stay in `Open` before trying `HalfOpen`.
  - `half_open_max_calls`: optional limit on how many trial calls are allowed (we will keep it simple in this lab).

- API:
  - `call(&mut self, op: impl FnOnce() -> Result<T, E>) -> Result<T, CircuitBreakerError<E>>`
  - If the breaker is `Open` and timeout not passed → return `CircuitBreakerError::Open`.
  - Otherwise → execute the operation and update state accordingly.

---

## 3. Non-Functional Goals

- Purely in-memory, no I/O in this lab.
- Focus on:
  - Clear state machine representation.
  - Time-based transitions using `Instant` and `Duration`.
  - Generic error handling: wrap downstream error type `E` in `CircuitBreakerError<E>`.

---

## 4. Why This Lab?

Circuit breakers are a core resilience pattern in distributed systems.

This lab shows:

- How I design a small but realistic **stateful component**.
- How I model **state transitions** with a clear, explicit enum.
- How I expose a clean, generic API around an unreliable dependency.

---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir