pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs: Vec<_> = strs.into_iter().map(|s| s.into_bytes()).collect();
        let mut res = 0;

        for j in 0..strs[0].len() {
            for i in 1..strs.len() {
                if strs[i][j] < strs[i - 1][j] {
                    res += 1;
                    break;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_deletion_size(vec!["cba".to_owned(), "daf".to_owned(), "ghi".to_owned()]),
            1
        );

        assert_eq!(
            Solution::min_deletion_size(vec!["a".to_owned(), "b".to_owned()]),
            0
        );
    }
}
