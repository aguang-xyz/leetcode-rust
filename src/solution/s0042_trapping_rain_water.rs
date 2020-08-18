pub struct Solution {}

use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;
        let mut left: usize = 0;

        while left + 2 < height.len() {
            let mut best_right: usize = left + 1;

            let mut maxh: i32 = i32::min_value();

            for right in (left + 1)..height.len() {
                if min(height[left], height[right]) > maxh {
                    best_right = right;
                }

                maxh = max(maxh, height[right]);
            }

            for i in (left + 1)..best_right {
                ans += min(height[left], height[best_right]) - height[i];
            }

            left = best_right;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0042() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
