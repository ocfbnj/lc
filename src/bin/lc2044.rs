struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let max_value = nums.iter().fold(0, |accum, &item| accum | item);

        for bits in 0..1 << nums.len() {
            let mut sum = 0;

            for i in 0..nums.len() {
                if (bits >> i) & 1 == 1 {
                    sum |= nums[i];
                }
            }

            if sum == max_value {
                res += 1;
            }
        }

        return res;
    }
}

fn main() {
    assert_eq!(Solution::count_max_or_subsets(vec![3, 1]), 2);
    assert_eq!(Solution::count_max_or_subsets(vec![2, 2, 2]), 7);
    assert_eq!(Solution::count_max_or_subsets(vec![3, 2, 1, 5]), 6);
}
