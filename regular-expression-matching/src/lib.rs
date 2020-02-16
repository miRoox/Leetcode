pub struct Solution { }

//---
use std::str::Chars;
impl Solution {

    pub fn is_match(s: String, p: String) -> bool {
        Solution::is_match_impl(s.chars(),p.chars())
    }

    fn is_match_impl(mut s: Chars, mut p: Chars) -> bool {
        //println!("{:?}\n{:?}",s.as_str(),p.as_str());
        if let Some(pc) = p.next() {
            let p1 = p.clone();
            let mut sco = s.clone().next();
            match p.next() {
                Some('*') => {
                    while Solution::match_single(sco, pc) {
                        if Solution::is_match_impl(s.clone(),p.clone()) {
                            return true
                        }
                        s.next();
                        sco=s.clone().next();
                    }
                    Solution::is_match_impl(s,p)
                },
                _=> if Solution::match_single(s.next(), pc) {
                    Solution::is_match_impl(s, p1)
                } else {
                    false
                }
            }
        } else {
            s.next()==None
        }
    }

    fn match_single(sco: Option<char>, pc: char) -> bool {
        //println!(">> {:?} : {:?}",sco,pc);
        if let Some(sc) = sco {
            sc==pc || pc=='.'
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
    fn match_any_repeat2() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*ip*.");
        assert!(Solution::is_match(s,p));
    }

    #[test]
    fn mismatch_other_repeat() {
        let s = String::from("aaa");
        let p = String::from("ab*a");
        assert!(!Solution::is_match(s,p));
    }

    #[test]
    fn mismatch_any_repeat() {
        let s = String::from("mississippi");
        let p = String::from("mis*is*p*.");
        assert!(!Solution::is_match(s,p));
    }

    #[test]
    fn mismatch_any_repeat2() {
        let s = String::from("ab");
        let p = String::from(".*c");
        assert!(!Solution::is_match(s,p));
    }
}
