use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Solution;

impl Solution {
    pub fn alien_order(words: Vec<String>) -> String {
        let mut nodes = HashMap::<char, Rc<RefCell<Node>>>::new();

        for c in words.iter().flat_map(|s| s.chars()) {
            nodes
                .entry(c)
                .or_insert(Rc::new(RefCell::new(Node::new(c))));
        }

        for i in 1..words.len() {
            let mut f = false;

            for (a, b) in words[i - 1].chars().zip(words[i].chars()) {
                if a != b {
                    let anode = nodes.get(&a).unwrap();
                    let bnode = nodes.get(&b).unwrap();
                    anode.borrow_mut().next.push(bnode.clone());

                    f = true;
                    break;
                }
            }

            if !f && words[i - 1].len() > words[i].len() {
                return "".to_owned();
            }
        }

        if nodes.is_empty() {
            return words[0].chars().next().unwrap().to_string();
        }

        let mut res = Vec::<char>::new();

        fn dfs(cur: Rc<RefCell<Node>>, res: &mut Vec<char>) -> bool {
            let status = cur.borrow().status;
            match status {
                Status::NoVisited => {
                    cur.borrow_mut().status = Status::Visiting;
                    for next in cur.borrow().next.iter() {
                        if !dfs(next.clone(), res) {
                            return false;
                        }
                    }
                    cur.borrow_mut().status = Status::Visited;
                    res.push(cur.borrow().val);

                    true
                }
                Status::Visiting => false,
                Status::Visited => true,
            }
        }

        for node in nodes.values() {
            if !dfs(node.clone(), &mut res) {
                return String::new();
            }
        }

        res.iter().rev().collect()
    }
}

struct Node {
    val: char,
    next: Vec<Rc<RefCell<Node>>>,
    status: Status,
}

impl Node {
    fn new(val: char) -> Self {
        Node {
            val,
            next: Vec::new(),
            status: Status::NoVisited,
        }
    }
}

#[derive(Clone, Copy)]
enum Status {
    NoVisited,
    Visiting,
    Visited,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::alien_order(vec![
                "wrt".to_owned(),
                "wrf".to_owned(),
                "er".to_owned(),
                "ett".to_owned(),
                "rftt".to_owned()
            ]),
            "wertf".to_owned()
        );

        assert_eq!(
            Solution::alien_order(vec!["z".to_owned(), "x".to_owned()]),
            "zx".to_owned()
        );

        assert_eq!(
            Solution::alien_order(vec!["z".to_owned(), "x".to_owned(), "z".to_owned()]),
            "".to_owned()
        );

        assert_eq!(
            Solution::alien_order(vec!["ab".to_owned(), "a".to_owned()]),
            "".to_owned()
        );

        assert_eq!(
            Solution::alien_order(vec!["z".to_owned(), "z".to_owned()]),
            "z".to_owned()
        );
    }
}
