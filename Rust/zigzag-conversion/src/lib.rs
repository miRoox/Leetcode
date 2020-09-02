pub struct Solution {}

//---
impl Solution {
    /// each line is indexed by a fixed step length of `2*num_rows-2`
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let s: Vec<char> = s.chars().collect();
        let len = s.len();
        let step = 2 * num_rows - 2;
        let mut result = String::with_capacity(len);
        for row in 0..num_rows {
            if row == 0 || row == num_rows - 1 {
                for i in (row..len).step_by(step) {
                    result.push(s[i]);
                }
            } else {
                for (l, r) in (row..len).step_by(step).zip((step - row..).step_by(step)) {
                    result.push(s[l]);
                    if r < len {
                        // note: r may be out of range when l is still in bound
                        result.push(s[r]);
                    }
                }
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
    fn row3() {
        let s = String::from("LEETCODEISHIRING");
        let s_out = String::from("LCIRETOESIIGEDHN");
        assert_eq!(Solution::convert(s, 3), s_out);
    }

    #[test]
    fn row4() {
        let s = String::from("LEETCODEISHIRING");
        let s_out = String::from("LDREOEIIECIHNTSG");
        assert_eq!(Solution::convert(s, 4), s_out);
    }

    #[test]
    fn row1() {
        let s = String::from("LEETCODEISHIRING");
        let s_out = String::from("LEETCODEISHIRING");
        assert_eq!(Solution::convert(s, 1), s_out);
    }
}
