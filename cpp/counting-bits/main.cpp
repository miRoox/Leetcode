#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

class Solution {
public:
    vector<int> countBits(int num) {
        vector<int> result{0};
        result.reserve(num);
        for (size_t i = 1; i <= num; ++i) {
            result.push_back(i % 2 == 0 ? result[i >> 1] : result[i - 1] + 1);
        }
        return result;
    }
};

TEST_CASE("Solution") {
    Solution sol;
    CHECK(sol.countBits(5) == vector<int>{0, 1, 1, 2, 1, 2});
}
