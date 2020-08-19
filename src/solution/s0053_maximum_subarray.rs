pub struct Solution {}

use std::cmp::max;
use std::cmp::min;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let sums: Vec<i32> = nums.iter().fold(vec![0], |mut sums, &num| {
            sums.push(sums[sums.len() - 1] + num);
            sums
        });

        (1..sums.len())
            .fold((0, i32::min_value()), |(min_prev_sum, ans), idx| {
                (
                    min(min_prev_sum, sums[idx]),
                    max(ans, sums[idx] - min_prev_sum),
                )
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0053() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
