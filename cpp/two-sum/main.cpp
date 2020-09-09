#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

class Solution {
public:
    vector<int> twoSum(const vector<int>& nums, int target) {
        const auto size = nums.size();
        for (int i = 0; i < size; ++i)
        {
            for (int j = i+1; j < size; ++j)
            {
                if (nums[i] + nums[j] == target)
                    return {i, j};
            }
        }
        return {};
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("basic") {
        CHECK(sol.twoSum(vector<int>{2, 7, 11, 15}, 9) == vector<int>{0, 1});
    }
    SUBCASE("last-two") {
        CHECK(sol.twoSum(vector<int>{3, 2, 4}, 6) == vector<int>{1, 2});
    }
}
