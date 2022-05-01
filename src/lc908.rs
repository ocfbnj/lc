pub struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();

        let distance = max - min;

        if distance <= 2 * k {
            0
        } else {
            distance - 2 * k
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_range_i(vec![1], 0), 0);
    assert_eq!(Solution::smallest_range_i(vec![0, 10], 2), 6);
    assert_eq!(Solution::smallest_range_i(vec![1, 3, 6], 3), 0);
}
