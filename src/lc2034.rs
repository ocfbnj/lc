use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub struct StockPrice {
    ts: HashMap<i32, i32>,
    maximum_ts: i32,
    maximum_price: BinaryHeap<(i32, i32)>,
    minimum_price: BinaryHeap<(Reverse<i32>, i32)>,
}

impl StockPrice {
    pub fn new() -> Self {
        StockPrice {
            ts: HashMap::new(),
            maximum_ts: i32::MIN,
            maximum_price: BinaryHeap::new(),
            minimum_price: BinaryHeap::new(),
        }
    }

    pub fn update(&mut self, timestamp: i32, price: i32) {
        self.maximum_ts = self.maximum_ts.max(timestamp);
        *self.ts.entry(timestamp).or_default() = price;

        self.maximum_price.push((price, timestamp));
        self.minimum_price.push((Reverse(price), timestamp));
    }

    pub fn current(&self) -> i32 {
        *self.ts.get(&self.maximum_ts).unwrap()
    }

    pub fn maximum(&mut self) -> i32 {
        loop {
            let (price, timestamp) = self.maximum_price.peek().unwrap();
            if self.ts.get(&timestamp) == Some(price) {
                return *price;
            }

            self.maximum_price.pop();
        }
    }

    pub fn minimum(&mut self) -> i32 {
        loop {
            let (price, timestamp) = self.minimum_price.peek().unwrap();
            if self.ts.get(&timestamp) == Some(&price.0) {
                return price.0;
            }

            self.minimum_price.pop();
        }
    }
}

#[test]
fn test() {
    let mut stock_price = StockPrice::new();
    stock_price.update(1, 10);
    stock_price.update(2, 5);
    assert_eq!(stock_price.current(), 5);
    assert_eq!(stock_price.maximum(), 10);
    stock_price.update(1, 3);
    assert_eq!(stock_price.maximum(), 5);
    stock_price.update(4, 2);
    assert_eq!(stock_price.minimum(), 2);
}
