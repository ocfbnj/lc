pub struct Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut index = HashMap::with_capacity(list1.len());

        for (i, item) in list1.iter().enumerate() {
            index.insert(item.as_str(), i);
        }

        let mut res = vec![];
        let mut sum = usize::MAX;

        for (i, item) in list2.iter().enumerate() {
            if let Some(j) = index.get(item.as_str()) {
                let s = i + j;

                if s <= sum {
                    if s < sum {
                        sum = s;
                        res.clear();
                    }
                    res.push(item.clone());
                }
            }
        }

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_owned(),
                "Tapioca Express".to_owned(),
                "Burger King".to_owned(),
                "KFC".to_owned(),
            ],
            vec![
                "Piatti".to_owned(),
                "The Grill at Torrey Pines".to_owned(),
                "Hungry Hunter Steakhouse".to_owned(),
                "Shogun".to_owned(),
            ],
        ),
        vec!["Shogun".to_owned()]
    );
}
