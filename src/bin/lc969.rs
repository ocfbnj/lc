struct Solution;

impl Solution {
    pub fn pancake_sort(mut arr: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();

        let mut n = arr.len();

        while n > 1 {
            let pos = arr.iter().position(|&x| x as usize == n).unwrap();
            if pos + 1 < n {
                arr[0..pos + 1].reverse();
                arr[0..n].reverse();

                res.push(pos as i32 + 1);
                res.push(n as i32);
            }

            n -= 1;
        }

        return res;
    }
}

fn main() {
    assert_eq!(
        Solution::pancake_sort(vec![3, 2, 4, 1]),
        vec![3, 4, 2, 3, 1, 2]
    );
}
