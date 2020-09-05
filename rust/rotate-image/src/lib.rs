pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n / 2 {
            matrix.swap(i, n - 1 - i);
        }
        let mut swap_nested = |(i1, j1), (i2, j2)| unsafe {
            let v1: &mut Vec<_> = matrix.get_unchecked_mut(i1);
            let p1: *mut _ = &mut v1[j1];
            let v2: &mut Vec<_> = matrix.get_unchecked_mut(i2);
            let p2: *mut _ = &mut v2[j2];
            std::ptr::swap(p1, p2);
        };
        for i in 0..n {
            for j in i..n {
                swap_nested((i, j), (j, i));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank3() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, result);
    }

    #[test]
    fn rank4() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let result = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, result);
    }
}
