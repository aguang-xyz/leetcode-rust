pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let map: HashMap<i32, i32> = intervals.iter().fold(HashMap::new(), |mut map, v| {
            *map.entry(v[0]).or_insert(0) += 1;
            *map.entry(v[1]).or_insert(0) -= 1;
            map
        });

        let mut points: Vec<i32> = map.keys().cloned().collect();

        points.sort();

        let mut ans: Vec<Vec<i32>> = vec![];

        let mut count: i32 = 0;
        let mut left: i32 = 0;

        for pos in points.iter().cloned() {
            let delta: i32 = *map.get(&pos).unwrap();
            count += delta;

            if delta == 0 && count == 0 {
                ans.push(vec![pos, pos]);
            }

            // 0 -> delta, left side.
            if delta > 0 && count == delta {
                left = pos;
            }

            // delta -> 0, right side.
            if delta < 0 && count == 0 {
                ans.push(vec![left, pos]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0056() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]]
        );

        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![1, 5]]),
            vec![vec![1, 5]]
        );

        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );

        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![0, 0]]),
            vec![vec![0, 0], vec![1, 4]]
        );
    }
}
