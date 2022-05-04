pub struct Solution;

impl Solution {
    pub fn reorder_log_files(mut logs: Vec<String>) -> Vec<String> {
        logs.sort_by(|a, b| {
            let (id1, content1) = a.split_once(' ').unwrap();
            let (id2, content2) = b.split_once(' ').unwrap();

            let a_is_letter_log = content1.chars().next().unwrap().is_alphabetic();
            let b_is_letter_log = content2.chars().next().unwrap().is_alphabetic();

            match (a_is_letter_log, b_is_letter_log) {
                (true, true) => content1.cmp(content2).then(id1.cmp(id2)),
                (false, false) => std::cmp::Ordering::Equal,
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
            }
        });

        return logs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::reorder_log_files(vec![
                "dig1 8 1 5 1".to_owned(),
                "let1 art can".to_owned(),
                "dig2 3 6".to_owned(),
                "let2 own kit dig".to_owned(),
                "let3 art zero".to_owned()
            ]),
            vec![
                "let1 art can".to_owned(),
                "let3 art zero".to_owned(),
                "let2 own kit dig".to_owned(),
                "dig1 8 1 5 1".to_owned(),
                "dig2 3 6".to_owned()
            ]
        );

        assert_eq!(
            Solution::reorder_log_files(vec![
                "a1 9 2 3 1".to_owned(),
                "g1 act car".to_owned(),
                "zo4 4 7".to_owned(),
                "ab1 off key dog".to_owned(),
                "a8 act zoo".to_owned()
            ]),
            vec![
                "g1 act car".to_owned(),
                "a8 act zoo".to_owned(),
                "ab1 off key dog".to_owned(),
                "a1 9 2 3 1".to_owned(),
                "zo4 4 7".to_owned()
            ]
        );

        assert_eq!(
            Solution::reorder_log_files(vec![
                "a1 9 2 3 1".to_owned(),
                "g1 act car".to_owned(),
                "zo4 4 7".to_owned(),
                "ab1 off key dog".to_owned(),
                "a8 act zoo".to_owned(),
                "a2 act car".to_owned()
            ]),
            vec![
                "a2 act car".to_owned(),
                "g1 act car".to_owned(),
                "a8 act zoo".to_owned(),
                "ab1 off key dog".to_owned(),
                "a1 9 2 3 1".to_owned(),
                "zo4 4 7".to_owned()
            ]
        );
    }
}
