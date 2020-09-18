using System;
using System.Linq;

namespace add_two_numbers {
    public class ListNode : IEquatable<ListNode> {
        public int val;
        public ListNode next;
        public ListNode(int x) { val = x; }

        public bool Equals(ListNode other) { 
            return other != null 
                && val == other.val 
                && next is null ? other.next is null : next.Equals(other.next);
        }

        public override bool Equals(object obj) { 
            return obj is ListNode node && Equals(node);
        }

        public override int GetHashCode() { 
            return next is null ? int.MaxValue : (val << 1) ^ next.GetHashCode();
        }

        public static ListNode Create(params int[] args) {
            return args.Reverse().Aggregate((ListNode)null, (next, val) => new ListNode(val){next = next});
        }
    }

    public class Solution {
        private ListNode implAdd(ListNode l1, ListNode l2, int carry) {
            int v1 = getNext(ref l1);
            int v2 = getNext(ref l2);
            int v = v1 + v2 + carry;
            if (v == 0 && l1 is null && l2 is null)
                return null;
            else
                return new ListNode(v%10) { next = implAdd(l1, l2, v/10) };
        }

        private int getNext(ref ListNode l) {
            if (l is null) return 0;
            int val = l.val;
            l = l.next;
            return val;
        }

        public ListNode AddTwoNumbers(ListNode l1, ListNode l2) {
            var result = implAdd(l1, l2, 0);
            return result is null ? new ListNode(0) : result;
        }
    }
}