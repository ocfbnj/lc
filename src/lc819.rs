pub struct Solution;

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned_words: std::collections::HashSet<_> = banned.into_iter().collect();

        let mut counter = std::collections::HashMap::new();
        paragraph
            .split(" !?',;.".chars().collect::<Vec<_>>().as_slice())
            .map(|word| word.to_ascii_lowercase())
            .filter(|word| !word.is_empty())
            .filter(|word| !banned_words.contains(word))
            .for_each(|word| {
                *counter.entry(word).or_insert(0) += 1;
            });

        return counter.iter().max_by_key(|x| x.1).unwrap().0.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_owned(),
                vec!["hit".to_owned()]
            ),
            "ball".to_owned()
        );
    }
}
