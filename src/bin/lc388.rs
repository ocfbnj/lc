struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        let mut res = 0;
        let mut st: Vec<&str> = Vec::new();
        let mut sum = 0;

        for line in input.split('\n') {
            let name = line.trim_start_matches('\t');

            let depth = line.len() - name.len();
            while st.len() > depth {
                sum -= st.pop().unwrap().len();
            }

            sum += name.len();
            if name.contains('.') && sum + st.len() > res {
                res = sum + st.len();
            }

            st.push(name);
        }

        return res as i32;
    }
}

fn main() {
    assert_eq!(
        Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_owned()),
        20
    );

    assert_eq!(
        Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_owned()),
        32
    );

    assert_eq!(Solution::length_longest_path("a".to_owned()), 0);

    assert_eq!(
        Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_owned()),
        12
    );
}
