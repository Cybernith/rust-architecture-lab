#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrderId(pub u64);

pub type Price = i64;
pub type Quantity = i64;

#[derive(Debug, Clone)]
pub struct Order {
    pub id: OrderId,
    pub side: Side,
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trade {
    pub buy_id: OrderId,
    pub sell_id: OrderId,
    pub price: Price,
    pub quantity: Quantity,
}

#[derive(Debug, Default)]
pub struct OrderBook {
    pub bids: Vec<Order>, // highest price first
    pub asks: Vec<Order>, // lowest price first
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Vec<Trade> {
        match order.side {
            Side::Buy => self.add_buy(order),
            Side::Sell => self.add_sell(order),
        }
    }

    fn add_buy(&mut self, mut buy: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        while buy.quantity > 0 {
            if let Some(best_ask) = self.asks.first_mut() {
                if buy.price < best_ask.price {
                    break;
                }

                let trade_qty = buy.quantity.min(best_ask.quantity);
                trades.push(Trade {
                    buy_id: buy.id,
                    sell_id: best_ask.id,
                    price: best_ask.price,
                    quantity: trade_qty,
                });

                buy.quantity -= trade_qty;
                best_ask.quantity -= trade_qty;

                if best_ask.quantity == 0 {
                    self.asks.remove(0);
                }
            } else {
                break;
            }
        }

        if buy.quantity > 0 {
            self.insert_bid(buy);
        }

        trades
    }

    fn add_sell(&mut self, mut sell: Order) -> Vec<Trade> {
        let mut trades = Vec::new();

        while sell.quantity > 0 {
            if let Some(best_bid) = self.bids.first_mut() {
                if sell.price > best_bid.price {
                    break;
                }

                let trade_qty = sell.quantity.min(best_bid.quantity);
                trades.push(Trade {
                    buy_id: best_bid.id,
                    sell_id: sell.id,
                    price: best_bid.price,
                    quantity: trade_qty,
                });

                sell.quantity -= trade_qty;
                best_bid.quantity -= trade_qty;

                if best_bid.quantity == 0 {
                    self.bids.remove(0);
                }
            } else {
                break;
            }
        }

        if sell.quantity > 0 {
            self.insert_ask(sell);
        }

        trades
    }

    fn insert_bid(&mut self, order: Order) {
        let pos = self
            .bids
            .iter()
            .position(|o| o.price < order.price)
            .unwrap_or(self.bids.len());
        self.bids.insert(pos, order);
    }

    fn insert_ask(&mut self, order: Order) {
        let pos = self
            .asks
            .iter()
            .position(|o| o.price > order.price)
            .unwrap_or(self.asks.len());
        self.asks.insert(pos, order);
    }
}
