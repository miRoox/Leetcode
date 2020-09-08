#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>

using std::vector;

#include <algorithm>
#include <set>
#include <queue>
#include <cstdlib>
#include <utility>
#include <iterator>
using std::multiset;
using std::priority_queue;
using std::size_t;
using std::pair;
class Solution {
public:
    vector<int> topKFrequent(const vector<int>& nums, int k) {
        multiset<int> grouped(nums.begin(), nums.end());
        using Ty = pair<size_t, int>;
        priority_queue<Ty, vector<Ty>, std::greater<Ty> > heap;
        auto it = grouped.begin();
        while (it != grouped.cend()) {
            size_t c = grouped.count(*it);
            heap.emplace(c, *it);
            if (heap.size() > k) heap.pop();
            std::advance(it, c);
        }
        vector<int> result;
        result.reserve(k);
        while(!heap.empty()) {
            result.emplace_back(heap.top().second);
            heap.pop();
        }
        return result;
    }
};

TEST_CASE("top k frequent elements") {
    using std::set;
    Solution sol;
    SUBCASE("top 2") {
        set<int> expected{1, 2};
        auto result = sol.topKFrequent({1,1,1,2,2,3}, 2);
        CHECK(set<int>(result.begin(), result.end()) == expected);
    }
    SUBCASE("top 1") {
        set<int> expected{1};
        auto result = sol.topKFrequent({1}, 1);
        CHECK(set<int>(result.begin(), result.end()) == expected);
    }
}
