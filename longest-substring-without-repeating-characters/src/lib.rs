pub struct Solution { }

//---
use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last = HashMap::new();
        let mut start : usize = 0;
        let mut longest : usize = 0;
        for (i,v) in s.char_indices() {
            if let Some(current) = last.get(&v) {
                if *current>=start {
                    longest = max(i-start,longest);
                    start = current+1;
                }
            }
            last.insert(v,i);
        }
        max(s.len()-start,longest) as i32
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let instr = String::from("abcabcbb");
        assert_eq!(Solution::length_of_longest_substring(instr), 3);
    }

    #[test]
    fn single() {
        let instr = String::from("bbbbb");
        assert_eq!(Solution::length_of_longest_substring(instr), 1);
    }

    #[test]
    fn subseq() {
        let instr = String::from("pwwkew");
        assert_eq!(Solution::length_of_longest_substring(instr), 3);
    }
}
