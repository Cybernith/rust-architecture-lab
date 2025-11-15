# Lab: Order Book (Matching Engine Skeleton)

## 1. Problem Statement

We want to model the **core domain of a limit order book**, similar to what
an exchange or trading engine would use.

The goal is **not** to build a full high-frequency engine, but to:

- define a clean domain model (orders, sides, trades)
- separate bids and asks with clear rules
- prepare the ground for matching logic (even if partially implemented)

This lab is about **state and ordering**, not persistence or networking.

---

## 2. Functional Requirements

- The system maintains an **order book** for a single market (one symbol).
- Incoming orders:
  - Have:
    - `id`
    - `side` (Buy / Sell)
    - `price`
    - `quantity`
  - Can be:
    - added to the book
    - matched partially or fully against existing orders
- The book maintains:
  - A list of **bids** (buy orders) — highest price has priority
  - A list of **asks** (sell orders) — lowest price has priority
- Matching:
  - When a buy order enters:
    - it can match against the **best asks** (lowest prices) if `buy_price >= ask_price`
  - When a sell order enters:
    - it can match against the **best bids** (highest prices) if `sell_price <= bid_price`
  - For this lab, we can start with a simplified but correct matching implementation.

---

## 3. Outputs

- When an order is added, the function returns a list of **trades** executed:

  - `Trade { buy_id, sell_id, price, quantity }`

- Remaining (unfilled) quantity can stay in the book.

---

## 4. Non-Functional Goals

- In-memory only, no database.
- Focus on:
  - Clear ordering rules (best bid / best ask)
  - Clean struct design for orders and trades
  - Code that expresses intent, not micro-optimizations

---

## 5. Why This Lab?

Order books are great for showcasing:

- priority structures
- matching rules
- domain invariants around price & quantity

Even with a simple implementation, the **model** itself is a strong signal about how
I approach complex, stateful domains.
---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir