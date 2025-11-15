use lab_order_book::domain::{Order, OrderBook, OrderId, Quantity, Side};

fn mk_order(id: u64, side: Side, price: i64, qty: Quantity) -> Order {
    Order {
        id: OrderId(id),
        side,
        price,
        quantity: qty,
    }
}

#[test]
fn add_buy_without_match_creates_bid() {
    let mut book = OrderBook::new();
    let buy = mk_order(1, Side::Buy, 100, 10);

    let trades = book.add_order(buy);

    assert!(trades.is_empty());
    assert_eq!(book.bids.len(), 1);
    assert_eq!(book.asks.len(), 0);
    assert_eq!(book.bids[0].price, 100);
}

#[test]
fn exact_match_clears_book() {
    let mut book = OrderBook::new();

    let ask = mk_order(1, Side::Sell, 100, 10);
    book.add_order(ask);

    let buy = mk_order(2, Side::Buy, 100, 10);
    let trades = book.add_order(buy);

    assert_eq!(trades.len(), 1);
    let t = &trades[0];
    assert_eq!(t.price, 100);
    assert_eq!(t.quantity, 10);

    assert!(book.bids.is_empty());
    assert!(book.asks.is_empty());
}

#[test]
fn partial_fill_leaves_rest_in_book() {
    let mut book = OrderBook::new();

    let ask = mk_order(1, Side::Sell, 100, 10);
    book.add_order(ask);

    let buy = mk_order(2, Side::Buy, 100, 5);
    let trades = book.add_order(buy);

    assert_eq!(trades.len(), 1);
    let t = &trades[0];
    assert_eq!(t.quantity, 5);

    assert!(book.bids.is_empty());
    assert_eq!(book.asks.len(), 1);
    assert_eq!(book.asks[0].quantity, 5);
}

#[test]
fn best_price_priority_for_asks() {
    let mut book = OrderBook::new();

    book.add_order(mk_order(1, Side::Sell, 95, 5));
    book.add_order(mk_order(2, Side::Sell, 100, 5));

    let buy = mk_order(3, Side::Buy, 100, 7);
    let trades = book.add_order(buy);

    assert_eq!(trades.len(), 2);
    assert_eq!(trades[0].price, 95);
    assert_eq!(trades[0].quantity, 5);
    assert_eq!(trades[1].price, 100);
    assert_eq!(trades[1].quantity, 2);

    assert_eq!(book.asks.len(), 1);
    assert_eq!(book.asks[0].price, 100);
    assert_eq!(book.asks[0].quantity, 3);
}
