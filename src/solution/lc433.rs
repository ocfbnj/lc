pub struct Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        use std::collections::{HashSet, VecDeque};

        let route: HashSet<String> = HashSet::from_iter(bank);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        let mut step = 0;

        visited.insert(start.clone());
        queue.push_back(start);

        while !queue.is_empty() {
            let n = queue.len();

            for _ in 0..n {
                let mut node = queue.pop_front().unwrap();

                if node == end {
                    return step;
                }

                let bytes = unsafe { node.as_bytes_mut() };

                for i in 0..bytes.len() {
                    for c in [b'A', b'C', b'G', b'T'] {
                        if bytes[i] != c {
                            let previous = bytes[i];
                            bytes[i] = c;

                            let new_str = unsafe { String::from_utf8_unchecked(bytes.to_vec()) };
                            if route.contains(&new_str) && !visited.contains(&new_str) {
                                visited.insert(new_str.clone());
                                queue.push_back(new_str);
                            }

                            bytes[i] = previous;
                        }
                    }
                }
            }

            step += 1;
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
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AACCGGTA".to_owned(),
                vec!["AACCGGTA".to_owned()],
            ),
            1
        );

        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AAACGGTA".to_owned(),
                vec![
                    "AACCGGTA".to_owned(),
                    "AACCGCTA".to_owned(),
                    "AAACGGTA".to_owned()
                ],
            ),
            2
        );

        assert_eq!(
            Solution::min_mutation(
                "AAAAACCC".to_owned(),
                "AACCCCCC".to_owned(),
                vec![
                    "AAAACCCC".to_owned(),
                    "AAACCCCC".to_owned(),
                    "AACCCCCC".to_owned()
                ],
            ),
            3
        );

        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AACCGCTA".to_owned(),
                vec![
                    "AACCGGTA".to_owned(),
                    "AACCGCTA".to_owned(),
                    "AAACGGTA".to_owned()
                ],
            ),
            2
        );
    }
}
