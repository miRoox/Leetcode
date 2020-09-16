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
    inline ListNode* join(ListNode* l1, ListNode* l2) {
        if (!l1) 
            return l2;
        ListNode* tail = l1;
        while (tail->next) tail = tail->next;
        tail->next = l2;
        return l1;
    }
public:
    // quick sort
    ListNode* sortList(ListNode* head) {
        if (!head)
            return head;
        const auto x = head->val;
        ListNode* next = head->next;
        ListNode* lhead = nullptr;
        ListNode* ltail = nullptr;
        ListNode* rhead = nullptr;
        ListNode* rtail = nullptr;
        while (next) {
            if (next->val < x) {
                if (lhead) {
                    ltail->next = next;  // append
                    ltail = ltail->next; // advance
                } else { // init
                    lhead = next;
                    ltail = next;
                }
            } else {
                if (rhead) {
                    rtail->next = next;  // append
                    rtail = rtail->next; // advance
                } else { // init
                    rhead = next;
                    rtail = next;
                }
            }
            next = next->next;
        }
        if (ltail) {
            ltail->next = nullptr;
        }
        if (rtail) {
            rtail->next = nullptr;
        }
        lhead = sortList(lhead);
        rhead = sortList(rhead);
        head->next = rhead; // prepend
        return join(lhead, head);
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
