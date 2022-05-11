use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub struct Codec;

impl Codec {
    pub fn new() -> Self {
        Codec {}
    }

    pub fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(node) => {
                format!(
                    "{} {} {}",
                    node.borrow().val.to_string().as_str(),
                    self.serialize(node.borrow().left.clone()),
                    self.serialize(node.borrow().right.clone()).as_str()
                )
            }
            None => "".to_owned(),
        }
    }

    pub fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let data: Vec<i32> = data
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        self.construct(&data)
    }

    fn construct(&self, data: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }

        let val = *data.first().unwrap();
        let end = data.iter().position(|&x| x > val).unwrap_or(data.len());
        let left = self.construct(&data[1..end]);
        let right = self.construct(&data[end..]);

        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let codec = Codec::new();

        let node = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        let ret = codec.serialize(node.clone());

        assert_eq!(codec.deserialize(ret), node);
    }
}
