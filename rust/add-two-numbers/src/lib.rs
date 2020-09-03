pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

//---
impl Solution {
    fn add_two_impl(
        l1: &Option<Box<ListNode>>,
        l2: &Option<Box<ListNode>>,
        carry: i32,
    ) -> Option<Box<ListNode>> {
        let (val1, next1) = match &l1 {
            Some(node) => (node.val, &node.next),
            None => (0, &None),
        };
        let (val2, next2) = match &l2 {
            Some(node) => (node.val, &node.next),
            None => (0, &None),
        };
        let val0 = val1 + val2 + carry;
        if val0 == 0 && *next1 == None && *next2 == None {
            None
        } else {
            Some(Box::new(ListNode {
                next: Self::add_two_impl(&next1, &next2, val0 / 10),
                val: val0 % 10,
            }))
        }
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let result = Self::add_two_impl(&l1, &l2, 0);
        if result == None {
            Some(Box::new(ListNode::new(0)))
        } else {
            result
        }
    }
}
//---

#[cfg(test)]
mod tests {
    use super::*;

    fn cons_list(array: &[i32]) -> Option<Box<ListNode>> {
        // construct linked list from array slice
        array
            .iter()
            .rev()
            .fold(None, |next, &val| Some(Box::new(ListNode { next, val })))
    }

    #[test]
    fn cons_list_size0() {
        let manual = None;
        assert_eq!(cons_list(&[]), manual);
    }

    #[test]
    fn cons_list_size1() {
        let manual = Some(Box::new(ListNode::new(1)));
        assert_eq!(cons_list(&[1]), manual);
    }

    #[test]
    fn cons_list_size2() {
        let manual = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        }));
        assert_eq!(cons_list(&[1, 2]), manual);
    }

    #[test]
    fn it_works() {
        let l1 = cons_list(&[2, 4, 3]);
        let l2 = cons_list(&[5, 6, 4]);
        let result = cons_list(&[7, 0, 8]);
        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }

    #[test]
    fn zero_sum() {
        let l1 = cons_list(&[0]);
        let l2 = cons_list(&[0]);
        let result = cons_list(&[0]);
        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }

    #[test]
    fn zero_in_digit() {
        let l1 = cons_list(&[1, 6, 0, 3, 3, 6, 7, 2, 0, 1]);
        let l2 = cons_list(&[6, 3, 0, 8, 9, 6, 6, 9, 6, 1]);
        let result = cons_list(&[7, 9, 0, 1, 3, 3, 4, 2, 7, 2]);
        assert_eq!(Solution::add_two_numbers(l1, l2), result);
    }
}
