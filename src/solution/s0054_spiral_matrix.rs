pub struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix: Vec<Vec<Option<i32>>> = matrix
            .iter()
            .cloned()
            .map(|row| {
                row.iter()
                    .cloned()
                    .map(|num| Some(num))
                    .collect::<Vec<Option<i32>>>()
            })
            .collect();

        let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut ans: Vec<i32> = vec![];

        if matrix.len() > 0 && matrix[0].len() > 0 {
            let mut row: i32 = 0;
            let mut col: i32 = 0;

            ans.push(matrix[row as usize][col as usize].take().unwrap());

            loop {
                let mut moved: bool = false;

                for (d_row, d_col) in directions.iter() {
                    while row + d_row >= 0
                        && row + d_row < matrix.len() as i32
                        && col + d_col >= 0
                        && col + d_col < matrix[row as usize].len() as i32
                        && matrix[(row + d_row) as usize][(col + d_col) as usize].is_some()
                    {
                        row += d_row;
                        col += d_col;

                        ans.push(matrix[row as usize][col as usize].take().unwrap());

                        moved = true;
                    }
                }

                if !moved {
                    break;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0054() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }
}
