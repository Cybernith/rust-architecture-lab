# Design: Order Book (Matching Engine Skeleton)

## 1. Domain Model

**Side**

- `Buy` or `Sell`
- Determines which side of the book the order goes to and how matching works.

**OrderId**

- Newtype: `struct OrderId(u64)`
- Unique per order.

**Price & Quantity**

- `Price = i64`
- `Quantity = i64`
- Both are integer to avoid floating point issues.

**Order**

```rust
struct Order {
    id: OrderId,
    side: Side,
    price: Price,
    quantity: Quantity,
}
```

**Trade**

```rust
struct Trade {
    buy_id: OrderId,
    sell_id: OrderId,
    price: Price,
    quantity: Quantity,
}
```

---

## 2. OrderBook Structure

```rust
struct OrderBook {
    bids: Vec<Order>, // sorted: highest price first
    asks: Vec<Order>, // sorted: lowest price first
}
```

For this lab we use `Vec<Order>` and maintain sorting manually.
We care about readability and domain clarity more than micro-optimizations.

---

## 3. Core Operations

**Initialization**

- `OrderBook::new() -> OrderBook`

**Adding an Order**

- `add_order(order: Order) -> Vec<Trade>`

Matching logic:

- For a **Buy** order:
  - While:
    - there is at least one ask
    - `buy_price >= best_ask_price`
    - we still have quantity to fill
  - We:
    - execute trades against the **best ask** (index 0)
    - reduce quantities
    - remove fully filled asks

- For a **Sell** order:
  - Mirror the logic with `bids`:
    - match with best bids where `sell_price <= best_bid_price`
    - reduce / remove as needed

Any remaining quantity after matching is inserted back into `bids` or `asks` in sorted order.

---

## 4. Sorting & Priority

**Bids:**

- Sorted by price **descending** (highest first).

**Asks:**

- Sorted by price **ascending** (lowest first).

Insertion helpers:

- `insert_bid(order)`
- `insert_ask(order)`

These ensure the `Vec` remains sorted after every insertion.

---

## 5. Testing Strategy

We test:

1. **Add without match:**
   - Add a buy order to an empty book → no trades, 1 bid.

2. **Exact match:**
   - Ask: price 100, qty 10
   - Buy: price 100, qty 10
   - Expect 1 trade at price 100 qty 10
   - Book becomes empty.

3. **Partial fill:**
   - Ask: price 100, qty 10
   - Buy: price 100, qty 5
   - Expect 1 trade qty 5
   - Remaining ask with qty 5.

4. **Better price levels:**
   - Multiple asks at different prices.
   - Buy should first match against the **lowest price asks**.

---

## 6. Extensibility

Future directions:

- Time priority (FIFO) at same price level.
- Better data structures (`BTreeMap<Price, Vec<Order>>`) for performance.
- Support for market orders.
- Persistence & replay of all orders/trades.

This lab is mainly about demonstrating a clean, extensible domain and reasonable matching logic.

---

**Author:** Soroosh Morshedi (Cybernith)  
**Website:** https://sorooshmorshedi.ir