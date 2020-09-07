#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <string>

using std::string;

class Solution {
    inline bool implMatch(const char* s, const char* p) {
        if (*p == '\0') return *s == '\0';
        if (p[1] == '*') {
            while (matchSingle(*s,*p)) {
                if (implMatch(s++, p+2)) {
                    return true;
                }
            }
            return implMatch(s, p+2);
        } else {
            return matchSingle(*s, *p) && implMatch(s+1, p+1);
        }
    }

    inline bool matchSingle(char c, char p) {
        return c==p || (p=='.' && c!='\0');
    }

public:
    bool isMatch(string s, string p) {
        return implMatch(s.c_str(), p.c_str());
    }
};

TEST_CASE("regular expression matching") {
    Solution sol;
    SUBCASE("mismatch single") {
        CHECK(!sol.isMatch("aa", "a"));
    }
    SUBCASE("match repeat") {
        CHECK(sol.isMatch("aa", "a*"));
    }
    SUBCASE("match any repeat") {
        CHECK(sol.isMatch("aa", ".*"));
    }
    SUBCASE("match repeat null") {
        CHECK(sol.isMatch("aab", "c*a*b"));
    }
    SUBCASE("match any repeat 2") {
        CHECK(sol.isMatch("mississippi", "mis*is*ip*."));
    }
    SUBCASE("mismatch other repeat") {
        CHECK(!sol.isMatch("aaa", "ab*a"));
    }
    SUBCASE("mismatch any repeat") {
        CHECK(!sol.isMatch("mississippi", "mis*is*p*."));
    }
    SUBCASE("mismatch any repeat 2") {
        CHECK(!sol.isMatch("ab", ".*c"));
    }
}