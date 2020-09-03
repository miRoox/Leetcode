#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>
using std::vector;

#include <algorithm>
class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        std::next_permutation(nums.begin(), nums.end());
    }
};


TEST_CASE("next permutation") {
    Solution sol;
    SUBCASE("basic") {
        vector<int> nums{1,2,3};
        sol.nextPermutation(nums);
        CHECK(nums == vector<int>{1,3,2});
    }
    SUBCASE("last") {
        vector<int> nums{3,2,1};
        sol.nextPermutation(nums);
        CHECK(nums == vector<int>{1,2,3});
    }
    SUBCASE("duplicate") {
        vector<int> nums{1,1,5};
        sol.nextPermutation(nums);
        CHECK(nums == vector<int>{1,5,1});
    }
}

