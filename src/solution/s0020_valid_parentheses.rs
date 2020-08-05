pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: VecDeque<char> = VecDeque::new();

        for c in s.chars() {
            match c {
                '(' => {
                    stack.push_back(c);
                }

                '[' => {
                    stack.push_back(c);
                }

                '{' => {
                    stack.push_back(c);
                }

                ')' => {
                    if stack.back().is_some() && stack.back().unwrap() == &'(' {
                        stack.pop_back();
                    } else {
                        return false;
                    }
                }

                ']' => {
                    if stack.back().is_some() && stack.back().unwrap() == &'[' {
                        stack.pop_back();
                    } else {
                        return false;
                    }
                }

                '}' => {
                    if stack.back().is_some() && stack.back().unwrap() == &'{' {
                        stack.pop_back();
                    } else {
                        return false;
                    }
                }
                _ => {
                    return false;
                }
            }
        }

        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0020() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
        assert_eq!(Solution::is_valid(String::from("(])]")), false);
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    }
}
