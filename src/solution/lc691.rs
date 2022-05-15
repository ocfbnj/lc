pub struct Solution;

impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let m = target.len();
        let mut saved = vec![-1; 1 << m];
        saved[0] = 0;

        let ret = Self::resolve((1 << m) - 1, &stickers, target.as_bytes(), &mut saved);

        if ret == (m + 1) as i32 {
            -1
        } else {
            ret
        }
    }

    fn resolve(mask: usize, stickers: &[String], target: &[u8], saved: &mut [i32]) -> i32 {
        if saved[mask] != -1 {
            return saved[mask];
        }

        saved[mask] = (target.len() + 1) as i32;

        for sticker in stickers.iter().map(|s| s.as_bytes()) {
            let mut counter = [0; 26];

            for c in sticker.iter() {
                counter[(c - b'a') as usize] += 1;
            }

            let mut new_mask = mask;
            for i in 0..target.len() {
                if (mask >> i) & 1 == 1 {
                    let c = target[i];
                    if counter[(c - b'a') as usize] > 0 {
                        counter[(c - b'a') as usize] -= 1;
                        new_mask &= !(1 << i);
                    }
                }
            }

            if new_mask != mask {
                saved[mask] = std::cmp::min(
                    saved[mask],
                    Self::resolve(new_mask, stickers, target, saved) + 1,
                );
            }
        }

        return saved[mask];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_stickers(
                vec![
                    "with".to_owned(),
                    "example".to_owned(),
                    "science".to_owned()
                ],
                "thehat".to_owned()
            ),
            3
        );

        assert_eq!(
            Solution::min_stickers(
                vec!["notice".to_owned(), "possible".to_owned()],
                "basicbasic".to_owned()
            ),
            -1
        );
    }
}
