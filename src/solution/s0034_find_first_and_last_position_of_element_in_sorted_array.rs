pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let map_r: HashMap<i32, i32> =
            nums.iter()
                .cloned()
                .enumerate()
                .fold(HashMap::new(), |mut map, (idx, num)| {
                    map.insert(num, idx as i32);
                    map
                });

        let map_l: HashMap<i32, i32> =
            nums.iter()
                .cloned()
                .enumerate()
                .fold(HashMap::new(), |mut map, (idx, num)| {
                    map.entry(num).or_insert(idx as i32);
                    map
                });

        vec![
            *map_l.get(&target).unwrap_or(&-1),
            *map_r.get(&target).unwrap_or(&-1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0034() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );

        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }
}
