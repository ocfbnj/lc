pub struct Solution;

impl Solution {
    fn dfs(i: i32, j: i32, heights: &[Vec<i32>], ocean: &mut [Vec<bool>]) {
        let m = ocean.len() as i32;
        let n = ocean[0].len() as i32;

        if ocean[i as usize][j as usize] {
            return;
        }
        ocean[i as usize][j as usize] = true;

        for (a, b) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next_i = i + a;
            let next_j = j + b;

            if next_i < m
                && next_j < n
                && next_i >= 0
                && next_j >= 0
                && heights[next_i as usize][next_j as usize] >= heights[i as usize][j as usize]
            {
                Self::dfs(next_i, next_j, heights, ocean);
            }
        }
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len() as i32;
        let n = heights[0].len() as i32;

        let mut pacific = vec![vec![false; n as usize]; m as usize];
        let mut atlantic = vec![vec![false; n as usize]; m as usize];

        for i in 0..m {
            Self::dfs(i, 0, &heights, &mut pacific);
            Self::dfs(i, n - 1, &heights, &mut atlantic);
        }

        for j in 0..n {
            Self::dfs(0, j, &heights, &mut pacific);
            Self::dfs(m - 1, j, &heights, &mut atlantic);
        }

        let mut res = vec![];
        for i in 0..m as usize {
            for j in 0..n as usize {
                if pacific[i][j] && atlantic[i][j] {
                    res.push(vec![i as i32, j as i32])
                }
            }
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ]),
        vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0]
        ]
    );

    assert_eq!(
        Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2]]),
        vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]
    );

    assert_eq!(
        Solution::pacific_atlantic(vec![vec![10, 10, 10], vec![10, 1, 10], vec![10, 10, 10]]),
        vec![
            vec![0, 0],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![1, 2],
            vec![2, 0],
            vec![2, 1],
            vec![2, 2]
        ]
    );
}
