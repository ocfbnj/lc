use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub struct Solution;

impl Solution {
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut arr1 = Vec::new();
        let mut arr2 = Vec::new();

        Self::dfs(root1, &mut arr1);
        Self::dfs(root2, &mut arr2);

        return Self::merge(&arr1, &arr2);
    }

    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
        if let Some(ref node) = node {
            Self::dfs(node.borrow().left.clone(), vec);
            vec.push(node.borrow().val);
            Self::dfs(node.borrow().right.clone(), vec);
        }
    }

    fn merge(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
        let mut vec = Vec::with_capacity(arr1.len() + arr2.len());

        let mut i = 0;
        let mut j = 0;

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] <= arr2[j] {
                vec.push(arr1[i]);
                i += 1;
            } else {
                vec.push(arr2[j]);
                j += 1;
            }
        }

        while i < arr1.len() {
            vec.push(arr1[i]);
            i += 1;
        }

        while j < arr2.len() {
            vec.push(arr2[j]);
            j += 1;
        }

        return vec;
    }
}

#[test]
fn test() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
    })));

    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));

    assert_eq!(
        Solution::get_all_elements(root1, root2),
        vec![0, 1, 1, 2, 3, 4]
    );

    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode::new(8)))),
    })));

    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: 8,
        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
        right: None,
    })));

    assert_eq!(Solution::get_all_elements(root1, root2), vec![1, 1, 8, 8]);
}
