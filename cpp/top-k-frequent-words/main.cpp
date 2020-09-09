#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <vector>
#include <string>

using std::vector;
using std::string;

#include <utility>
#include <unordered_map>
#include <queue>
#include <deque>
using std::size_t;
using std::unordered_map;
using std::priority_queue;
using std::deque;
class Solution {
    unordered_map<string, size_t> count(const vector<string>& words) {
        unordered_map<string, size_t> result;
        for (const auto& w: words) {
            if (result.count(w) == 0) {
                result.insert_or_assign(w, 0);
            } else {
                ++result[w];
            }
        }
        return result;
    }
    vector<string> topKFromWordCount(const unordered_map<string, size_t>& wc, int k) {
        using Pss = std::pair<string, size_t>;
        // min heap
        auto gt = [](const Pss& left, const Pss& right) -> bool {
                return left.second == right.second 
                    ? left.first < right.first // note: heap will be reversed after
                    : left.second > right.second;
        };
        priority_queue heap{gt, deque<Pss>()};
        for (const auto& wcp : wc) {
            heap.push(wcp);
            if (heap.size() > k) {
                heap.pop();
            }
        }
        // reverse from heap
        vector<string> result(k);
        for(auto it=result.rbegin(); it!=result.rend(); ++it) {
            *it = heap.top().first;
            heap.pop();
        }
        return result;
    }
public:
    vector<string> topKFrequent(const vector<string>& words, int k) {
        return topKFromWordCount(count(words), k);
    }
};


TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("top2") {
        vector<string> words{"i", "love", "leetcode", "i", "love", "coding"};
        vector<string> result{"i", "love"};
        CHECK(sol.topKFrequent(words, 2) == result);
    }
    SUBCASE("all") {
        vector<string> words{"the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"};
        vector<string> result{"the", "is", "sunny", "day"};
        CHECK(sol.topKFrequent(words, 4) == result);
    }
}
