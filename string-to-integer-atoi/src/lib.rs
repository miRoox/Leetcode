pub struct Solution { }

//---
impl Solution {
    pub fn my_atoi(s: String) -> i32 { 
        // TODO: ParseIntError cannot be used in the current stable channel
        // this solution is quite tricky, found in https://github.com/rust-lang/rust/issues/22639
        let overflow = "1231123123123123".parse::<i32>().err().unwrap();
        let underflow = "-1231123123123123".parse::<i32>().err().unwrap();
        let res = s.trim()
                   .trim_end_matches(|c:char|{!c.is_numeric()})
                   .parse::<i32>();
        match res {
            Ok(n) => n,
            Err(ref e) if *e == overflow => std::i32::MAX,
            Err(ref e) if *e == underflow => std::i32::MIN,
            _ => 0,
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let s = String::from("42");
        assert_eq!(Solution::my_atoi(s), 42);
    }

    #[test]
    fn start_whitespace() {
        let s = String::from("   -42");
        assert_eq!(Solution::my_atoi(s), -42);
    }

    #[test]
    fn end_words() {
        let s = String::from("4193 with words");
        assert_eq!(Solution::my_atoi(s), 4193);
    }

    #[test]
    fn start_words() {
        let s = String::from("words and 987");
        assert_eq!(Solution::my_atoi(s), 0);
    }

    #[test]
    fn overflow() {
        let s = String::from("-91283472332");
        assert_eq!(Solution::my_atoi(s), std::i32::MIN);
    }
}
