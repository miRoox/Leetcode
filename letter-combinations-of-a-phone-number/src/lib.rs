pub struct Solution {}

//---
const KEY_MAP: [&str; 8] = ["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result = Vec::with_capacity(3usize.pow(digits.len() as u32));
        Self::search(&mut result, digits.bytes(), String::new());
        result
    }

    fn search<Iter>(result: &mut Vec<String>, mut iter: Iter, current: String)
    where
        Iter: Iterator<Item = u8> + Clone,
    {
        if let Some(key) = iter.next() {
            for c in KEY_MAP[(key - b'2') as usize].chars() {
                let mut next = current.clone();
                next.push(c);
                Self::search(result, iter.clone(), next)
            }
        } else if !current.is_empty() {
            result.push(current);
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let digits = "23".to_string();
        let result: Vec<String> = ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
            .iter()
            .cloned()
            .map(String::from)
            .collect();
        assert_eq!(Solution::letter_combinations(digits), result);
    }

    #[test]
    fn empty_input() {
        let digits = String::new();
        let result: Vec<String> = vec![];
        assert_eq!(Solution::letter_combinations(digits), result);
    }
}
