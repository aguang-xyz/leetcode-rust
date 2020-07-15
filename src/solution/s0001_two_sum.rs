pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (idx, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, idx);
                }

                Some(idx2) => {
                    return vec![*idx2 as i32, idx as i32];
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_s0001() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
