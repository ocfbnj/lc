struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        // f(n) = n + 1 - f1(n)

        // if n is even
        // suppose one round is executed first
        // [1, 2, 3, 4, 5, 6] => [2, 4, 6]
        // so the length is n / 2 = 3
        // now we need to execute it with f1(3) ([1, 2, 3])
        // we don't care about the result of f1(3), we just need to adjust [2, 4, 6] to [1, 2, 3]
        // so f(n) = 2 * f1(n / 2)
        //
        // f(2n) = 2 * f1(n)
        // f(n) = n + 1 - (f(2n) / 2)
        // f(n / 2) = n / 2 + 1 - (f(n) / 2)
        // f(n) / 2 = 1 + n / 2 - f(n / 2)
        // f(n) = 2 * (1 + n / 2 - f(n / 2))

        match n {
            1 => 1,
            _ => 2 * (1 + n / 2 - Solution::last_remaining(n / 2)),
        }
    }
}

fn main() {
    assert_eq!(Solution::last_remaining(9), 6);
    assert_eq!(Solution::last_remaining(1), 1);
}
