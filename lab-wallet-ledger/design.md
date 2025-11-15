# Design: Wallet Ledger

## 1. Domain Concepts

**Money**

- Internal type: `Money = i64`
- Represents integer units (e.g. cents).
- No floating point.

**WalletId**

- Newtype wrapper: `struct WalletId(String)`
- Makes the intent explicit and avoids mixing raw strings everywhere.

**WalletEvent**

- `Deposited { amount: Money }`
- `Withdrawn { amount: Money }`

Each event is an append-only record in the wallet’s event stream.

**Wallet**

- Fields:
  - `id: WalletId`
  - `events: Vec<WalletEvent>`
  - `balance: Money`
- Responsibilities:
  - Validate and apply business rules.
  - Maintain an always-correct `balance`.
  - Keep a full event history.

Methods:

- `new(id: WalletId) -> Wallet`
- `balance(&self) -> Money`
- `events(&self) -> &[WalletEvent]`
- `deposit(&mut self, amount: Money) -> Result<(), DomainError>`
- `withdraw(&mut self, amount: Money) -> Result<(), DomainError>`

Internally:

- `apply_event(&mut self, event: WalletEvent)` updates both:
  - `balance`
  - `events`

---

## 2. Errors and Invariants

**DomainError**

- `WalletNotFound(WalletId)`
- `InsufficientFunds { balance, attempted }`
- `InvalidAmount(Money)`

Invariants enforced by the domain:

- Deposit:
  - `amount > 0`
- Withdraw:
  - `amount > 0`
  - `balance >= amount` (no overdraft)

The service layer should **never silently ignore** invalid inputs — everything is expressed via `Result<_, DomainError>`.

---

## 3. Service Layer Design

**WalletService**

- Acts as an in-memory “repository + façade” over wallets.

Internal storage:

- `HashMap<WalletId, Wallet>`

Public API:

- `new() -> WalletService`
- `create_wallet(id: WalletId) -> &Wallet`
- `deposit(&mut self, id: &WalletId, amount: Money) -> Result<(), DomainError>`
- `withdraw(&mut self, id: &WalletId, amount: Money) -> Result<(), DomainError>`
- `balance(&self, id: &WalletId) -> Result<Money, DomainError>`
- `events(&self, id: &WalletId) -> Result<&[WalletEvent], DomainError>`

Error mapping:

- When a wallet id is unknown, we return `DomainError::WalletNotFound`.

---

## 4. Testing Strategy

We will test:

1. **Happy path:**

   - Create wallet
   - Deposit multiple times
   - Withdraw valid amount
   - Check final balance
   - Check event count and order

2. **Over-withdrawal:**

   - Deposit 500
   - Try to withdraw 1000 → error
   - Balance must stay 500

3. **Invalid amount:**

   - `deposit(0)` → InvalidAmount
   - `withdraw(-10)` → InvalidAmount
   - No events added when invalid.

---

## 5. Extensibility

Future extensions this design supports easily:

- Persistence layer (database / file) wrapping the event stream.
- Idempotency keys on events.
- Snapshots to speed up reconstruction.
- Multiple currency support by wrapping `Money` in a richer type.
- Adding a transaction type (`Transfer`) built on top of two wallets and two event streams.

The point of this lab is to show a **clear, explicit domain model** that doesn’t hide logic
in helpers or “magic” framework features.
