pub struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut s: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                s.entry(nums[i] + nums[j]).or_insert(vec![]).push((i, j));
            }
        }

        let mut quadruplets: HashSet<Vec<i32>> = HashSet::new();

        for x in s.keys() {
            match s.get(&(target - x)) {
                Some(vec1) => {
                    for a in vec1 {
                        for b in s.get(x).unwrap() {
                            if a.0 != b.0 && a.0 != b.1 && a.1 != b.0 && a.1 != b.1 {
                                let mut quadruplet: Vec<i32> =
                                    vec![nums[a.0], nums[a.1], nums[b.0], nums[b.1]];

                                quadruplet.sort();

                                quadruplets.insert(quadruplet);
                            }
                        }
                    }
                }

                None => {}
            }
        }

        quadruplets.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0018() {
        let ret = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);

        assert!(ret.contains(&vec![-1, 0, 0, 1]));
        assert!(ret.contains(&vec![-2, -1, 1, 2]));
        assert!(ret.contains(&vec![-2, 0, 0, 2]));
    }
}
