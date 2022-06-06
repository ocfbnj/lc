use rand::prelude::*;

pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            radius,
            x_center,
            y_center,
            rng: thread_rng(),
        }
    }

    pub fn rand_point(&mut self) -> Vec<f64> {
        let r = self.rng.gen_range(0.0f64..=1.0).sqrt() * self.radius;
        let si = self.rng.gen_range(0.0..2.0 * std::f64::consts::PI);

        vec![self.x_center + r * si.cos(), self.y_center + r * si.sin()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut solution = Solution::new(1.0, 0.0, 0.0);
        println!("{:?}", solution.rand_point());
    }
}
