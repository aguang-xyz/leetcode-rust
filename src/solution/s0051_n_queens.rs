pub struct Solution {}

use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let n = n as usize;

        let mut ans: Vec<Vec<String>> = vec![];

        let mut que: VecDeque<Vec<usize>> = VecDeque::new();

        que.push_back(vec![]);

        while !que.is_empty() {
            let top: Vec<usize> = que.pop_front().unwrap();

            if top.len() == n {
                ans.push(
                    top.iter()
                        .cloned()
                        .map(|pos| {
                            (0..n)
                                .map(|col| if col == pos { 'Q' } else { '.' })
                                .collect::<String>()
                        })
                        .collect::<Vec<String>>(),
                );
            } else {
                let choices: Vec<usize> = (0..n)
                    .filter(|pos| {
                        if top.contains(pos) {
                            return false;
                        }

                        for (row, &col) in top.iter().enumerate() {
                            if top.len() - row == max(col, *pos) - min(col, *pos) {
                                return false;
                            }
                        }

                        true
                    })
                    .collect();

                for choice in choices.iter().cloned() {
                    let mut new: Vec<usize> = top.clone();

                    new.push(choice);

                    que.push_back(new);
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
    fn test_0051() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    String::from(".Q.."),
                    String::from("...Q"),
                    String::from("Q..."),
                    String::from("..Q.")
                ],
                vec![
                    String::from("..Q."),
                    String::from("Q..."),
                    String::from("...Q"),
                    String::from(".Q..")
                ]
            ]
        );
    }
}
