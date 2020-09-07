#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

class Solution {
    vector<bool> cached;
    vector<bool> cache;

    inline bool canJumpFrom(const vector<int>& nums, size_t start) {
        if (start+1 >= nums.size()) {
            return true;
        }
        if (cached[start]) {
            return cache[start];
        }
        const int max = nums[start];
        if (max==0){
            cached[start] = true;
            return cache[start] = false;
        }
        bool found = false;
        for(int i=max; i>0 ; --i) {
            found |= canJumpFrom(nums, start + i);
            if (found)
                break;
        }
        cached[start] = true;
        return cache[start] = found;
    }
public:
    bool canJump(const vector<int>& nums) {
        cache = vector<bool>(nums.size());
        cached = vector<bool>(nums.size(), false);
        return canJumpFrom(nums, 0);
    }
};

TEST_CASE("jump game") {
    Solution sol;
    SUBCASE("reachable") {
        CHECK(sol.canJump({2,3,1,1,4}));
    }
    SUBCASE("unreachable") {
        CHECK(!sol.canJump({3,2,1,0,4}));
    }
}
