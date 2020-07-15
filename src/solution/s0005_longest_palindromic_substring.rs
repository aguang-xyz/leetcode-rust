pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let chs: Vec<char> = s.chars().collect();
        let mut ret: String = String::from("");

        // xax
        for i in 0..chs.len() {
            let (mut p, mut q) = (i, i);

            while p > 0 && q + 1 < chs.len() && chs[p - 1] == chs[q + 1] {
                p = p - 1;
                q = q + 1;
            }

            if q - p + 1 > ret.len() {
                ret = s[p..(q + 1)].to_string();
            }
        }

        // xx
        for i in 1..chs.len() {
            if chs[i - 1] == chs[i] {
                let (mut p, mut q) = (i - 1, i);

                while p > 0 && q + 1 < chs.len() && chs[p - 1] == chs[q + 1] {
                    p = p - 1;
                    q = q + 1;
                }

                if q - p + 1 > ret.len() {
                    ret = s[p..(q + 1)].to_string();
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0005() {
        assert_eq!(
            Solution::longest_palindrome(String::from("babad")),
            String::from("bab")
        );

        assert_eq!(
            Solution::longest_palindrome(String::from("cbbd")),
            String::from("bb")
        );
    }
}
