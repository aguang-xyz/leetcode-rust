pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Convert string to chars.
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        // Patterns: [(character, is_zero_or_more)].
        let mut patterns: Vec<(char, bool)> = vec![];

        // Parse p to patterns.
        for i in 0..p.len() {
            if p[i] == '*' {
                continue;
            }

            patterns.push((p[i], i + 1 < p.len() && p[i + 1] == '*'))
        }

        // f[i][j] indicates wether the sub-pattern starting from i match with
        // the sub-string starting from j.
        let mut f: Vec<Vec<bool>> = vec![vec![false; s.len() + 1]; patterns.len() + 1];

        for i in (0..patterns.len() + 1).rev() {
            for j in (0..s.len() + 1).rev() {
                if i == patterns.len() {
                    f[i][j] = j == s.len()
                } else if j == s.len() {
                    f[i][j] = patterns[i].1 && f[i + 1][j]
                } else if patterns[i].1 == false {
                    f[i][j] = (patterns[i].0 == '.' || patterns[i].0 == s[j]) && f[i + 1][j + 1]
                } else {
                    f[i][j] = f[i + 1][j];

                    let mut k: usize = j;

                    while f[i][j] == false
                        && k < s.len()
                        && (patterns[i].0 == '.' || patterns[i].0 == s[k])
                    {
                        if f[i + 1][k + 1] {
                            f[i][j] = true
                        }

                        k = k + 1
                    }
                }
            }
        }

        f[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0010() {
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a")),
            false
        );
        assert_eq!(
            Solution::is_match(String::from("aa"), String::from("a*")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("ab"), String::from(".*")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("aab"), String::from("c*a*b")),
            true
        );
        assert_eq!(
            Solution::is_match(String::from("mississippi"), String::from("mis*is*p*.")),
            false
        );
        assert_eq!(
            Solution::is_match(String::from("ab"), String::from(".*c")),
            false
        );
    }
}
