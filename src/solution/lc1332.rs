pub struct Solution;

impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        match s.as_bytes().iter().rev().eq(s.as_bytes().iter()) {
            true => 1,
            false => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::remove_palindrome_sub("ababa".to_owned()), 1);
        assert_eq!(Solution::remove_palindrome_sub("abb".to_owned()), 2);
        assert_eq!(Solution::remove_palindrome_sub("baabb".to_owned()), 2);
    }
}
