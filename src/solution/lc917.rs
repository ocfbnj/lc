pub struct Solution;

impl Solution {
    pub fn reverse_only_letters(mut s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let arr = unsafe { s.as_bytes_mut() };

        let mut i = 0;
        let mut j = arr.len() - 1;

        loop {
            while i < j && !arr[i].is_ascii_alphabetic() {
                i += 1;
            }

            while i < j && !arr[j].is_ascii_alphabetic() {
                j -= 1;
            }

            if i >= j {
                break;
            }

            arr.swap(i, j);

            i += 1;
            j -= 1;
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse_only_letters("".to_owned()), "");
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_owned()), "dc-ba");
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_owned()),
            "j-Ih-gfE-dCba"
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_owned()),
            "Qedo1ct-eeLg=ntse-T!"
        );
    }
}
