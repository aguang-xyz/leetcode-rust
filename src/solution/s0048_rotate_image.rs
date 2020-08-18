pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let size: usize = matrix.len();

        for row in 0..(size / 2) {
            for col in 0..((size + 1) / 2) {
                let tmp = matrix[row][col];

                matrix[row][col] = matrix[size - col - 1][row];

                matrix[size - col - 1][row] = matrix[size - row - 1][size - col - 1];

                matrix[size - row - 1][size - col - 1] = matrix[col][size - row - 1];

                matrix[col][size - row - 1] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0048() {
        let mut m: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut m);
        assert_eq!(m, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

        let mut m: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        Solution::rotate(&mut m);
        assert_eq!(
            m,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
