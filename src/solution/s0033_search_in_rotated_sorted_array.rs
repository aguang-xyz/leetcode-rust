pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let map: HashMap<i32, i32> =
            nums.iter()
                .cloned()
                .enumerate()
                .fold(HashMap::new(), |mut map, (idx, num)| {
                    map.insert(num, idx as i32);
                    map
                });

        *map.get(&target).unwrap_or(&-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0033() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
    }
}
