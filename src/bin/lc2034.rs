use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

struct StockPrice {
    ts: HashMap<i32, i32>,
    maximum_ts: i32,
    maximum_price: BinaryHeap<(i32, i32)>,
    minimum_price: BinaryHeap<(Reverse<i32>, i32)>,
}

impl StockPrice {
    fn new() -> Self {
        StockPrice {
            ts: HashMap::new(),
            maximum_ts: i32::MIN,
            maximum_price: BinaryHeap::new(),
            minimum_price: BinaryHeap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.maximum_ts = self.maximum_ts.max(timestamp);
        *self.ts.entry(timestamp).or_default() = price;

        self.maximum_price.push((price, timestamp));
        self.minimum_price.push((Reverse(price), timestamp));
    }

    fn current(&self) -> i32 {
        *self.ts.get(&self.maximum_ts).unwrap()
    }

    fn maximum(&mut self) -> i32 {
        loop {
            let (price, timestamp) = self.maximum_price.peek().unwrap();
            if self.ts.get(&timestamp) == Some(price) {
                return *price;
            }

            self.maximum_price.pop();
        }
    }

    fn minimum(&mut self) -> i32 {
        loop {
            let (price, timestamp) = self.minimum_price.peek().unwrap();
            if self.ts.get(&timestamp) == Some(&price.0) {
                return price.0;
            }

            self.minimum_price.pop();
        }
    }
}

fn main() {
    let mut stock_price = StockPrice::new();
    stock_price.update(1, 10); // 时间戳为 [1] ，对应的股票价格为 [10] 。
    stock_price.update(2, 5); // 时间戳为 [1,2] ，对应的股票价格为 [10,5] 。
    assert_eq!(stock_price.current(), 5); // 返回 5 ，最新时间戳为 2 ，对应价格为 5 。
    assert_eq!(stock_price.maximum(), 10); // 返回 10 ，最高价格的时间戳为 1 ，价格为 10 。
    stock_price.update(1, 3); // 之前时间戳为 1 的价格错误，价格更新为 3 。
                              // 时间戳为 [1,2] ，对应股票价格为 [3,5] 。
    assert_eq!(stock_price.maximum(), 5); // 返回 5 ，更正后最高价格为 5 。
    stock_price.update(4, 2); // 时间戳为 [1,2,4] ，对应价格为 [3,5,2] 。
    assert_eq!(stock_price.minimum(), 2); // 返回 2 ，最低价格时间戳为 4 ，价格为 2 。
}
