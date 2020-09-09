#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

#include <algorithm>
class Solution {
    static constexpr int TARGET = 0;
    vector<vector<int>> implForSorted(const vector<int>& nums) {
        vector<vector<int>> result;
        const auto start = nums.begin();
        const auto end = nums.end() - 1;
        for(auto i = start; i < end; ++i) {
            if (*i > TARGET)
                break;
            while (i > start && i < end && *i == *(i-1)) ++i;
            auto j = i + 1;
            auto k = end;
            while (j < k) {
                int v = (*i) + (*j) + (*k);
                if (v < TARGET) {
                    ++j;
                } else if (v > TARGET) {
                    --k;
                } else {
                    result.push_back({*i, *j, *k});
                    while (j < k && *j == *(j+1)) ++j;
                    while (j < k && *k == *(k-1)) --k;
                    ++j;
                    --k;
                }
            }
        }
        return result;
    }
public:
    vector<vector<int>> threeSum(vector<int>& nums) {
        if (nums.size()<3) {
            return {};
        }
        std::sort(nums.begin(), nums.end());
        return implForSorted(nums);
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("basic") {
        vector<int> nums{-1, 0, 1, 2, -1, -4};
        vector<vector<int>> result{
            {-1, -1, 2},
            {-1, 0, 1}
        };
        CHECK(sol.threeSum(nums) == result);
    }
    SUBCASE("duplicate") {
        vector<int> nums{-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6};
        vector<vector<int>> result{
            {-4, -2, 6},
            {-4, 0, 4},
            {-4, 1, 3},
            {-4, 2, 2},
            {-2, -2, 4},
            {-2, 0, 2}
        };
        CHECK(sol.threeSum(nums) == result);
    }
    SUBCASE("zeros") {
        vector<int> nums{0, 0, 0, 0};
        vector<vector<int>> result{
            {0, 0, 0}
        };
        CHECK(sol.threeSum(nums) == result);
    }
}
