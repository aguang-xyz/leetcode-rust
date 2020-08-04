pub struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        let nums: Vec<i32> = nums
            .into_iter()
            .filter(|&num| match cnt.get_mut(&num) {
                Some(c) => {
                    *c = *c + 1;
                    *c <= 3
                }

                None => {
                    cnt.insert(num, 1);
                    true
                }
            })
            .collect();

        let mut num2idx: HashMap<i32, usize> = HashMap::new();

        for (idx, &num) in nums.iter().enumerate() {
            num2idx.insert(num, idx);
        }

        let mut triplets: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                match num2idx.get_mut(&-(nums[i] + nums[j])) {
                    Some(k) => {
                        if *k != i && *k != j {
                            let mut triplet = vec![nums[i], nums[j], -(nums[i] + nums[j])];

                            triplet.sort();

                            triplets.insert(triplet);
                        }
                    }

                    None => {}
                }
            }
        }

        triplets.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0015() {
        let ret = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);

        assert!(ret.contains(&vec![-1, -1, 2]));
        assert!(ret.contains(&vec![-1, 0, 1]));
    }
}
