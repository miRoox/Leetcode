#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include "doctest.h"
#include <initializer_list>
#include <iterator>
#include <numeric>

struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
public:
    // note: no allocation
    ListNode* mergeTwoLists(ListNode* l1, ListNode* l2) {
        if (!l1) 
            return l2;
        else if (!l2)
            return l1;
        else if (l1->val < l2->val) {
            l1->next = mergeTwoLists(l1->next, l2);
            return l1;
        } else {
            l2->next = mergeTwoLists(l1, l2->next);
            return l2;
        }
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
                return new ListNode(val, next);
            });
    }

public:
    List() noexcept : first(nullptr){ }

    List(ListNode* node) noexcept : first(node) { }

    List(const std::initializer_list<int>& list) : first(construct(list)) { }

    List(const List&) = delete;
    List(List&& other) noexcept : first(std::exchange(other.first, nullptr)) { }

    ~List() {
        deleteNode(first);
    }

    const ListNode* node() const noexcept { return first; }

    ListNode* take_node() noexcept {
        return std::exchange(first, nullptr);
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
        CHECK(List{1, 2} == List(new ListNode(1, new ListNode(2))));
    }
}

TEST_CASE("Solution") {
    Solution sol;
    SUBCASE("it works") {
        List l1{1, 2, 4};
        List l2{1, 3, 4};
        List res{1, 1, 2, 3, 4, 4};
        CHECK(List(sol.mergeTwoLists(l1.take_node(), l2.take_node())) == res);
    }
}
