pub struct Solution {}

// ---
use std::fmt::{self, Debug};
use std::ops::{Index, IndexMut};
struct Matrix<T> {
    col: usize,
    data: Vec<T>,
}
impl<T: Default> Matrix<T> {
    pub fn new(col: usize, row: usize) -> Matrix<T> {
        let size = row * col;
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, T::default);
        Matrix { col, data }
    }
}
impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + self.col * index.1]
    }
}
impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + self.col * index.1]
    }
}
impl<T: Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.data.chunks_exact(self.col))
            .finish()
    }
}
// 1. if we need k steps from word1[0..i-1] to word2[0..j-1],
//   we can
//     use k steps from word1[0..i] to word2[0..j], if word1[i] == word2[j]
//     or k+1 steps from word1[0..i] to word2[0..j]
// 2. if we need k steps from word1[0..i-1] to word2[0..j],
//   we can use k steps from word1[0..i-1] to word2[0..j], than remove word1[i],
//   so we need k+1 steps from word1[0..i] to word2[0..j]
// 3. if we need k steps from word1[0..i] to word2[0..j-1]
//   we can use k steps from word1[0..i] to word2[0..j-1]，than append word2[j],
//   so we need k+1 steps from word1[0..i] to word2[0..j]
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n1 = word1.len() + 1;
        let n2 = word2.len() + 1;
        let mut steps = Matrix::new(n1, n2);
        for i in 0..n1 {
            steps[(i, 0)] = i;
        }
        for j in 0..n2 {
            steps[(0, j)] = j;
        }
        for i in 1..n1 {
            for j in 1..n2 {
                steps[(i, j)] = if word1[(i - 1)..i] == word2[(j - 1)..j] {
                    steps[(i - 1, j - 1)]
                } else {
                    1 + *[steps[(i - 1, j - 1)], steps[(i - 1, j)], steps[(i, j - 1)]]
                        .iter()
                        .min()
                        .unwrap()
                };
            }
        }
        steps[(n1 - 1, n2 - 1)] as i32
    }
}
// ---

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    fn demo2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
