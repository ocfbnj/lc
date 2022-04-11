struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let arr = n.as_bytes();
        let len = arr.len();
        let mut candidates = vec![10_i64.pow(len as u32 - 1) - 1, 10_i64.pow(len as u32) + 1];
        let prefix: i64 = unsafe { String::from_utf8_unchecked(arr[0..(len + 1) / 2].to_vec()) }
            .parse()
            .unwrap();

        for num in [prefix - 1, prefix, prefix + 1] {
            let str = num.to_string();
            let candidate: String =
                str.clone() + &str.chars().rev().skip(len & 1).collect::<String>();
            candidates.push(candidate.parse().unwrap());
        }

        let origin: i64 = n.parse().unwrap();
        let mut candidates = candidates.iter().filter(|&&x| x != origin);
        let mut res = candidates.next().unwrap().clone();

        for &candidate in candidates {
            if (origin - candidate).abs() < (origin - res).abs() {
                res = candidate;
            }
        }

        res.to_string()
    }
}

fn main() {
    assert_eq!(
        Solution::nearest_palindromic(String::from("123")),
        String::from("121")
    );

    assert_eq!(
        Solution::nearest_palindromic(String::from("1")),
        String::from("0")
    );

    assert_eq!(
        Solution::nearest_palindromic(String::from("8")),
        String::from("7")
    );

    assert_eq!(
        Solution::nearest_palindromic(String::from("10")),
        String::from("9")
    );
}
