pub struct Solution;

impl Solution {
    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let modu = 1e9 as usize + 7;
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let mut counts = [0; 31];
        nums.iter().for_each(|&x| counts[x as usize] += 1);

        let mut dp = [0usize; (1 << 10) + 1];
        dp[0] = 1;
        for _ in 0..counts[1] {
            dp[0] = (dp[0] * 2) % modu;
        }

        'outer: for i in 2..=30 {
            if counts[i] == 0 {
                continue;
            }

            let mut subset = 0u16;

            for (j, num) in primes.iter().enumerate() {
                if i % (num * num) == 0 {
                    continue 'outer;
                }

                if i % num == 0 {
                    subset |= 1 << j;
                }
            }

            for mask in (1..1u16 << 10).rev() {
                if mask & subset == subset {
                    dp[mask as usize] = (dp[mask as usize]
                        + (dp[(mask & !subset) as usize] * counts[i]) % modu)
                        % modu;
                }
            }
        }

        dp.iter().skip(1).fold(0usize, |acc, x| (acc + x) % modu) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::number_of_good_subsets(vec![2]), 1);
        assert_eq!(Solution::number_of_good_subsets(vec![1, 2, 3, 4]), 6);
        assert_eq!(Solution::number_of_good_subsets(vec![4, 2, 3, 15]), 5);
        assert_eq!(
            Solution::number_of_good_subsets(vec![
                10, 11, 5, 1, 10, 1, 3, 1, 26, 11, 6, 1, 1, 15, 1, 7, 22, 1, 1, 1, 1, 1, 23, 1, 29,
                5, 6, 1, 1, 29, 1, 1, 21, 19, 1, 1, 1, 2, 1, 11, 1, 15, 1, 22, 14, 1, 1, 1, 1, 6,
                7, 1, 14, 3, 5, 1, 22, 1, 1, 1, 17, 1, 29, 2, 1, 15, 10, 1, 5, 7, 1, 1, 1, 30, 1,
                30, 1, 21, 10, 1, 1, 1, 1, 1, 2, 6, 5, 7, 3, 1, 1, 19, 29, 1, 7, 13, 14, 1, 5, 26,
                19, 11, 1, 1, 1, 1, 1, 1, 1, 1, 22, 15, 1, 1, 13, 1, 17, 1, 1, 1, 13, 6, 1, 10, 1,
                1, 17, 1, 1, 3, 14, 7, 17, 1, 13, 1, 1, 1, 1, 1, 11, 1, 1, 6, 1, 1, 1, 1, 1, 2, 1,
                30, 2, 26, 1, 1, 14, 1, 26, 29, 30, 1, 13, 21, 1, 1, 14, 21, 1, 23, 1, 15, 23, 21,
                1, 30, 19, 19, 1, 10, 23, 3, 3, 17, 22, 2, 26, 1, 11, 1, 23, 1, 1, 1, 15, 1, 1, 13,
                1, 1
            ]),
            520317213
        );
    }
}
