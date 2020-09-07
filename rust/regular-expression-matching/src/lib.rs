pub struct Solution {}

//---
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_impl(s.as_bytes(), p.as_bytes())
    }

    #[inline]
    fn is_match_impl(mut s: &[u8], p: &[u8]) -> bool {
        if let Some(&pc) = p.first() {
            let mut sco = s.first();
            match p.get(1) {
                Some(b'*') => {
                    while Self::match_single(sco.copied(), pc) {
                        if Self::is_match_impl(s, &p[2..]) {
                            return true;
                        }
                        s = &s[1..];
                        sco = s.first();
                    }
                    Self::is_match_impl(s, &p[2..])
                }
                _ => {
                    Self::match_single(s.first().copied(), pc)
                        && Self::is_match_impl(&s[1..], &p[1..])
                }
            }
        } else {
            s.first() == None
        }
    }

    #[inline(always)]
    fn match_single(sco: Option<u8>, pc: u8) -> bool {
        if let Some(sc) = sco {
            sc == pc || pc == b'.'
        } else {
            false
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mismatch_single() {
        let s = String::from("aa");
        let p = String::from("a");
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn match_repeat() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn match_any_repeat() {
        let s = String::from("ab");
        let p = String::from(".*");
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn match_repeat_null() {
        let s = String::from("aab");
        let p = String::from("c*a*b");
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn match_any_repeat2() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*ip*.");
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn mismatch_other_repeat() {
        let s = String::from("aaa");
        let p = String::from("ab*a");
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn mismatch_any_repeat() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*p*.");
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn mismatch_any_repeat2() {
        let s = String::from("ab");
        let p = String::from(".*c");
        assert!(!Solution::is_match(s, p));
    }
}
