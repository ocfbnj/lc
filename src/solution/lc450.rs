use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub struct Solution;

impl Solution {
    pub fn delete_node(
        root: Option<Rc<RefCell<TreeNode>>>,
        key: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut cur = root.as_ref().map(Rc::clone);
        let mut parent = None;
        let mut at_left = false;

        while let Some(node) = cur.as_ref().map(Rc::clone) {
            let val = node.borrow().val;

            match val.cmp(&key) {
                std::cmp::Ordering::Less => {
                    parent = Some(Rc::clone(&node));
                    cur = node.borrow().right.as_ref().map(Rc::clone);
                    at_left = false;
                }
                std::cmp::Ordering::Greater => {
                    parent = Some(Rc::clone(&node));
                    cur = node.borrow().left.as_ref().map(Rc::clone);
                    at_left = true;
                }
                std::cmp::Ordering::Equal => break,
            }
        }

        if let Some(node) = cur {
            let node: &mut TreeNode = &mut node.borrow_mut();

            let p = match (node.left.is_none(), node.right.is_none()) {
                (true, true) => None,
                (true, false) => node.right.take(),
                (false, true) => node.left.take(),
                (false, false) => {
                    let left = node.left.take().unwrap();
                    let right = node.right.take().unwrap();

                    let mut prev = None;
                    let mut cur = Rc::clone(&right);

                    loop {
                        let next = cur.borrow().left.as_ref().map(Rc::clone);

                        if let Some(next) = next {
                            prev = Some(cur);
                            cur = next;
                        } else {
                            break;
                        }
                    }

                    {
                        let mut cur: &mut TreeNode = &mut cur.borrow_mut();

                        if let Some(prev) = prev {
                            prev.borrow_mut().left = cur.right.take();
                            cur.left = Some(left);
                            cur.right = Some(right);
                        } else {
                            cur.left = Some(left);
                        }
                    }

                    Some(cur)
                }
            };

            if let Some(parent) = parent {
                if at_left {
                    parent.borrow_mut().left = p;
                } else {
                    parent.borrow_mut().right = p;
                }
            } else {
                return p;
            }
        }

        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        let ret = Solution::delete_node(root, 3);
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(ret, root);
    }
}
