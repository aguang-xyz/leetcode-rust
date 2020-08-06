pub struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut s: Vec<HashSet<String>> =
            vec![HashSet::from_iter(vec![String::from("")].into_iter())];

        for i in 1..(n + 1) {
            let mut si: HashSet<String> = HashSet::new();

            for j in 0..i {
                let p = &s[j as usize];
                let q = &s[(i - j - 1) as usize];

                for s1 in p {
                    for s2 in q {
                        si.insert(format!("({}){}", s1, s2).to_string());
                    }
                }
            }

            s.push(si);
        }

        s[n as usize].iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0022() {
        let ret = Solution::generate_parenthesis(3);

        for s in vec!["((()))", "(()())", "(())()", "()(())", "()()()"] {
            assert!(ret.contains(&s.to_string()));
        }
    }
}
