pub struct Solution {}

use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chs: Vec<char> = s.chars().collect();
        let mut set: HashSet<char> = HashSet::new();
        let mut ret: i32 = 0;

        let mut i = 0;

        for j in 0..(chs.len()) {
            while set.contains(&chs[j]) {
                set.remove(&chs[i]);
                i += 1;
            }

            set.insert(chs[j]);

            ret = max(ret, (j - i + 1) as i32);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0003() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );

        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
