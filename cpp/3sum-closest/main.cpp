#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

#include <algorithm>
#include <limits>
#include <cmath>
class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        std::sort(nums.begin(), nums.end());
        int result = 0;
        int min_diff = std::numeric_limits<int>::max();
        int last = static_cast<int>(nums.size()-1);
        for (int i=0; i<last; ++i) {
            int j = i+1;
            int k = last;
            while (j < k) {
                int sum = nums[i] + nums[j] + nums[k];
                if (sum < target) {
                    ++j;
                } else if (sum > target) {
                    --k;
                } else {
                    return target;
                }
                int diff = std::abs(sum-target);
                if (diff < min_diff) {
                    min_diff = diff;
                    result = sum;
                }
            }
        }
        return result;
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("basic") {
        vector<int> nums{-1,2,1,-4};
        CHECK(sol.threeSumClosest(nums, 1) == 2);
    }
}
