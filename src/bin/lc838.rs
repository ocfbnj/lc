struct Solution;

impl Solution {
    pub fn push_dominoes(mut dominoes: String) -> String {
        let n = dominoes.len();
        let arr = unsafe { dominoes.as_bytes_mut() };

        let mut left = b'L';
        let mut i = 0;

        while i < n {
            let mut j = i;

            while j < n && arr[j] == b'.' {
                j += 1;
            }

            let right = if j < n { arr[j] } else { b'R' };
            if left == right {
                arr[i..j].fill(left);
            } else if left == b'R' && right == b'L' {
                let mut k = j - 1;

                while i < k {
                    arr[i] = left;
                    arr[k] = right;
                    i += 1;
                    k -= 1;
                }
            }

            left = right;
            i = j + 1;
        }

        return dominoes;
    }
}

fn main() {
    assert_eq!(
        Solution::push_dominoes("RR.L".to_owned()),
        "RR.L".to_owned()
    );

    assert_eq!(
        Solution::push_dominoes(".L.R...LR..L..".to_owned()),
        "LL.RR.LLRRLL..".to_owned()
    );

    println!("{}", std::mem::size_of::<[i32; 2]>());
}
