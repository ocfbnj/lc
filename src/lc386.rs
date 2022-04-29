pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = Vec::new();

        for i in 1..=9.min(n) {
            Self::resolve(&mut res, i, n);
        }

        return res;
    }

    fn resolve(arr: &mut Vec<i32>, i: i32, n: i32) {
        arr.push(i);

        let left = i * 10;
        if left > n {
            return;
        }
        let right = (i * 10 + 9).min(n);

        for i in left..=right {
            Self::resolve(arr, i, n);
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );

    assert_eq!(Solution::lexical_order(2), vec![1, 2]);
}
