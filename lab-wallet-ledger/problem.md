# Lab: Wallet Ledger (Event-Sourced Style)

## 1. Problem Statement

We want to design a small **wallet ledger** system that models how money moves in and out of user accounts.

The goal is not to build a full-blown bank, but to show how to:

- model a financial domain cleanly
- enforce invariants (no negative balances, valid amounts)
- keep a clear history of **all operations** (events)
- derive current state (balance) from past events

Think of it as a miniature version of a wallet service inside an e-commerce or crypto platform.

---

## 2. Functional Requirements

- The system manages multiple wallets, each identified by a **Wallet ID**.
- For each wallet, we must support:
  - `Deposit(amount)`
  - `Withdraw(amount)`
- Invariants:
  - Balance must **never go negative**.
  - Amount must be **strictly positive** for both deposit and withdraw.
- Every operation must be recorded as a **WalletEvent**, so we always have an audit trail.
- The current balance is **derived from events**, but we also keep an up-to-date value for fast reads.

---

## 3. API / Use Cases

At the service level, the lab should expose an in-memory API:

- `create_wallet(wallet_id)`
- `deposit(wallet_id, amount)`
- `withdraw(wallet_id, amount)`
- `get_balance(wallet_id) -> Money`
- `get_events(wallet_id) -> &[WalletEvent]`

---

## 4. Error Cases

We must handle:

- Unknown wallet:
  - `WalletNotFound`
- Insufficient funds:
  - Attempt to withdraw more than current balance
- Invalid amount:
  - Amount is `<= 0` for deposit or withdraw

These should be represented as domain-level errors, not just `Result<bool>`.

---

## 5. Non-Functional Goals

- No I/O, no database — keep everything **in-memory** for this lab.
- Code should be:
  - modular (domain, service, error separated)
  - testable (unit tests for flows and edge cases)
  - easy to extend (idempotency keys, snapshotting, persistence can be added later)

---

## 6. Why This Lab?

This lab is about showing how I think when I design **financial flows**:

- balances as a function of events
- explicit domain errors
- simple event-sourced style without framework magic
- clear boundary between domain and service layer
---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir