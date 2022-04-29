pub struct Solution;

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        properties.sort_by(|a, b| {
            if a[0] == b[0] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });

        let mut max = i32::MIN;
        let mut res = 0;

        for property in properties.iter() {
            if property[1] < max {
                res += 1;
            }

            max = std::cmp::max(max, property[1]);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::number_of_weak_characters(vec![vec![5, 5], vec![6, 3], vec![3, 6]]),
        0
    );
}
