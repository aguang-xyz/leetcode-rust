pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() > 0 {
            let chs: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();

            for i in 0..strs[0].len() {
                if strs.len()
                    > chs
                        .iter()
                        .map(|s| {
                            if i < s.len() && s[i] == chs[0][i] {
                                1
                            } else {
                                0
                            }
                        })
                        .sum()
                {
                    return strs[0][0..i].to_string();
                }
            }

            return strs[0].clone();
        }

        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0014() {
        assert_eq!(Solution::longest_common_prefix(vec![]), String::from(""));

        assert_eq!(
            Solution::longest_common_prefix(vec![String::from("a")]),
            String::from("a")
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );
    }
}
