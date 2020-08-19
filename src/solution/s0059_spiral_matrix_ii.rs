pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<Option<i32>>> = (0..n)
            .map(|_| (0..n).map(|_| None).collect::<Vec<Option<i32>>>())
            .collect();

        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut count: i32 = 1;

        if matrix.len() > 0 && matrix[0].len() > 0 {
            let mut row: i32 = 0;
            let mut col: i32 = 0;

            matrix[row as usize][col as usize].get_or_insert(count);
            count += 1;

            loop {
                let mut moved: bool = false;

                for (d_row, d_col) in directions.iter() {
                    while row + d_row >= 0
                        && row + d_row < matrix.len() as i32
                        && col + d_col >= 0
                        && col + d_col < matrix[row as usize].len() as i32
                        && matrix[(row + d_row) as usize][(col + d_col) as usize].is_none()
                    {
                        row += d_row;
                        col += d_col;

                        matrix[row as usize][col as usize].get_or_insert(count);
                        count += 1;

                        moved = true;
                    }
                }

                if !moved {
                    break;
                }
            }
        }

        matrix
            .iter()
            .map(|row| row.iter().map(|num| num.unwrap()).collect::<Vec<i32>>())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0059() {
        assert_eq!(
            Solution::generate_matrix(3),
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
        );
    }
}
