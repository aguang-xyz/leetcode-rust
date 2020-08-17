pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // By rows.
        for row in 0..9 {
            let items: Vec<char> = (0..9)
                .map(|col| board[row][col])
                .filter(|&c| c != '.')
                .collect();

            let uniq_items: HashSet<char> = items.iter().cloned().collect();

            if items.len() != uniq_items.len() {
                return false;
            }
        }

        // By columns.
        for col in 0..9 {
            let items: Vec<char> = (0..9)
                .map(|row| board[row][col])
                .filter(|&c| c != '.')
                .collect();

            let uniq_items: HashSet<char> = items.iter().cloned().collect();

            if items.len() != uniq_items.len() {
                return false;
            }
        }

        // By blocks.
        for i in 0..3 {
            for j in 0..3 {
                let items: Vec<char> = ((i * 3)..(i * 3 + 3))
                    .flat_map(move |row| ((j * 3)..(j * 3 + 3)).map(move |col| (row, col)))
                    .map(|(row, col)| board[row][col])
                    .filter(|&c| c != '.')
                    .collect();

                let uniq_items: HashSet<char> = items.iter().cloned().collect();

                if items.len() != uniq_items.len() {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0036() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            true
        );

        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            false
        );
    }
}
