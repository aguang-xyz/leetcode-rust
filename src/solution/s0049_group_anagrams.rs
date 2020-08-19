pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs.iter().cloned() {
            let mut key: Vec<char> = s.chars().collect();
            key.sort();

            let key: String = key.iter().collect();

            map.entry(key).or_insert(vec![]).push(s);
        }

        map.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_0049() {
        let ret = Solution::group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ]);

        assert!(ret.contains(&vec![String::from("bat")]));
        assert!(ret.contains(&vec![String::from("tan"), String::from("nat")]));
        assert!(ret.contains(&vec![
            String::from("eat"),
            String::from("tea"),
            String::from("ate")
        ]));
    }
}
