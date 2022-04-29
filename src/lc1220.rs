pub struct Solution;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        use std::collections::HashMap;

        let mut dp = HashMap::from([
            (b'a', vec![1; n as usize + 1]),
            (b'e', vec![1; n as usize + 1]),
            (b'i', vec![1; n as usize + 1]),
            (b'o', vec![1; n as usize + 1]),
            (b'u', vec![1; n as usize + 1]),
        ]);

        let m = 1e9 as i32 + 7;

        for i in 2..=n as usize {
            dp.entry(b'a').or_default()[i] =
                (((dp[&b'e'][i - 1] + dp[&b'i'][i - 1]) % m) + dp[&b'u'][i - 1]) % m;
            dp.entry(b'e').or_default()[i] = (dp[&b'a'][i - 1] + dp[&b'i'][i - 1]) % m;
            dp.entry(b'i').or_default()[i] = (dp[&b'e'][i - 1] + dp[&b'o'][i - 1]) % m;
            dp.entry(b'o').or_default()[i] = dp[&b'i'][i - 1];
            dp.entry(b'u').or_default()[i] = (dp[&b'i'][i - 1] + dp[&b'o'][i - 1]) % m;
        }

        dp.values()
            .map(|item| item[n as usize])
            .reduce(|accum, item| (accum + item) % m)
            .unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_vowel_permutation(1), 5);
    assert_eq!(Solution::count_vowel_permutation(2), 10);
    assert_eq!(Solution::count_vowel_permutation(5), 68);
    assert_eq!(Solution::count_vowel_permutation(20000), 759959057);
}
