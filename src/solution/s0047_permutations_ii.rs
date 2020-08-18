pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::permute(nums)
            .iter()
            .cloned()
            .collect::<HashSet<Vec<i32>>>()
            .iter()
            .cloned()
            .collect()
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() <= 1 {
            vec![nums.clone()]
        } else {
            let mut ans: Vec<Vec<i32>> = vec![];

            for pos in 0..nums.len() {
                let sub_rets: Vec<Vec<i32>> = Self::permute(
                    (0..nums.len())
                        .filter(|&x| x != pos)
                        .map(|pos| nums[pos])
                        .collect(),
                );

                for sub_ret in sub_rets.iter() {
                    let mut new_ret = sub_ret.clone();
                    new_ret.insert(0, nums[pos]);
                    ans.push(new_ret);
                }
            }

            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0047() {
        let ret = Solution::permute_unique(vec![1, 1, 2]);

        for v in vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]].iter() {
            assert!(ret.contains(v));
        }
    }
}
