pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let s: String = match x > 0 {
            true => x.to_string().chars().rev().collect(),
            false => String::from("-") + &(x.to_string()[1..].chars().rev().collect::<String>()),
        };

        s.parse::<i32>().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0001() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(1534236469), 0);
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}
