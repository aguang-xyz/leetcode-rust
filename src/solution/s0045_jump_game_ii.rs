pub struct Solution {}

use std::cmp::max;
use std::cmp::min;

use std::collections::VecDeque;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut vis: Vec<bool> = vec![false; nums.len()];
        let mut dis: Vec<i32> = vec![i32::max_value(); nums.len()];

        let mut que: VecDeque<usize> = VecDeque::new();

        vis[0] = true;
        dis[0] = 0;

        que.push_back(0 as usize);

        while !que.is_empty() {
            let top = que.pop_front().unwrap();

            vis[top] = false;

            let left = max(0, top as i32 - nums[top]) as usize;
            let right = min(top + nums[top] as usize + 1, nums.len());

            for tgt in left..right {
                if dis[top] + 1 < dis[tgt] {
                    dis[tgt] = dis[top] + 1;

                    if !vis[tgt] {
                        que.push_back(tgt);
                        vis[tgt] = true;
                    }
                }
            }
        }

        dis[nums.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0045() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }
}
