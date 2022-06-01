pub struct Solution;

impl Solution {
    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let total: i32 = matchsticks.iter().sum();
        if total % 4 != 0 {
            return false;
        }

        let target = total / 4;
        let mut lengths = [0; 4];

        if matchsticks.iter().any(|&x| x > target) {
            return false;
        }

        fn resolve(i: usize, lengths: &mut [i32], matchsticks: &[i32], target: i32) -> bool {
            if i == matchsticks.len() {
                return lengths.iter().skip(1).all(|&x| x == lengths[0]);
            }

            let mut res = false;

            for j in 0..lengths.len() {
                lengths[j] += matchsticks[i];
                if lengths[j] <= target {
                    res |= resolve(i + 1, lengths, matchsticks, target);
                }
                lengths[j] -= matchsticks[i];

                if res {
                    return true;
                }
            }

            res
        }

        resolve(0, &mut lengths, &matchsticks, target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
        assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
        assert_eq!(Solution::makesquare(vec![3, 3, 3, 3]), true);
        assert_eq!(
            Solution::makesquare(vec![3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2]),
            false
        );
        assert_eq!(
            Solution::makesquare(vec![3, 3, 3, 3, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1]),
            true
        );
        assert_eq!(
            Solution::makesquare(vec![10, 6, 5, 5, 5, 3, 3, 3, 2, 2, 2, 2]),
            true
        );
        assert_eq!(
            Solution::makesquare(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 102]),
            false
        );
    }
}
