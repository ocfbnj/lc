use std::collections::HashMap;

pub struct MyCalendarThree {
    nodes: HashMap<i32, (i32, i32)>,
}

impl MyCalendarThree {
    pub fn new() -> Self {
        MyCalendarThree {
            nodes: HashMap::new(),
        }
    }

    pub fn book(&mut self, start: i32, end: i32) -> i32 {
        self.update(start, end - 1, 0, 1e9 as i32, 0);
        self.nodes.get(&0).unwrap().0
    }

    fn update(&mut self, q_low: i32, q_high: i32, low: i32, high: i32, root: i32) {
        if q_low > high || q_high < low {
            return;
        }

        if q_low <= low && q_high >= high {
            let entry = self.nodes.entry(root).or_default();
            entry.0 += 1;
            entry.1 += 1;

            return;
        }

        let mid = (low + high) / 2;
        self.update(q_low, q_high, low, mid, 2 * root + 1);
        self.update(q_low, q_high, mid + 1, high, 2 * root + 2);

        let left = self.nodes.entry(2 * root + 1).or_default().0;
        let right = self.nodes.entry(2 * root + 2).or_default().0;
        let entry = self.nodes.entry(root).or_default();
        entry.0 = entry.1 + std::cmp::max(left, right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut calendar = MyCalendarThree::new();

        assert_eq!(calendar.book(10, 20), 1);
        assert_eq!(calendar.book(50, 60), 1);
        assert_eq!(calendar.book(10, 40), 2);
        assert_eq!(calendar.book(5, 15), 3);
        assert_eq!(calendar.book(5, 10), 3);
        assert_eq!(calendar.book(25, 55), 3);
    }
}
