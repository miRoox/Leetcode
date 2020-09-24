#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <initializer_list>
#include <iterator>
#include <numeric>

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(nullptr) {}
};

#include <utility>
#include <limits>
using std::size_t;
using std::numeric_limits;
using std::move;
struct ListRange {
    ListNode* head = nullptr;
    ListNode** ptail = nullptr;

    inline ListRange() = default;
    inline ListRange(std::nullptr_t) { }
    inline ListRange(ListNode* /*not null*/ init)
        : head(init) { 
            for(;init->next; init = init->next);
            ptail = &init->next;
        }
    inline bool nonempty() const { return *this && head != *ptail; }
    inline operator bool() const { return head && ptail; }
    inline ListRange& append(ListNode* next) {
        if (head) {
            *ptail = next; // append
        } else { // init
            head = next;
        }
        ptail = &next->next;
        return *this;
    }
    inline ListRange& prepend(ListNode* prev) {
        if (head) {
            prev->next = head;
        }
        else {
            ptail = &prev->next;
        }
        head = prev;
        return *this;
    }
};

class Solution {
    inline ListRange sortImpl(ListRange&& range) {
        if (!range)
            return range;
        ListRange left;
        ListRange right;
        ListNode* prior = range.head;
        const auto x = prior->val;
        range.head = prior->next;
        for (; range.nonempty(); range.head = range.head->next) {
            if (range.head->val < x) {
                left.append(range.head);
            } else {
                right.append(range.head);
            }
        }
        left = sortImpl(move(left));
        right = sortImpl(move(right));
        return join(move(left), move(right.prepend(prior)));
    }
    inline ListRange join(ListRange&& l1, ListRange&& l2) {
        if (!l1)
            return l2;
        *l1.ptail = l2.head;
        *l2.ptail = nullptr;
        l1.ptail = l2.ptail;
        return l1;
    }
public:
    // quick sort
    ListNode* sortList(ListNode* head) {
        if (!head)
            return head;
        ListRange range = sortImpl(ListRange(head));
        return range.head;
    }
};

class List {
    ListNode* first;

    static void deleteNode(ListNode* node) noexcept {
        if (node != nullptr) {
            deleteNode(node->next);
            delete node;
        }
    }

    static bool equiv(const ListNode* l1, const ListNode* l2) {
        if (l1 == l2)
            return true;
        else if (l1 == nullptr || l2 == nullptr || l1->val != l2->val)
            return false;
        else 
            return equiv(l1->next, l2->next);
    }

    static ListNode* construct(const std::initializer_list<int>& list) {
        return std::accumulate(std::rbegin(list), std::rend(list), (ListNode*)nullptr, 
            [](ListNode* next, int val){
                auto res = new ListNode(val);
                res->next = next;
                return res;
            });
    }

public:
    List() noexcept : first(nullptr){ }

    List(ListNode* node) noexcept : first(node) { }

    List(const std::initializer_list<int>& list) : first(construct(list)) { }

    List(const List&) = delete;
    List(List&& other) noexcept { 
        std::swap(first, other.first); 
    }

    ~List() {
        deleteNode(first);
    }

    const ListNode* node() const noexcept { return first; }

    ListNode* take_node() noexcept {
        ListNode* node = nullptr;
        std::swap(first, node);
        return node;
    }

    bool operator == (const List& rhs) const & {
        return equiv(this->first, rhs.first);
    }

    List& operator =(const List& other) = delete;
    List& operator =(List&& other) noexcept {
        std::swap(this->first, other.first);
        return *this;
    }
};

TEST_CASE("List") {
    SUBCASE("empty") {
        CHECK(List{} == List(nullptr));
    }
    SUBCASE("one") {
        CHECK(List{1} == List(new ListNode(1)));
    }
    SUBCASE("two") {
        auto node = new ListNode(1);
        node->next = new ListNode(2);
        CHECK(List{1, 2} == List(node));
    }
}

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("demo1") {
        List list{4, 2, 1, 3};
        List result{1, 2, 3, 4};
        CHECK(List(sol.sortList(list.take_node())) == result);
    }
    SUBCASE("demo2") {
        List list{-1, 5, 3, 4, 0};
        List result{-1, 0, 3, 4, 5};
        CHECK(List(sol.sortList(list.take_node())) == result);
    }
}
