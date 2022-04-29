pub struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        use std::collections::VecDeque;

        let mut path = vec![vec![i32::MAX; 2]; n as usize + 1];
        let mut queue = VecDeque::<i32>::new();
        let mut next = vec![Vec::new(); n as usize + 1];

        for edge in edges.iter() {
            let (u, v) = (edge[0], edge[1]);
            next[u as usize].push(v);
            next[v as usize].push(u);
        }

        path[1][0] = 0;
        queue.push_back(1);

        let mut len = 0;
        'outer: loop {
            let size = queue.len();
            for _ in 0..size {
                let node = queue.pop_front().unwrap();

                if path[n as usize][1] != i32::MAX {
                    break 'outer;
                }

                for &nex in next[node as usize].iter() {
                    if len + 1 < path[nex as usize][0] {
                        path[nex as usize][0] = len + 1;
                        queue.push_back(nex);
                    } else if len + 1 > path[nex as usize][0] && len + 1 < path[nex as usize][1] {
                        path[nex as usize][1] = len + 1;
                        queue.push_back(nex);
                    }
                }
            }

            len += 1;
        }

        let mut res = 0;
        for _ in 0..path[n as usize][1] {
            let temp = res / change;
            if (temp & 1) == 1 {
                res = temp * change + change;
            }

            res += time;
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::second_minimum(
            6,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![5, 4],
                vec![4, 6]
            ],
            710,
            10
        ),
        2870
    );

    assert_eq!(
        Solution::second_minimum(
            5,
            vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]],
            3,
            5
        ),
        13
    );

    assert_eq!(
        Solution::second_minimum(
            6,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![2, 4],
                vec![3, 5],
                vec![5, 4],
                vec![4, 6]
            ],
            3,
            5
        ),
        16
    );

    assert_eq!(Solution::second_minimum(2, vec![vec![1, 2]], 3, 2), 11);
}
