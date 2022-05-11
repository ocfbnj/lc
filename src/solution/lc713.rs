pub struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;

        let mut left = 0;
        let mut right = 0;
        let mut mul = 1;

        while right < nums.len() {
            mul *= nums[right];
            right += 1;

            while left < right && mul >= k {
                mul /= nums[left];
                left += 1;
            }

            res += (right - left) as i32;
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );

        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
    }
}
