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

class Solution {
    ListNode* implAdd(const ListNode* l1, const ListNode* l2, int carry) {
        int v1 = getNext(l1);
        int v2 = getNext(l2);
        int v = v1 + v2 + carry;
        if (v==0 && l1 == nullptr && l2 == nullptr)
            return nullptr;
        auto res = new ListNode(v%10);
        res->next = implAdd(l1, l2, v/10);
        return res;
    }

    int getNext(const ListNode*& l) {
        if (l == nullptr) 
            return 0;
        int val = l->val;
        l = l->next;
        return val;
    }

public:
    ListNode* addTwoNumbers(const ListNode* l1, const ListNode* l2) {
        auto res = implAdd(l1, l2, 0);
        if (res == nullptr) 
            return new ListNode(0);
        return res;
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

    // assume no leading zeros
    static bool equiv(const ListNode* l1, const ListNode* l2) {
        if (l1 == l2)
            return true;
        if (l1 == nullptr || l2 == nullptr || l1->val != l2->val)
            return false;
        return equiv(l1->next, l2->next);
    }

public:
    List() noexcept : first(nullptr){ }

    List(ListNode* node) noexcept : first(node) { }

    List(const std::initializer_list<int>& list) {
        first = std::accumulate(std::rbegin(list), std::rend(list), (ListNode*)nullptr, 
            [](ListNode* next, int val){
                auto res = new ListNode(val);
                res->next = next;
                return res;
            });
    }

    ~List() {
        deleteNode(first);
    }

    const ListNode* node() const { return first; }

    bool operator == (const List& rhs) const & {
        return equiv(this->first, rhs.first);
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

TEST_CASE("add two numbers") {
    Solution sol;
    SUBCASE("basic") {
        List l1{2, 4, 3};
        List l2{5, 6, 4};
        CHECK(List(sol.addTwoNumbers(l1.node(), l2.node())) == List{7, 0, 8});
    }
    SUBCASE("zero") {
        List l1{0};
        List l2{0};
        CHECK(List(sol.addTwoNumbers(l1.node(), l2.node())) == List{0});
    }
    SUBCASE("zero in digits") {
        List l1{1, 6, 0, 3, 3, 6, 7, 2, 0, 1};
        List l2{6, 3, 0, 8, 9, 6, 6, 9, 6, 1};
        CHECK(List(sol.addTwoNumbers(l1.node(), l2.node())) == List{7, 9, 0, 1, 3, 3, 4, 2, 7, 2});
    }
}
