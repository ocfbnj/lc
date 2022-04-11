struct Solution;

impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        use std::{cell::RefCell, rc::Rc};

        fn dfs(i: usize, n: usize, arr: &[Vec<i32>], count: Rc<RefCell<Vec<u8>>>) -> bool {
            if n == 0 {
                return count.borrow().iter().all(|&x| x == 0);
            }

            if i == arr.len() {
                return false;
            }

            let ret = dfs(i + 1, n, arr, count.clone());
            if ret {
                return true;
            }

            let u = arr[i][0];
            let v = arr[i][1];

            count.borrow_mut()[u as usize] += 1;
            count.borrow_mut()[v as usize] -= 1;
            let ret = dfs(i + 1, n - 1, arr, count.clone());
            count.borrow_mut()[u as usize] -= 1;
            count.borrow_mut()[v as usize] += 1;

            return ret;
        }

        let count = Rc::new(RefCell::new(vec![0; n as usize]));

        for i in (0..=requests.len()).rev() {
            if dfs(0, i, requests.as_slice(), count.clone()) {
                return i as i32;
            }
        }

        return 0;
    }
}

fn main() {
    assert_eq!(
        Solution::maximum_requests(
            5,
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![0, 1],
                vec![1, 2],
                vec![2, 0],
                vec![3, 4]
            ]
        ),
        5
    );

    assert_eq!(
        Solution::maximum_requests(5, vec![vec![0, 0], vec![1, 2], vec![2, 1]]),
        3
    );

    assert_eq!(
        Solution::maximum_requests(
            20,
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![0, 1],
                vec![1, 2],
                vec![2, 0],
                vec![3, 4],
                vec![0, 1],
                vec![3, 4],
                vec![5, 3],
                vec![2, 6]
            ]
        ),
        5
    );
}
