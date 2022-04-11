struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut res = Vec::with_capacity(n);

        'outer: for i in 0..n {
            let mut x = 0;
            let mut y = i;

            while x < m {
                if grid[x][y] == -1 {
                    // left
                    if y == 0 || grid[x][y - 1] == 1 {
                        res.push(-1);
                        continue 'outer;
                    }

                    y -= 1;
                } else {
                    // right
                    if y == n - 1 || grid[x][y + 1] == -1 {
                        res.push(-1);
                        continue 'outer;
                    }

                    y += 1;
                }

                x += 1;
            }

            res.push(y as i32);
        }

        return res;
    }
}

fn main() {
    assert_eq!(
        Solution::find_ball(vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1]
        ]),
        vec![1, -1, -1, -1, -1]
    );

    assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1]);

    assert_eq!(
        Solution::find_ball(vec![
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![1, 1, 1, 1, 1, 1],
            vec![-1, -1, -1, -1, -1, -1]
        ]),
        vec![0, 1, 2, 3, 4, -1]
    );
}
