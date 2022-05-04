pub struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }

        let left_max = (10_i32.pow(n as u32) - 1) as i64;
        let left_min = 10_i32.pow(n as u32 - 1) as i64;

        for i in (left_min..=left_max).rev() {
            let mut palindrome = i;

            let mut x = i;
            while x != 0 {
                palindrome = palindrome * 10 + (x % 10);
                x /= 10;
            }

            let mut x = left_max;
            while x * x >= palindrome {
                if palindrome % x == 0 {
                    return (palindrome % 1337) as i32;
                }
                x -= 1;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::largest_palindrome(1), 9);
        assert_eq!(Solution::largest_palindrome(2), 987);
        assert_eq!(Solution::largest_palindrome(6), 1218);
    }
}
