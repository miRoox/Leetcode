#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"

class Solution {
public:
    bool isPowerOfTwo(int n) {
        // see https://www.bilibili.com/video/BV1rz4y1Q7z8
        return (n>0) && ((n&-n) == n);
    }
};

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("one") {
        CHECK(sol.isPowerOfTwo(1));
    }
    SUBCASE("even") {
        CHECK(!sol.isPowerOfTwo(218));
    }
    SUBCASE("negative") {
        CHECK(!sol.isPowerOfTwo(-8));
    }
}
