pub struct Solution;

impl Solution {
    pub fn number_of_matches(mut n: i32) -> i32 {
        let mut res = 0;
        while n != 1 {
            let odd = (n & 1) == 1;
            if odd {
                n -= 1;
            }

            n >>= 1;
            res += n;

            if odd {
                n += 1;
            }
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_matches(1), 0);
    assert_eq!(Solution::number_of_matches(2), 1);
    assert_eq!(Solution::number_of_matches(7), 6);
    assert_eq!(Solution::number_of_matches(14), 13);

    for i in 1..=200 {
        println!("{}", Solution::number_of_matches(i));
    }
}
