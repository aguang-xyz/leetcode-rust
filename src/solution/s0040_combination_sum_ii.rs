pub struct Solution {}

use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let count: HashMap<i32, usize> =
            candidates
                .iter()
                .cloned()
                .fold(HashMap::new(), |mut count, candidate| {
                    *count.entry(candidate).or_insert(0) += 1;
                    count
                });

        let candidates: Vec<i32> = count.keys().cloned().collect();

        // f[i][target]
        let mut f: Vec<Vec<Vec<Vec<i32>>>> =
            vec![vec![vec![]; (target + 1) as usize]; candidates.len() + 1];

        f[0][0].push(vec![]);

        for i in 1..(candidates.len() + 1) as usize {
            for j in 0..(target + 1) as usize {
                f[i][j] = vec![];

                let candidate: i32 = candidates[i - 1];

                for k in 0..(min(
                    *count.get(&candidate).unwrap() as i32,
                    (j as i32) / candidate,
                ) + 1)
                {
                    let sets: Vec<Vec<i32>> = f[i - 1][j - (candidate * k) as usize].clone();

                    for set in sets {
                        let mut new_set: Vec<i32> = set.clone();

                        for _p in 0..k {
                            new_set.push(candidate as i32);
                        }

                        new_set.sort();

                        f[i][j].push(new_set);
                    }
                }

                println!("{},{} => {:?}", i, j, f[i][j]);
            }
        }

        f[candidates.len()][target as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0039() {
        let ret = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);

        assert!(ret.contains(&vec![1, 7]));
        assert!(ret.contains(&vec![1, 2, 5]));
        assert!(ret.contains(&vec![2, 6]));
        assert!(ret.contains(&vec![1, 1, 6]));

        let ret = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);

        assert!(ret.contains(&vec![1, 2, 2]));
        assert!(ret.contains(&vec![5]));
    }
}
