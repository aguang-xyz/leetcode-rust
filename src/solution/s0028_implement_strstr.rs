pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).map_or(-1, |x| x as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0028() {
        assert_eq!(
            Solution::str_str(String::from("hello"), String::from("ll")),
            2
        );
        assert_eq!(
            Solution::str_str(String::from("aaaa"), String::from("bba")),
            -1
        );
    }
}
