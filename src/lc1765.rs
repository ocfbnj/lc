pub struct Solution;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let mut res = vec![vec![-1; is_water[0].len()]; is_water.len()];
        let mut queue = VecDeque::new();

        for i in 0..is_water.len() {
            for j in 0..is_water[i].len() {
                if is_water[i][j] == 1 {
                    res[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        let mut height = 1;

        while !queue.is_empty() {
            let n = queue.len();
            for _ in 0..n {
                let (i, j) = queue.pop_front().unwrap();

                if i > 0 && res[i - 1][j] == -1 {
                    res[i - 1][j] = height;
                    queue.push_back((i - 1, j));
                }

                if i + 1 < res.len() && res[i + 1][j] == -1 {
                    res[i + 1][j] = height;
                    queue.push_back((i + 1, j));
                }

                if j > 0 && res[i][j - 1] == -1 {
                    res[i][j - 1] = height;
                    queue.push_back((i, j - 1));
                }

                if j + 1 < res[0].len() && res[i][j + 1] == -1 {
                    res[i][j + 1] = height;
                    queue.push_back((i, j + 1));
                }
            }

            height += 1;
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::highest_peak(vec![vec![0, 1], vec![0, 0]]),
        vec![vec![1, 0], vec![2, 1]]
    );

    assert_eq!(
        Solution::highest_peak(vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]]),
        vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]
    );
}
