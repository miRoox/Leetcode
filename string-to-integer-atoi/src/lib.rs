pub struct Solution {}

//---
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result = 0i32;
        let mut iter = s.chars().skip_while(|c| c.is_whitespace()).peekable();
        let sign = match iter.peek() {
            Some('-') => {
                iter.next();
                -1
            }
            Some('+') => {
                iter.next();
                1
            }
            Some(c) if c.is_ascii_digit() => 1,
            _ => return 0,
        };
        for c in iter {
            if c.is_ascii_digit() {
                let d = c.to_digit(10).unwrap() as i32;
                result = result.saturating_mul(10).saturating_add(sign * d);
            } else {
                break;
            }
        }
        result
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
        let s = String::from("91283472332");
        assert_eq!(Solution::my_atoi(s), std::i32::MAX);
    }

    #[test]
    fn underflow() {
        let s = String::from("-91283472332");
        assert_eq!(Solution::my_atoi(s), std::i32::MIN);
    }

    #[test]
    fn digit_dot() {
        let s = String::from("3.14159");
        assert_eq!(Solution::my_atoi(s), 3);
    }

    #[test]
    fn inner_words() {
        let s = String::from("4193 with words then 123");
        assert_eq!(Solution::my_atoi(s), 4193);
    }
}
