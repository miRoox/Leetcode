pub struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut counts = Vec::with_capacity(1 + num as usize);
        counts.push(0);
        for i in 1..=num {
            counts.push(if i % 2 == 0 {
                counts[i as usize >> 1]
            } else {
                counts[i as usize - 1] + 1
            });
        }
        counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
