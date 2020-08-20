pub struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        (x as f64).sqrt().trunc() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0069() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
