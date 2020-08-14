pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let res = ((dividend as f64) / (divisor as f64)).trunc();

        if res >= (i32::min_value() as f64) && res <= (i32::max_value() as f64) {
            res as i32
        } else {
            i32::max_value()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_0029() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    }
}
