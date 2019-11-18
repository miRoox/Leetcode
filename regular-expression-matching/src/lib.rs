pub struct Solution { }

//---
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Solution::is_match_impl(&s[..],&p[..])
    }

    fn is_match_impl(s: &str, p: &str) -> bool {
        if !s.is_empty() && !p.is_empty() {
            if Solution::str_eq_first(&p[1..],"*") {
                let mut i = 0;
                while i<=s.len() && Solution::match_first(&s[i..],&p[0..]) {
                    if Solution::is_match_impl(&s[i..],&p[2..]) {
                        return true
                    }
                    i += 1;
                }
                Solution::is_match_impl(&s[i..],&p[2..])
            } else {
                Solution::match_first(&s[0..],&p[0..]) && Solution::is_match_impl(&s[1..],&p[1..])
            }
        } else {
            s.is_empty() && p.is_empty()
        }
    }

    fn match_first(s: &str, p: &str) -> bool {
        Solution::str_eq_first(p,".") || Solution::str_eq_first(s,p)
    }

    // we need this since &str is not null-terminated string
    fn str_eq_first(s: &str, p: &str) -> bool {
        if !s.is_empty() && !p.is_empty() {
            &s[0..=0]==&p[0..=0]
        } else {
            s.is_empty() && p.is_empty()
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_null_slice() {
        let s = "aa";
        assert!(s[2..].is_empty());
    }

    #[test]
    fn mismatch_single() {
        let s = String::from("aa");
        let p = String::from("a");
        assert!(!Solution::is_match(s,p));
    }

    #[test]
    fn match_repeat() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert!(Solution::is_match(s,p));
    }

    #[test]
    fn match_any_repeat() {
        let s = String::from("ab");
        let p = String::from(".*");
        assert!(Solution::is_match(s,p));
    }

    #[test]
    fn match_repeat_null() {
        let s = String::from("aab");
        let p = String::from("c*a*b");
        assert!(Solution::is_match(s,p));
    }

    #[test]
    fn mismatch_any_repeat() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*p*.");
        assert!(!Solution::is_match(s,p));
    }
}
