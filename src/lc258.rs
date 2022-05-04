pub struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        while num >= 10 {
            num = num
                .to_string()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .sum();
        }

        if num < 10 {
            return num;
        }

        num = num % 9;
        if num == 0 {
            9
        } else {
            num
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(test)]
    fn test() {
        assert_eq!(Solution::add_digits(38), 2);
        assert_eq!(Solution::add_digits(0), 0);
    }
}
