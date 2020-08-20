pub struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.len() == 0 {
            vec![]
        } else {
            let target: HashMap<String, usize> =
                words
                    .iter()
                    .cloned()
                    .fold(HashMap::new(), |mut target, word| {
                        *target.entry(word).or_insert(0) += 1;
                        target
                    });

            let target_len: usize = words.iter().map(|word| word.len()).sum();

            let uniq_words: HashSet<String> = words.iter().cloned().collect();

            let forward: Vec<HashSet<String>> = (0..s.len())
                .map(|i| {
                    uniq_words
                        .iter()
                        .cloned()
                        .filter(|word| s[i..].starts_with(word))
                        .collect()
                })
                .collect();

            let mut queue: VecDeque<(usize, HashMap<String, usize>)> = (0..s.len())
                .filter(|i| (i + target_len) <= s.len())
                .map(|i| (i, HashMap::new()))
                .collect();

            let mut ans: HashSet<usize> = HashSet::new();

            while !queue.is_empty() {
                let top = queue.pop_front().unwrap();

                let pos: usize = top.0;
                let vis: HashMap<String, usize> = top.1;

                let current_len: usize = vis.iter().map(|(word, cnt)| word.len() * cnt).sum();
                let start_pos: usize = pos - current_len;

                if current_len == target_len {
                    ans.insert(start_pos);
                } else {
                    if !ans.contains(&start_pos) && pos < forward.len() {
                        for word in forward[pos].iter() {
                            if vis.get(word).unwrap_or(&0) < target.get(word).unwrap() {
                                let new_pos: usize = pos + word.len();
                                let mut new_vis: HashMap<String, usize> = vis.clone();

                                *new_vis.entry(word.clone()).or_insert(0) += 1;

                                queue.push_back((new_pos, new_vis))
                            }
                        }
                    }
                }
            }

            ans.iter().cloned().map(|x| x as i32).collect::<Vec<i32>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0030() {
        let ret = Solution::find_substring(
            String::from("barfoothefoobarman"),
            vec![String::from("foo"), String::from("bar")],
        );

        assert!(ret.contains(&9));
        assert!(ret.contains(&0));

        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word"),
                ]
            ),
            Vec::<i32>::new()
        );

        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("good"),
                ]
            ),
            vec![8]
        );

        assert_eq!(
            Solution::find_substring(String::from("a"), vec![]),
            Vec::<i32>::new()
        );

        let ret = Solution::find_substring(
            String::from("aaaaaaaa"),
            vec![String::from("aa"), String::from("aa"), String::from("aa")],
        );

        assert!(ret.contains(&0));
        assert!(ret.contains(&1));
        assert!(ret.contains(&2));

        Solution::find_substring(
            (0..5000).map(|_i| 'a').collect::<String>(),
            vec!["a".to_string(); 5001],
        );
    }
}
