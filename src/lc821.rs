pub struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut res = vec![0; s.len()];

        let mut index = i32::MIN / 2;
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                index = i as i32;
            }
            res[i] = i as i32 - index;
        }

        let mut index = i32::MAX / 2;
        for (i, ch) in s.chars().rev().enumerate() {
            let i = (s.len() - i - 1) as i32;
            if ch == c {
                index = i as i32;
            }
            res[i as usize] = std::cmp::min(res[i as usize], index - i);
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::shortest_to_char("loveleetcode".to_owned(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );
}
