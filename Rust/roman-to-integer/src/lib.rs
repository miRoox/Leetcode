pub struct Solution {}

//---
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut last = 0;
        for c in s.chars() {
            let current = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => unreachable!(),
            };
            result += if current > last {
                current - 2 * last
            } else {
                current
            };
            last = current;
        }
        result
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_3() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn for_4() {
        assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn for_9() {
        assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn for_58() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn for_1994() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
