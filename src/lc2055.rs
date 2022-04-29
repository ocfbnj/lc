pub struct Solution;

impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let mut sum = 0;
        let mut prefix = vec![0; n];

        for i in 0..n {
            sum += if s[i] == b'*' { 1 } else { 0 };
            prefix[i] = sum;
        }

        let mut left = vec![-1; n];
        let mut l = -1;
        for i in 0..n {
            if s[i] == b'|' {
                l = i as i32;
            }
            left[i] = l;
        }

        let mut right = vec![-1; n];
        let mut r = -1;
        for i in (0..n).rev() {
            if s[i] == b'|' {
                r = i as i32;
            }
            right[i] = r;
        }

        let mut res = vec![];
        for query in queries {
            let a = right[query[0] as usize];
            let b = left[query[1] as usize];

            let val = if a == -1 || b == -1 || a >= b {
                0
            } else {
                prefix[b as usize] - prefix[a as usize]
            };

            res.push(val);
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::plates_between_candles("**|**|***|".to_owned(), vec![vec![2, 5], vec![5, 9]]),
        vec![2, 3]
    );

    assert_eq!(
        Solution::plates_between_candles(
            "***|**|*****|**||**|*".to_owned(),
            vec![
                vec![1, 17],
                vec![4, 5],
                vec![14, 17],
                vec![5, 11],
                vec![15, 16]
            ]
        ),
        vec![9, 0, 0, 0, 0]
    );
}
