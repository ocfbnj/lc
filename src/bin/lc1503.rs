struct Solution;

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter()
            .chain(right.into_iter().map(|x| n - x))
            .max()
            .unwrap()
    }
}

fn main() {
    assert_eq!(Solution::get_last_moment(4, vec![4, 3], vec![0, 1]), 4);
    assert_eq!(
        Solution::get_last_moment(7, vec![], vec![0, 1, 2, 3, 4, 5, 6, 7]),
        7
    );
}
