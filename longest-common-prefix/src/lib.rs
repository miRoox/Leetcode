pub struct Solution { }

//---
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut iter = strs.iter();
        if let Some(init) = iter.next() {
            let mut prefix = init.clone();
            for s in iter {
                let mut s = s.chars();
                prefix = prefix.chars().take_while(|c| Some(*c)==s.next()).collect();
                if prefix.is_empty() {
                    break
                }
            }
            prefix
        } else {
            String::new()
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = ["flower","flow","flight"].iter().map(|s| s.to_string()).collect();
        let output = "fl".to_string();
        assert_eq!(Solution::longest_common_prefix(input), output);
    }

    #[test]
    fn no_common() {
        let input = ["dog","racecar","car"].iter().map(|s| s.to_string()).collect();
        let output = "".to_string();
        assert_eq!(Solution::longest_common_prefix(input), output);
    }
}
