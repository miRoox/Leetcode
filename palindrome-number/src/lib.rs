pub struct Solution {}

//---
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut d = x;
        let mut r = 0i32;
        while d > 0 {
            if let Some(try_mul) = r.checked_mul(10) {
                if let Some(try_add) = try_mul.checked_add(d % 10) {
                    r = try_add; // r=r*10+d%10
                    d /= 10;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        x == r
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(121), true);
    }

    #[test]
    fn negative() {
        assert_eq!(Solution::is_palindrome(-121), false);
    }

    #[test]
    fn ends_zero() {
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn overflow() {
        assert_eq!(Solution::is_palindrome(1234567899), false);
    }
}
