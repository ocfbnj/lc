pub struct RecentCounter {
    times: std::collections::VecDeque<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            times: std::collections::VecDeque::new(),
        }
    }

    pub fn ping(&mut self, t: i32) -> i32 {
        while !self.times.is_empty() && *self.times.front().unwrap() < t - 3000 {
            self.times.pop_front();
        }

        self.times.push_back(t);

        return self.times.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut recent_counter = RecentCounter::new();
        assert_eq!(recent_counter.ping(1), 1);
        assert_eq!(recent_counter.ping(100), 2);
        assert_eq!(recent_counter.ping(3001), 3);
        assert_eq!(recent_counter.ping(3002), 3);
    }
}
