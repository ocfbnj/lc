pub struct Solution;

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        for i in 0..nums.len() {
            let num = nums[i].abs();
            let target = nums[(num - 1) as usize];

            if target < 0 {
                res.push(num);
            } else {
                nums[(num - 1) as usize] = -nums[(num - 1) as usize];
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
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );

        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);

        assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
    }
}
