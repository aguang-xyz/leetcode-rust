pub struct Solution {}

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        let is_match = |s, p| s == p || p == '?';

        let mut f: Vec<Vec<bool>> = vec![vec![false; p.len() + 1]; s.len() + 1];

        for i in (0..(s.len() + 1)).rev() {
            for j in (0..(p.len() + 1)).rev() {
                if i == s.len() && j == p.len() {
                    f[i][j] = true;
                } else if i == s.len() {
                    f[i][j] = p[j] == '*' && f[i][j + 1];
                } else if j != p.len() {
                    if p[j] == '*' {
                        f[i][j] = f[i][j + 1];

                        for k in i..s.len() {
                            f[i][j] = f[i][j] || f[k + 1][j + 1];
                        }
                    } else {
                        f[i][j] = is_match(s[i], p[j]) && f[i + 1][j + 1];
                    }
                }
            }
        }

        f[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0044() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a")),
            false
        );

        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("*")),
            true
        );

        assert_eq!(
            Solution::is_match(String::from("cb"), String::from("?a")),
            false
        );

        assert_eq!(
            Solution::is_match(String::from("adceb"), String::from("a*b")),
            true
        );

        assert_eq!(
            Solution::is_match(String::from("acdcb"), String::from("a*c?b")),
            false
        );

        assert_eq!(
            Solution::is_match(String::from(""), String::from("*")),
            true
        );

        assert_eq!(
            Solution::is_match(String::from("a"), String::from("a*")),
            true
        );
    }
}
