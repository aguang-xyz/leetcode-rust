pub struct Solution {}

impl Solution {
    pub fn line_justify(words: Vec<String>, max_width: usize, is_last: bool) -> String {
        if words.len() == 0 {
            panic!("std wrong..");
        }

        if words.len() == 1 {
            format!(
                "{}{}",
                words[0],
                (0..(max_width - words[0].len()))
                    .map(|_| ' ')
                    .collect::<String>()
            )
        } else {
            let char_cnt: usize = words.iter().map(|word| word.len()).sum::<usize>();

            if is_last {
                return format!(
                    "{}{}",
                    words.join(" "),
                    (0..(max_width - char_cnt + 1 - words.len()))
                        .map(|_| ' ')
                        .collect::<String>()
                );
            }

            let a: usize = (max_width - char_cnt) / (words.len() - 1);
            let b: usize = (max_width - char_cnt) % (words.len() - 1);

            words
                .iter()
                .enumerate()
                .map(|(idx, word)| {
                    if idx == 0 {
                        word.clone()
                    } else if idx <= b {
                        format!(" {}{}", (0..a).map(|_| ' ').collect::<String>(), word)
                    } else {
                        format!("{}{}", (0..a).map(|_| ' ').collect::<String>(), word)
                    }
                })
                .collect()
        }
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let max_width = max_width as usize;

        let lines: Vec<Vec<String>> =
            words
                .iter()
                .fold(Vec::<Vec<String>>::new(), |mut lines, word| {
                    let len = lines.len();

                    if len > 0
                        && lines[len - 1]
                            .iter()
                            .map(|word| word.len() + 1)
                            .sum::<usize>()
                            + word.len()
                            <= max_width
                    {
                        lines[len - 1].push(word.clone());
                    } else {
                        lines.push(vec![word.clone()]);
                    }

                    lines
                });

        let lines_cnt = lines.len();

        lines
            .iter()
            .enumerate()
            .map(|(idx, line)| Self::line_justify(line.clone(), max_width, idx == lines_cnt - 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0068() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This",
                    "is",
                    "an",
                    "example",
                    "of",
                    "text",
                    "justification."
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                16
            ),
            vec!["This    is    an", "example  of text", "justification.  "]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(
            Solution::full_justify(
                vec!["What", "must", "be", "acknowledgment", "shall", "be"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                16
            ),
            vec!["What   must   be", "acknowledgment  ", "shall be        "]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        );

        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science",
                    "is",
                    "what",
                    "we",
                    "understand",
                    "well",
                    "enough",
                    "to",
                    "explain",
                    "to",
                    "a",
                    "computer.",
                    "Art",
                    "is",
                    "everything",
                    "else",
                    "we",
                    "do"
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                20
            ),
            vec![
                "Science  is  what we",
                "understand      well",
                "enough to explain to",
                "a  computer.  Art is",
                "everything  else  we",
                "do                  "
            ]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );
    }
}
