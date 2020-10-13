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
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match Self::split(head) {
            (None, None) => None,
            (lhs, None) => lhs,
            (None, rhs) => rhs,
            (lhs, rhs) => Self::merge(Self::sort_list(lhs), Self::sort_list(rhs)),
        }
    }
    
    fn split(
        mut head: Option<Box<ListNode>>
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let (mut lhs, mut rhs) = (None, None);
        let mut parity = true;
        
        while let Some(mut node) = head {
            head = node.next.take();
            if parity {
                node.next = lhs.take();
                lhs = Some(node);
            } else {
                node.next = rhs.take();
                rhs = Some(node);
            }
            parity = !parity;
        }
        return (lhs, rhs);
    }
    
    fn merge(lhs: Option<Box<ListNode>>, rhs: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (lhs, rhs) {
            (None, None) => None,
            (head, None) => head,
            (None, head) => head,
            (Some(mut left), Some(mut right)) => {
                let (mut small, mut large) = (left, right);
                if small.val > large.val {
                    mem::swap(&mut small, &mut large);
                }
                let sucessor = small.next.take();
                small.next = Self::merge(sucessor, Some(large));
                
                Some(small)
            } 
        }
    }
}
