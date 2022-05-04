pub struct Solution;

impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let n = security.len();
        let time = time as usize;

        if time >= n {
            return vec![];
        }

        let mut inc = vec![0; n];
        let mut dec = vec![0; n];

        for i in 1..n {
            if security[i - 1] >= security[i] {
                dec[i] = dec[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if security[i] <= security[i + 1] {
                inc[i] = inc[i + 1] + 1;
            }
        }

        let mut res = vec![];
        for i in time..n - time {
            if dec[i] >= time && inc[i] >= time {
                res.push(i as i32);
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
            Solution::good_days_to_rob_bank(vec![5, 3, 3, 3, 5, 6, 2], 2),
            vec![2, 3]
        );

        assert_eq!(
            Solution::good_days_to_rob_bank(vec![1, 1, 1, 1, 1], 0),
            vec![0, 1, 2, 3, 4]
        );

        assert_eq!(
            Solution::good_days_to_rob_bank(vec![1, 2, 3, 4, 5, 6], 2),
            vec![]
        );

        assert_eq!(Solution::good_days_to_rob_bank(vec![1], 5), vec![]);
    }
}
