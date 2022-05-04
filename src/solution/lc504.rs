pub struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0 {
            return "0".to_owned();
        }

        let negative = num < 0;
        let mut arr = vec![];
        let mut num = num.abs();

        while num > 0 {
            arr.push((num % 7) as u8 + b'0');
            num /= 7;
        }

        if negative {
            arr.push(b'-');
        }
        arr.reverse();

        return String::from_utf8(arr).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_to_base7(0), "0".to_owned());
        assert_eq!(Solution::convert_to_base7(100), "202".to_owned());
        assert_eq!(Solution::convert_to_base7(-7), "-10".to_owned());
    }
}
