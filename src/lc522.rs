pub struct Solution;

impl Solution {
    pub fn find_lu_slength(mut strs: Vec<String>) -> i32 {
        strs.sort_by(|a, b| {
            let b1 = b.len().cmp(&a.len());
            if b1 != std::cmp::Ordering::Equal {
                return b1;
            }

            return a.cmp(b);
        });

        fn is_subsequence(a: &str, b: &str) -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();

            let mut i = 0;
            let mut j = 0;

            while i < a.len() && j < b.len() {
                if a[i] == b[j] {
                    j += 1;
                }

                i += 1;
            }

            return j == b.len();
        }

        'outer: for i in 0..strs.len() {
            for j in 0..strs.len() {
                if i == j {
                    continue;
                }

                if is_subsequence(&strs[j], &strs[i]) {
                    continue 'outer;
                }
            }

            return strs[i].len() as i32;
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::find_lu_slength(vec!["aaa".to_owned(), "aaa".to_owned(), "aa".to_owned()]),
            -1
        );

        assert_eq!(
            Solution::find_lu_slength(vec!["aba".to_owned(), "cdc".to_owned(), "eae".to_owned()]),
            3
        );

        assert_eq!(
            Solution::find_lu_slength(vec![
                "aabbcc".to_owned(),
                "aabbcc".to_owned(),
                "c".to_owned(),
                "e".to_owned(),
                "aabbcd".to_owned()
            ]),
            6
        );

        assert_eq!(
            Solution::find_lu_slength(vec![
                "aabbcc".to_owned(),
                "aabbcc".to_owned(),
                "cb".to_owned(),
                "abc".to_owned(),
                "mmnnqq".to_owned()
            ]),
            6
        );

        assert_eq!(
            Solution::find_lu_slength(vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
                "e".to_owned(),
                "f".to_owned(),
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
                "e".to_owned(),
                "f".to_owned()
            ]),
            -1
        );
    }
}
