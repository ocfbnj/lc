pub struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        match n {
            1 => 1,
            _ => ((Self::find_the_winner(n - 1, k) + k - 1) % n) + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_the_winner(5, 2), 3);
        assert_eq!(Solution::find_the_winner(6, 5), 1);
    }
}
