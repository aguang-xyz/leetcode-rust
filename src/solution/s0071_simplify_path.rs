pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut ret: VecDeque<String> = VecDeque::new();

        for p in path.split('/') {
            match p {
                "." => {}
                ".." => {
                    ret.pop_back();
                }
                "" => {}
                _ => {
                    ret.push_back(p.to_string());
                }
            }
        }

        format!("/{}", ret.into_iter().collect::<Vec<String>>().join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0071() {
        assert_eq!(
            Solution::simplify_path(String::from("/home/")),
            String::from("/home")
        );

        assert_eq!(
            Solution::simplify_path(String::from("/../")),
            String::from("/")
        );

        assert_eq!(
            Solution::simplify_path(String::from("/home//foo/")),
            String::from("/home/foo")
        );

        assert_eq!(
            Solution::simplify_path(String::from("/a/./b/../../c/")),
            String::from("/c")
        );
    }
}
