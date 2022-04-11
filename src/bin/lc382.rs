use lc::types::*;

struct Solution {
    head: Option<Box<ListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        Solution { head }
    }

    fn get_random(&self) -> i32 {
        use rand::Rng;

        let mut i = 1;
        let mut res = self.head.as_ref().unwrap().val;
        let mut node = &self.head.as_ref().unwrap().next;

        while let Some(cur_node) = node {
            let j = rand::thread_rng().gen_range(0..i + 1);
            if j == 0 {
                res = cur_node.val;
            }

            i += 1;
            node = &cur_node.next;
        }

        res
    }
}

fn main() {
    let node = Some(Box::new(ListNode {
        val: 0,
        next: Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        })),
    }));

    let solution = Solution::new(node);

    for _ in 0..5 {
        println!("{}", solution.get_random());
    }
}
