pub struct Solution;

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let xy = grid.iter().flatten().filter(|&&x| x != 0).count() as i32;

        let yz = grid
            .iter()
            .map(|x| x.iter().max().unwrap())
            .filter(|&&x| x != 0)
            .sum::<i32>();

        let mut first = grid.first().unwrap().clone();
        let n = first.len();
        grid.iter().skip(1).for_each(|x| {
            for i in 0..n {
                first[i] = std::cmp::max(first[i], x[i]);
            }
        });

        let zx = first.iter().sum::<i32>();

        return xy + yz + zx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
        assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
        assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
