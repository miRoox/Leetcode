#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

#include <cstdlib>
#include <algorithm>
#include <vector>
using std::size_t;
using std::swap;
class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
        const size_t n = matrix.size();
        // flip
        for (size_t i = 0; i < n/2; ++i) {
            swap(matrix[i],matrix[n-i-1]);
        }
        // then transpose
        for (size_t i = 0; i < n; ++i) {
            for (size_t j = i; j < n; ++j) {
                swap(matrix[i][j], matrix[j][i]);
            }
        }
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("rank3") {
        vector<vector<int>> matrix{
            {1,2,3},
            {4,5,6},
            {7,8,9}
        };
        vector<vector<int>> result{
            {7,4,1},
            {8,5,2},
            {9,6,3}
        };
        sol.rotate(matrix);
        CHECK(matrix == result);
    }
    SUBCASE("rank4") {
        vector<vector<int>> matrix{
            { 5, 1, 9,11},
            { 2, 4, 8,10},
            {13, 3, 6, 7},
            {15,14,12,16}
        };
        vector<vector<int>> result{
            {15,13, 2, 5},
            {14, 3, 4, 1},
            {12, 6, 8, 9},
            {16, 7,10,11}
        };
        sol.rotate(matrix);
        CHECK(matrix == result);
    }
}
