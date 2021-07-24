using System;
using System.Linq;

namespace MergeTwoSortedLists {
    public class ListNode : IEquatable<ListNode> {
        public int val;
        public ListNode next;
        public ListNode(int val=0, ListNode next=null) {
            this.val = val;
            this.next = next;
        }

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
            return args.Reverse().Aggregate((ListNode)null, (next, val) => new ListNode(val, next));
        }
    }
}