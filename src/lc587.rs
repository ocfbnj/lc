pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::andrew(trees)
    }

    fn cross(o: &[i32], a: &[i32], b: &[i32]) -> i32 {
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    }

    fn distance(a: &[i32], b: &[i32]) -> i32 {
        return (a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1]);
    }

    fn graham(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() <= 3 {
            return trees;
        }

        let mut x = 0;
        for (i, item) in trees.iter().enumerate().skip(1) {
            if item[1] < trees[x][1] || item[1] == trees[x][1] && item[0] < trees[x][0] {
                x = i;
            }
        }
        trees.swap(0, x);

        let origin = trees[0].clone();
        trees[1..].sort_by(|a, b| match Self::cross(&origin, &b, &a).cmp(&0) {
            std::cmp::Ordering::Equal => {
                Self::distance(&origin, &a).cmp(&Self::distance(&origin, &b))
            }
            ret => ret,
        });

        let mut i = trees.len() - 2;
        while i > 0 && Self::cross(&origin, &trees[i], &trees[trees.len() - 1]) == 0 {
            i -= 1;
        }
        trees[i + 1..].reverse();

        let mut st = vec![&trees[0], &trees[1], &trees[2]];
        for x in trees.iter().skip(3) {
            while Self::cross(&st[st.len() - 2], &st[st.len() - 1], &x) < 0 {
                st.pop();
            }

            st.push(x);
        }

        return st.into_iter().cloned().collect();
    }

    fn andrew(mut trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() <= 3 {
            return trees;
        }
        let a = trees[0][0];
        let b = trees[0][1];

        if trees.iter().filter(|x| x[0] == a).count() == trees.len()
            || trees.iter().filter(|x| x[1] == b).count() == trees.len()
        {
            return trees;
        }

        trees.sort();

        let mut res = vec![];

        let mut st = vec![&trees[0], &trees[1]];
        for x in trees.iter().skip(2) {
            while st.len() >= 2 && Self::cross(&st[st.len() - 2], &st[st.len() - 1], &x) < 0 {
                st.pop();
            }

            st.push(x);
        }
        res.append(&mut st.into_iter().skip(1).cloned().collect());

        let mut st = vec![&trees[trees.len() - 1], &trees[trees.len() - 2]];
        for x in trees.iter().rev().skip(2) {
            while st.len() >= 2 && Self::cross(&st[st.len() - 2], &st[st.len() - 1], &x) < 0 {
                st.pop();
            }

            st.push(x);
        }

        res.append(&mut st.into_iter().skip(1).cloned().collect());

        return res;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::outer_trees(vec![
            vec![1, 1],
            vec![2, 2],
            vec![2, 0],
            vec![2, 4],
            vec![3, 3],
            vec![4, 2]
        ]),
        vec![vec![2, 0], vec![4, 2], vec![3, 3], vec![2, 4], vec![1, 1]]
    );

    assert_eq!(
        Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]),
        vec![vec![1, 2], vec![2, 2], vec![4, 2]]
    );
}
