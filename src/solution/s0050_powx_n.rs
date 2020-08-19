pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            1.0
        } else if n < 0 {
            let mut ans: f64 = Self::my_pow(x, n / 2);

            ans *= ans;

            if n % 2 == -1 {
                ans /= x;
            }

            ans
        } else {
            let mut ans: f64 = Self::my_pow(x, n / 2);

            ans *= ans;

            if n % 2 == 1 {
                ans *= x;
            }

            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0050() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);

        assert!((Solution::my_pow(2.1, 3) - 9.261).abs() < 1e-5);
        assert!((Solution::my_pow(2.0, -2) - 0.25).abs() < 1e-5);
        assert!((Solution::my_pow(1.0, -2147483648) - 1.0).abs() < 1e-5);
    }
}
