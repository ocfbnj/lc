pub struct Solution;

impl Solution {
    pub fn is_valid(code: String) -> bool {
        let data = code.as_bytes();
        let mut st = Vec::<&str>::new();

        let mut i = 0;
        while i < code.len() {
            if data[i] == b'<' {
                if i + 1 >= code.len() {
                    return false;
                }

                if data[i + 1].is_ascii_uppercase() {
                    let j = match code[i + 1..].find('>') {
                        Some(j) => i + 1 + j,
                        None => return false,
                    };

                    let name = &code[i + 1..j];
                    if !name.chars().all(|c| c.is_ascii_uppercase())
                        || name.len() < 1
                        || name.len() > 9
                    {
                        return false;
                    }

                    st.push(name);
                    i = j + 1;
                } else if data[i + 1] == b'/' {
                    let j = match code[i + 2..].find('>') {
                        Some(j) => i + 2 + j,
                        None => return false,
                    };

                    let name = &code[i + 2..j];
                    if st.pop() != Some(name) {
                        return false;
                    }

                    i = j + 1;
                    if st.is_empty() && i < code.len() {
                        return false;
                    }
                } else if data[i + 1] == b'!' {
                    if st.is_empty() {
                        return false;
                    }

                    if &code[i + 2..(i + 2 + 7).min(code.len())] != "[CDATA[" {
                        return false;
                    }

                    match code[i + 2 + 7..].find("]]>") {
                        Some(j) => i = i + 2 + 7 + j + 3,
                        None => return false,
                    }
                } else {
                    return false;
                }
            } else {
                if st.is_empty() {
                    return false;
                }
                i += 1;
            }
        }

        return st.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::is_valid("<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_owned()),
            true
        );

        assert_eq!(
            Solution::is_valid("<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_owned()),
            true
        );

        assert_eq!(Solution::is_valid("<A>  <B> </A>   </B>".to_owned()), false);

        assert_eq!(Solution::is_valid("<A></A><B></B>".to_owned()), false);

        assert_eq!(Solution::is_valid("<A><!A></A>".to_owned()), false);

        assert_eq!(
            Solution::is_valid("<AAAAAAAAAA></AAAAAAAAAA>".to_owned()),
            false
        );

        assert_eq!(
            Solution::is_valid("<DIV>  unmatched <  </DIV>".to_owned()),
            false
        );

        assert_eq!(
            Solution::is_valid("<![CDATA[wahaha]]]><![CDATA[]> wahaha]]>".to_owned()),
            false
        );
    }
}
