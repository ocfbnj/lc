use std::collections::HashMap;

pub struct DetectSquares {
    axis: HashMap<i32, HashMap<i32, i32>>,
}

impl DetectSquares {
    pub fn new() -> Self {
        DetectSquares {
            axis: HashMap::new(),
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let x = point[0];
        let y = point[1];

        *self.axis.entry(y).or_default().entry(x).or_default() += 1
    }

    pub fn count(&mut self, point: Vec<i32>) -> i32 {
        let x = point[0];
        let y = point[1];

        let mut res = 0;

        if let Some(e) = self.axis.get(&y) {
            for (&xp, &c) in e.iter().filter(|(&elem, _)| elem != x) {
                let len = xp - x;
                for yp in [y + len, y - len].into_iter() {
                    if let Some(xx) = self.axis.get(&yp) {
                        if let (Some(&x1_count), Some(&x2_count)) = (xx.get(&xp), xx.get(&x)) {
                            res += c * x1_count * x2_count;
                        }
                    }
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut obj = DetectSquares::new();
        obj.add(vec![3, 10]);
        obj.add(vec![11, 2]);
        obj.add(vec![3, 2]);
        assert_eq!(obj.count(vec![11, 10]), 1);
        assert_eq!(obj.count(vec![14, 8]), 0);
        obj.add(vec![11, 2]);
        assert_eq!(obj.count(vec![11, 10]), 2);
    }
}
