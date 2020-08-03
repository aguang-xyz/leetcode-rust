pub struct Solution;

use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;

        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                ans = max(ans, (j - i) as i32 * min(height[i], height[j]))
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0011() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
