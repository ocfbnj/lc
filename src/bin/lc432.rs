struct AllOne {
    hash_map: std::collections::HashMap<String, usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    fn new() -> Self {
        AllOne {
            hash_map: std::collections::HashMap::new(),
        }
    }

    fn inc(&mut self, key: String) {
        *self.hash_map.entry(key).or_insert(0) += 1;
    }

    fn dec(&mut self, key: String) {
        let value = self.hash_map.get_mut(&key).unwrap();
        *value -= 1;
        if *value == 0 {
            self.hash_map.remove(&key);
        }
    }

    fn get_max_key(&self) -> String {
        match self.hash_map.iter().max_by_key(|x| x.1) {
            Some(item) => item.0.clone(),
            None => "".to_owned(),
        }
    }

    fn get_min_key(&self) -> String {
        match self.hash_map.iter().min_by_key(|x| x.1) {
            Some(item) => item.0.clone(),
            None => "".to_owned(),
        }
    }
}

fn main() {
    let mut all_one = AllOne::new();
    all_one.inc("hello".to_owned());
    all_one.inc("hello".to_owned());
    assert_eq!(all_one.get_max_key(), "hello".to_owned());
    assert_eq!(all_one.get_min_key(), "hello".to_owned());
    all_one.inc("leet".to_owned());
    assert_eq!(all_one.get_max_key(), "hello".to_owned());
    assert_eq!(all_one.get_min_key(), "leet".to_owned());
    all_one.dec("leet".to_owned());
    assert_eq!(all_one.get_min_key(), "hello".to_owned());
}
