// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    fn add_two_impl(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
        let (val1, next1) = match &l1 {
            Some(node) => (node.val,&node.next),
            None => (0,&None),
        };
        let (val2, next2) = match &l2 {
            Some(node) => (node.val,&node.next),
            None => (0,&None),
        };
        let val0 = val1+val2+carry;
        if val0 == 0 && *next1 == None && *next2 == None {
            None
        } else {
            Some(Box::new(ListNode{
                next: Solution::add_two_impl(&next1,&next2,val0/10),
                val: val0%10
            }))
        }
    }
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = Solution::add_two_impl(&l1, &l2, 0);
        if result==None {
            Some(Box::new(ListNode::new(0)))
        } else {
            result
        }
    }
}
