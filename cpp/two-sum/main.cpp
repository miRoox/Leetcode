#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        return {0, 1};
    }
};

TEST_CASE("two sum") {
    Solution sol;
    SUBCASE("basic") {
        vector<int> nums {2, 7, 11, 15};
        vector<int> pair {0, 1};
        CHECK(sol.twoSum(nums, 9) == pair);
    }
}
