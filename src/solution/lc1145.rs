use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn find_node(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = &root {
                if node.borrow().val == x {
                    return Some(node.clone());
                }

                let left = find_node(node.borrow().left.clone(), x);
                if left != None {
                    return left;
                }

                return find_node(node.borrow().right.clone(), x);
            }

            None
        }

        let node = find_node(root.clone(), x);

        fn count(root: Rc<RefCell<TreeNode>>) -> (i32, i32) {
            let mut left = 0;
            let mut right = 0;

            if let Some(l) = &root.borrow().left {
                let (a, b) = count(l.clone());
                left = a + b + 1;
            }

            if let Some(r) = &root.borrow().right {
                let (a, b) = count(r.clone());
                right = a + b + 1;
            }

            (left, right)
        }

        let (left, right) = count(node.unwrap());

        // up
        let count_b1 = n - (left + right + 1);
        let count_a1 = left + right + 1;

        // left or right
        let count_b2 = left.max(right);
        let count_a2 = n - count_b2;

        count_b1 > count_a1 || count_b2 > count_a2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::btree_game_winning_move(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 8,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 9,
                                left: None,
                                right: None
                            })))
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: 10,
                                left: None,
                                right: None
                            }))),
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 11,
                                left: None,
                                right: None
                            })))
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: None,
                            right: None
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 7,
                            left: None,
                            right: None
                        })))
                    })))
                }))),
                11,
                3
            ),
            true
        );

        assert_eq!(
            Solution::btree_game_winning_move(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None
                    })))
                }))),
                3,
                1
            ),
            false
        );
    }
}
