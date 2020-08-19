pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        match s.split_whitespace().last() {
            Some(word) => word.len() as i32,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0058() {
        assert_eq!(
            Solution::length_of_last_word(String::from("Hello World")),
            5
        );
    }
}
