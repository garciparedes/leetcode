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
use std::mem;

impl Solution {
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>, 
        a: i32, 
        b: i32,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = list1.unwrap();
        
        let mut current = head.as_mut();
        for _ in 0..(a - 1) {
            current = current.next.as_mut().unwrap();
        }
        mem::swap(&mut current.next, &mut list2);
        
        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        for _ in 0..(b - a + 1) {
            list2 = list2.unwrap().next;
        }
        current.next = list2;
        
        return Some(head);
    }
}
