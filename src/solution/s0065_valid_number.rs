pub struct Solution {}

impl Solution {
    pub fn is_number(s: String) -> bool {
        match s.trim().parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0065() {
        assert_eq!(Solution::is_number(String::from("0")), true);
        assert_eq!(Solution::is_number(String::from("-90e3")), true);
        assert_eq!(Solution::is_number(String::from("-+3")), false);
        assert_eq!(Solution::is_number(String::from("1 ")), true);
    }
}
