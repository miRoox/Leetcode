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
