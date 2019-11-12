pub struct Solution { }

//---
impl Solution {
    fn center_spread(s: &[u8], left: usize, right: usize) -> (usize, usize) {
        let mut lp = left;
        let mut rp = right;
        while s[lp]==s[rp] {
            if lp==0 || rp>=s.len()-1 {
                return (lp,rp);
            } else {
                lp -= 1;
                rp += 1;
            }
        }
        (lp+1, rp-1)
    }
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s
        }
        let s_chars: &[u8] = s.as_bytes(); // Assuming that all characters in the string s are within the range of ASCII code
        let (mut start, mut end) = (0, 0);
        let (mut current, mut offset) = (0, 0);
        while current+offset < s.len() {
            let (cstart, cend) = Solution::center_spread(s_chars,current,current+offset);
            if cstart < cend && cend-cstart > end-start {
                start = cstart;
                end = cend;
            }
            if offset==0 {
                offset = 1;
            } else {
                current += 1;
                offset = 0;
            }
        }
        s[start..end+1].to_string()
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic1() {
        let instr = String::from("babad");
        let result1 = String::from("bab");
        let result2 = String::from("aba");
        let actual = Solution::longest_palindrome(instr);
        assert!(actual==result1 || actual==result2,"The actual result is: {}", &actual);
    }

    #[test]
    fn basic2() {
        let instr = String::from("cbbd");
        let result = String::from("bb");
        assert_eq!(Solution::longest_palindrome(instr), result);
    }
}
