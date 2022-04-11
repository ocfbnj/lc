struct Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a != b {
            a.len().max(b.len()) as i32
        } else {
            -1
        }
    }
}

fn main() {
    assert_eq!(
        Solution::find_lu_slength("aba".to_owned(), "cdc".to_owned()),
        3
    );

    assert_eq!(
        Solution::find_lu_slength("aaa".to_owned(), "bbb".to_owned()),
        3
    );

    assert_eq!(
        Solution::find_lu_slength("aaa".to_owned(), "aaa".to_owned()),
        -1
    );

    assert_eq!(
        Solution::find_lu_slength("abc".to_owned(), "cba".to_owned()),
        3
    );
}
