pub struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(mut l1), Some(mut l2)) => {
                if l1.val < l2.val {
                    l1.next = Self::merge_two_lists(l1.next, Some(l2));
                    Some(l1)
                } else {
                    l2.next = Self::merge_two_lists(Some(l1), l2.next);
                    Some(l2)
                }
            }
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None) => None,
        }
    }
}

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
        let l1 = cons_list(&[1, 2, 4]);
        let l2 = cons_list(&[1, 3, 4]);
        let expected = cons_list(&[1, 1, 2, 3, 4, 4]);
        assert_eq!(expected, Solution::merge_two_lists(l1, l2));
    }
}
