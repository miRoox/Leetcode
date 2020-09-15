#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <string> 

using std::string;

#include <utility>
#include <vector>
#include <algorithm>
using std::size_t;
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
class Solution {
    template<typename T>
    class Matrix {
        size_t col;
        std::vector<T> data;
    public:
        Matrix(size_t col, size_t row): col(col), data(col*row, 0) {}
        inline T& operator()(size_t i, size_t j) {
            return data[i + j * col];
        }
        inline const T& operator()(size_t i, size_t j) const {
            return data[i + j * col];
        }
    };
public:
    int minDistance(const string& word1, const string& word2) {
        const size_t n1 = word1.size() + 1;
        const size_t n2 = word2.size() + 1;
        Matrix<size_t> steps(n1, n2);
        for (size_t i=0; i<n1; ++i) {
            steps(i, 0) = i;
        }
        for (size_t j=0; j<n2; ++j) {
            steps(0, j) = j;
        }
        for (size_t i=1; i<n1; ++i) {
            for (size_t j=1; j<n2; ++j) {
                if (word1[i-1] == word2[j-1]) {
                    steps(i, j) = steps(i-1, j-1);
                } else {
                    steps(i, j) = 1 + std::min({steps(i-1, j-1), steps(i-1, j), steps(i, j-1)});
                }
            }
        }
        return static_cast<int>(steps(n1-1, n2-1));
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("demo1") {
        CHECK(sol.minDistance("horse", "ros") == 3);
    }
    SUBCASE("demo2") {
        CHECK(sol.minDistance("intention", "execution") == 5);
    }
}
