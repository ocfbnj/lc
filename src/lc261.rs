pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    pub fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    pub fn pick(&self, target: i32) -> i32 {
        use rand::Rng;

        let mut count = 0;
        let mut index = 0;

        for (i, &num) in self.nums.iter().enumerate() {
            if num == target {
                count += 1;
                if rand::thread_rng().gen_range(0..count) == 0 {
                    index = i;
                }
            }
        }

        return index as i32;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

#[test]
fn test() {}
