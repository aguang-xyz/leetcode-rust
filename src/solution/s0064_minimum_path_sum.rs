pub struct Solution {}

use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        let mut f: Vec<Vec<i32>> = vec![vec![i32::max_value(); n]; m];

        f[0][0] = grid[0][0];

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    f[i][j] = min(f[i][j], f[i - 1][j] + grid[i][j]);
                }

                if j > 0 {
                    f[i][j] = min(f[i][j], f[i][j - 1] + grid[i][j]);
                }
            }
        }

        f[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0063() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
    }
}
