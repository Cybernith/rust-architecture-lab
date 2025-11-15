# Lab: Rate Limiter (Token Bucket)

## 1. Problem Statement

We want to design a **per-client rate limiter** that controls how many requests a client
can make over time.

The goal is to simulate a realistic building block used in APIs, gateways, and services:
a small, in-memory **Token Bucket**–based rate limiter.

---

## 2. Functional Requirements

- The system identifies each client by an ID (`ClientId`).
- For each client, we maintain a separate **token bucket**.
- Each bucket has:
  - a maximum capacity (max tokens)
  - a refill rate (tokens per second)
- Every time a client performs an action, we call:

  - `allow(client_id) -> bool`

- If a token is available:
  - we consume one token
  - return `true`
- If no token is available:
  - we return `false` (request is rejected / throttled)

---

## 3. Behavior Details

- Tokens replenish over time:
  - based on real elapsed time between calls
  - but never exceed the capacity
- The system should be able to handle:
  - bursts up to capacity
  - continuous rate close to `refill_per_sec` requests per second

---

## 4. Non-Functional Goals

- No external systems (no Redis, no DB) — purely in-memory for this lab.
- Focus on:
  - Clean modeling of time (`Instant`, `Duration`)
  - Correct refill logic
  - Separation between **per-client bucket** and **global limiter**.

---

## 5. Why This Lab?

Real systems often need **rate limiting** to:

- protect APIs from abuse
- enforce fair usage between tenants
- provide back-pressure

This lab shows how I think about:

- time-based state
- per-identity resource limits
- designing simple infrastructure components with Rust.
---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir