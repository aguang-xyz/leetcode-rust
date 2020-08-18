pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // f[i][target]
        let mut f: Vec<Vec<Vec<Vec<i32>>>> =
            vec![vec![vec![]; (target + 1) as usize]; candidates.len() + 1];

        f[0][0].push(vec![]);

        for i in 1..(candidates.len() + 1) as usize {
            for j in 0..(target + 1) as usize {
                f[i][j] = vec![];

                let candidate: i32 = candidates[i - 1];

                for k in 0..((j as i32) / candidate + 1) {
                    let sets: Vec<Vec<i32>> = f[i - 1][j - (candidate * k) as usize].clone();

                    for set in sets {
                        let mut new_set: Vec<i32> = set.clone();

                        for _p in 0..k {
                            new_set.push(candidate as i32);
                        }

                        f[i][j].push(new_set);
                    }
                }
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
        let ret = Solution::combination_sum(vec![2, 3, 6, 7], 7);

        assert!(ret.contains(&vec![7]));
        assert!(ret.contains(&vec![2, 2, 3]));
    }
}
