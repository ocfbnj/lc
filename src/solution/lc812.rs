pub struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let points = Self::andrew(points);

        let n = points.len();
        let mut res = f64::MIN;

        for x in 0..n {
            for y in x + 1..n {
                for z in y + 1..n {
                    res = res.max(Self::triangle_area(&points[x], &points[y], &points[z]));
                }
            }
        }

        res
    }

    fn cross(o: &[i32], a: &[i32], b: &[i32]) -> i32 {
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    }

    fn triangle_area(a: &[i32], b: &[i32], c: &[i32]) -> f64 {
        0.5 * Self::cross(a, b, c).abs() as f64
    }

    fn andrew(mut points: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if points.len() <= 3 {
            return points;
        }

        points.sort();

        let mut res = vec![];

        let mut st = vec![&points[0], &points[1]];
        for x in points.iter().skip(2) {
            while st.len() >= 2 && Self::cross(&st[st.len() - 2], &st[st.len() - 1], &x) <= 0 {
                st.pop();
            }

            st.push(x);
        }
        res.append(&mut st.into_iter().skip(1).cloned().collect());

        let mut st = vec![&points[points.len() - 1], &points[points.len() - 2]];
        for x in points.iter().rev().skip(2) {
            while st.len() >= 2 && Self::cross(&st[st.len() - 2], &st[st.len() - 1], &x) <= 0 {
                st.pop();
            }

            st.push(x);
        }
        res.append(&mut st.into_iter().skip(1).cloned().collect());

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0
        );

        assert_eq!(
            Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]),
            0.5
        );
    }
}
