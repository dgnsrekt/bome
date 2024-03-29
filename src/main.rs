use std::collections::BTreeMap;
use std::fmt;

type Price = u32;
type Size = u32;
type Total = u32;

enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    bids: BTreeMap<Price, Size>,
    asks: BTreeMap<Price, Size>,
}

impl OrderBook {
    fn new() -> Self {
        let bids: BTreeMap<Price, Size> = BTreeMap::new();
        let asks: BTreeMap<Price, Size> = BTreeMap::new();
        OrderBook { bids, asks }
    }

    fn is_empty(&self) -> bool {
        self.bids.is_empty() & self.asks.is_empty()
    }
    fn add(&mut self, price: Price, amount: Size, order_type: OrderType) {
        match order_type {
            OrderType::Bid => {
                *self.bids.entry(price).or_insert(0) += amount;
            }
            OrderType::Ask => {
                *self.asks.entry(price).or_insert(0) += amount;
            }
        }
    }

    fn add_bid(&mut self, price: Price, amount: Size) {
        self.add(price, amount, OrderType::Bid);
    }
    fn add_ask(&mut self, price: Price, amount: Size) {
        self.add(price, amount, OrderType::Ask);
    }

    fn clear_key(&mut self, price: Price, order_type: OrderType) {
        match order_type {
            OrderType::Bid => {
                if self.bids[&price] < 1 {
                    self.bids.remove(&price);
                } else {
                    panic!("Cannot clear keys.")
                }
            }
            OrderType::Ask => {
                if self.asks[&price] < 1 {
                    self.asks.remove(&price);
                } else {
                    panic!("Cannot clear keys.")
                }
            }
        }
    }

    fn remove(&mut self, price: Price, amount: Size, order_type: OrderType) -> Size {
        match order_type {
            OrderType::Bid => {
                if let Some(x) = self.bids.get_mut(&price) {
                    *x -= amount;
                } else {
                    panic!("No bids to remove from {}", &price);
                }
                return *self.bids.get(&price).unwrap();
            }
            OrderType::Ask => {
                if let Some(x) = self.asks.get_mut(&price) {
                    *x -= amount;
                } else {
                    panic!("No asks to remove from {}", &price);
                }
                return *self.asks.get(&price).unwrap();
            }
        };
    }

    fn query_orders(&self, price: Price) -> Option<Size> {
        if let Some(query) = self.bids.get(&price) {
            return Some(*query);
        }
        if let Some(query) = self.asks.get(&price) {
            return Some(*query);
        }
        None
    }

    fn remove_bid(&mut self, price: Price, amount: Size) {
        if let Some(query) = &self.query_orders(price) {
            if amount <= *query {
                let remainder = self.remove(price, amount, OrderType::Bid);
                if remainder < 1 {
                    self.clear_key(price, OrderType::Bid);
                }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }

    fn remove_ask(&mut self, price: Price, amount: Size) {
        if let Some(query) = &self.query_orders(price) {
            if amount <= *query {
                let remainder = self.remove(price, amount, OrderType::Ask);
                if remainder < 1 {
                    self.clear_key(price, OrderType::Ask);
                }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }

    fn total_asks(&self) -> BTreeMap<Price, Total> {
        let mut ask_total: BTreeMap<Price, Total> = BTreeMap::new();
        let mut total = 0;
        for (price, amount) in self.asks.iter() {
            total += amount;
            *ask_total.entry(*price).or_insert(0) += total;
        }
        ask_total
    }

    fn total_bids(&self) -> BTreeMap<Price, Total> {
        let mut bid_total: BTreeMap<Price, Total> = BTreeMap::new();
        let mut total = 0;
        for (price, amount) in self.bids.iter().rev() {
            total += amount;
            *bid_total.entry(*price).or_insert(0) += total;
        }
        bid_total
    }

    fn best_bid(&self) -> (Price, Size) {
        let (p, s) = self.bids.iter().rev().next().unwrap();
        (*p, *s)
    }

    fn best_ask(&self) -> (Price, Size) {
        let (p, s) = self.asks.iter().next().unwrap();
        (*p, *s)
    }

    // fn buy(&self, price: Price, amount: Size)
}

impl fmt::Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "Orderbook empty")
        } else {
            write!(f, "\tASK\n\n").unwrap();
            write!(f, "Price\tSize\tTotal\n").unwrap();

            let bid_total = self.total_asks();

            for (price, amount) in self.asks.iter().rev() {
                write!(f, "{}\t{}\t{}\n", price, amount, bid_total[price]).unwrap();
            }
            write!(f, "---------------------\n").unwrap();

            let ask_total = self.total_bids();

            for (price, amount) in self.bids.iter().rev() {
                write!(f, "{}\t{}\t{}\n", price, amount, ask_total[price]).unwrap();
            }
            write!(f, "\n\tBID\n")
        }
    }
}

fn main() {
    let mut orderbook = OrderBook::new();

    assert!(&orderbook.is_empty());

    println!("{}", orderbook);

    orderbook.add_ask(8723, 100);
    orderbook.add_ask(8881, 125);
    orderbook.add_ask(9900, 100);

    orderbook.add_bid(8720, 174);
    orderbook.add_bid(8600, 100);
    orderbook.add_bid(8499, 100);

    println!("{}", orderbook);
    println!("best ask{:?}", orderbook.best_ask());
    println!("best bid{:?}", orderbook.best_bid());

    println!("{:?}", orderbook.query_orders(8721));

    orderbook.remove_ask(8723, 100);
    orderbook.remove_bid(8720, 173);

    println!("{}", orderbook);
}
