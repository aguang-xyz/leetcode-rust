pub struct Solution {}

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m: usize = obstacle_grid.len();
        let n: usize = obstacle_grid[0].len();

        let mut f: Vec<Vec<i32>> = vec![vec![0; n]; m];

        if obstacle_grid[0][0] == 0 {
            f[0][0] = 1;
        }

        for i in 0..m {
            for j in 0..n {
                if obstacle_grid[i][j] == 0 {
                    if i > 0 {
                        f[i][j] += f[i - 1][j];
                    }

                    if j > 0 {
                        f[i][j] += f[i][j - 1];
                    }
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
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );

        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![1],]), 0);
    }
}
