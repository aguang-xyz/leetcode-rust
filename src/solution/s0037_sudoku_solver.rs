pub struct Solution {}

use std::char;
use std::collections::HashSet;

impl Solution {
    pub fn choices(board: &mut Vec<Vec<char>>, row: usize, col: usize) -> HashSet<char> {
        let mut chs: HashSet<char> = (1..10).map(|i| char::from_digit(i, 10).unwrap()).collect();

        for i in 0..9 {
            let chr: char = board[row][i];

            if chr != '.' {
                chs.remove(&chr);
            }
        }

        for i in 0..9 {
            let chr: char = board[i][col];

            if chr != '.' {
                chs.remove(&chr);
            }
        }

        for i in 0..3 {
            for j in 0..3 {
                let chr: char = board[(row / 3) * 3 + i][(col / 3) * 3 + j];

                if chr != '.' {
                    chs.remove(&chr);
                }
            }
        }

        chs
    }

    pub fn search(board: &mut Vec<Vec<char>>) -> bool {
        let mut decisions: Vec<(usize, usize, HashSet<char>)> = vec![];

        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] == '.' {
                    decisions.push((row, col, Self::choices(board, row, col)));
                }
            }
        }

        if decisions.len() == 0 {
            return true;
        }

        decisions.sort_by(|a, b| a.2.len().cmp(&b.2.len()));

        let row: usize = decisions[0].0;
        let col: usize = decisions[0].1;

        let choices: HashSet<char> = decisions[0].2.clone();

        if choices.is_empty() {
            return false;
        }

        for choice in choices {
            board[row][col] = choice;

            if Self::search(board) {
                return true;
            } else {
                board[row][col] = '.';
            }
        }

        false
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::search(board);
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0037() {
        let mut board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        Solution::solve_sudoku(&mut board);

        assert_eq!(
            board,
            vec![
                vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
                vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
                vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
                vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
                vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
                vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
                vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
                vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
                vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
            ]
        );
    }
}
