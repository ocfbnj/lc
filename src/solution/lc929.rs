pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut emails = emails
            .iter()
            .map(|x| x.split('@').collect::<Vec<_>>())
            .filter(|x| x.len() == 2)
            .map(|x| {
                let res = match x[0].split_once('+') {
                    Some((a, _)) => a,
                    None => x[0],
                };

                res.split('.').collect::<String>() + "@" + &x[1]
            })
            .collect::<Vec<_>>();

        emails.sort();

        fn unique(arr: &mut [String]) -> usize {
            let mut i = 0;
            let mut j = 1;

            while j < arr.len() {
                if arr[i] != arr[j] {
                    i += 1;
                    arr.swap(i, j);
                }

                j += 1;
            }

            i + 1
        }

        unique(&mut emails) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_owned(),
                "test.e.mail+bob.cathy@leetcode.com".to_owned(),
                "testemail+david@lee.tcode.com".to_owned(),
            ]),
            2
        );

        assert_eq!(
            Solution::num_unique_emails(vec![
                "a@leetcode.com".to_owned(),
                "b@leetcode.com".to_owned(),
                "c@leetcode.com".to_owned(),
            ]),
            3
        );

        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".to_owned(),
                "test.email.leet+alex@code.com".to_owned(),
            ]),
            2
        );
    }
}
