#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

#include <utility>
using std::size_t;
class Solution {
    inline size_t rfind_zero(const vector<int>& nums, size_t sentinel) {
        size_t i = sentinel;
        while (i--) {
            if (nums[i] == 0)
                break;
        }
        return i;
    }
    inline bool canJumpSpan(const vector<int>& nums, size_t sentinel) {
        if (sentinel == 0)
            return true;
        const size_t iz = rfind_zero(nums, sentinel);
        if (iz == static_cast<size_t>(-1)) 
            return true;
        for(size_t i = iz-1; i != static_cast<size_t>(-1); --i) {
            if (i+nums[i] > iz)
                return canJumpSpan(nums, i);
        }
        return false;
    }
public:
    bool canJump(const vector<int>& nums) {
        return canJumpSpan(nums, nums.size()-1);
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("without zero") {
        CHECK(sol.canJump({2,3,1,1,4}));
    }
    SUBCASE("unreachable with zero") {
        CHECK(!sol.canJump({3,2,1,0,4}));
    }
    SUBCASE("reachable with zero") {
        CHECK(sol.canJump({3,3,1,0,4}));
    }
    SUBCASE("reachable with double zeros") {
        CHECK(sol.canJump({3,0,2,0,1,4}));
    }
    SUBCASE("unreachable with double zeros") {
        CHECK(!sol.canJump({1,0,2,0,1,4}));
    }
}
