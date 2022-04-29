pub struct Solution;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        use std::collections::VecDeque;

        let mut res = 0;
        let mut map = HashMap::<i32, Vec<usize>>::new();

        for (i, num) in arr.iter().enumerate() {
            map.entry(*num).or_default().push(i);
        }

        let mut visited = vec![false; arr.len()];
        let mut queue = VecDeque::<usize>::new();

        queue.push_back(0);
        visited[0] = true;

        'outer: while !queue.is_empty() {
            let len = queue.len();
            for _ in 0..len {
                let i = queue.pop_front().unwrap();

                if i == arr.len() - 1 {
                    break 'outer;
                }

                if i + 1 < arr.len() && !visited[i + 1] {
                    visited[i + 1] = true;
                    queue.push_back(i + 1);
                }

                if i as i32 - 1 >= 0 && !visited[i - 1] {
                    visited[i - 1] = true;
                    queue.push_back(i - 1);
                }

                let vec = map.get_mut(&arr[i]).unwrap();
                for next in vec.iter() {
                    if *next != i {
                        queue.push_back(*next);
                    }
                }
                vec.clear();
            }

            res += 1;
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::min_jumps([100, -23, -23, 404, 100, 23, 23, 23, 3, 404].to_vec()),
        3
    );

    assert_eq!(Solution::min_jumps([7].to_vec()), 0);
    assert_eq!(Solution::min_jumps([7, 6, 9, 6, 9, 6, 9, 7].to_vec()), 1);
    assert_eq!(Solution::min_jumps([6, 1, 9].to_vec()), 2);
    assert_eq!(
        Solution::min_jumps([11, 22, 7, 7, 7, 7, 7, 7, 7, 22, 13].to_vec()),
        3
    );
}
