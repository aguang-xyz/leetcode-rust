pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let digits: Vec<char> = digits.chars().collect();

        let mut ret: Vec<String> = vec![];

        let mut que: VecDeque<(usize, Vec<char>)> = VecDeque::new();

        que.push_back((0, vec![]));

        while que.len() > 0 {
            let x = que.pop_front().unwrap();

            if x.0 == digits.len() {
                if x.1.len() > 0 {
                    ret.push(x.1.iter().collect());
                }
            } else {
                let choices: Vec<char> = match digits[x.0] {
                    '2' => vec!['a', 'b', 'c'],
                    '3' => vec!['d', 'e', 'f'],
                    '4' => vec!['g', 'h', 'i'],
                    '5' => vec!['j', 'k', 'l'],
                    '6' => vec!['m', 'n', 'o'],
                    '7' => vec!['p', 'q', 'r', 's'],
                    '8' => vec!['t', 'u', 'v'],
                    '9' => vec!['w', 'x', 'y', 'z'],
                    _ => vec![],
                };

                for c in choices {
                    let mut s = x.1.clone();

                    s.push(c);
                    que.push_back((x.0 + 1, s));
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
    fn test_0017() {
        let ret: Vec<String> = vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .into_iter()
            .map(|s| String::from(s))
            .collect();

        assert_eq!(Solution::letter_combinations(String::from("23")), ret);

        assert_eq!(Solution::letter_combinations(String::from("")).len(), 0);
    }
}
