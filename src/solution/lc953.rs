pub struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        use std::collections::HashMap;

        let mut ordering: HashMap<u8, usize> = HashMap::new();

        let order = order.into_bytes();
        for i in 0..order.len() {
            ordering.insert(order[i], i);
        }

        let lt = |a: &str, b: &str| -> bool {
            let a = a.as_bytes();
            let b = b.as_bytes();

            let mut i = 0;
            let mut j = 0;

            while i < a.len() && j < b.len() {
                let oa = ordering[&a[i]];
                let ob = ordering[&b[j]];

                if oa < ob {
                    return true;
                } else if ob < oa {
                    return false;
                }

                i += 1;
                j += 1;
            }

            a.len() < b.len()
        };

        for i in 1..words.len() {
            if lt(&words[i], &words[i - 1]) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_alien_sorted(
                vec!["hello".to_owned(), "leetcode".to_owned()],
                "hlabcdefgijkmnopqrstuvwxyz".to_owned()
            ),
            true
        );

        assert_eq!(
            Solution::is_alien_sorted(
                vec!["word".to_owned(), "world".to_owned(), "row".to_owned()],
                "worldabcefghijkmnpqstuvxyz".to_owned()
            ),
            false
        );

        assert_eq!(
            Solution::is_alien_sorted(
                vec!["apple".to_owned(), "app".to_owned()],
                "abcdefghijklmnopqrstuvwxyz".to_owned()
            ),
            false
        );

        assert_eq!(
            Solution::is_alien_sorted(
                vec!["hello".to_owned(), "hello".to_owned()],
                "abcdefghijklmnopqrstuvwxyz".to_owned()
            ),
            true
        );
    }
}
