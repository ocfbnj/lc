pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
        let mut res = 1;
        k -= 1;

        while k > 0 {
            let step = Self::step(res, n);
            if k > step {
                k -= step + 1;
                res += 1;
            } else {
                k -= 1;
                res *= 10;
            }
        }

        return res;
    }

    fn step(root: i32, n: i32) -> i32 {
        let root = root as i64;
        let n = n as i64;
        let mut count = 0i64;
        let mut first = root * 10;
        let mut last = root * 10 + 9;

        while first <= n {
            count += std::cmp::min(last, n) - first + 1;
            first = first * 10;
            last = last * 10 + 9;
        }

        return count as i32;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_number(1, 1), 1);
    assert_eq!(Solution::find_kth_number(2, 2), 2);
    assert_eq!(Solution::find_kth_number(13, 2), 10);
}
