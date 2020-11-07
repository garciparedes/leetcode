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
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>)
    -> Option<Box<ListNode>> {
        let mut n1 = Self::size(&l1);
        let mut n2 = Self::size(&l2);
        
        if n2 < n1 {
            mem::swap(&mut n1, &mut n2);
            mem::swap(&mut l1, &mut l2);
        }
        
        if n1 < n2 {
            for _ in n1..n2 {
                l1 = Some(Box::new(ListNode {val: 0, next: l1}));
            }
        } 

        l1 = Some(Box::new(ListNode {val: 0, next: l1}));
        l2 = Some(Box::new(ListNode {val: 0, next: l2}));
    
        if let Some(node) = Self::add(l1, l2).0 {
            if node.val == 0 {
                return node.next;
            }   
            return Some(node)
        }
        return None;
    }
    
    fn add(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        if let (Some(node1), Some(node2)) = (l1, l2) {
            let (next, carriage) = Self::add(node1.next, node2.next);
            
            let mut next_carriage = 0;
            let mut val = carriage + node1.val + node2.val;
            if val > 9 {
                val -= 10;
                next_carriage = 1;
            }
            return (Some(Box::new(ListNode {val: val, next: next})), next_carriage);
        } else {
            return (None, 0);
        }
    }
    
    fn size(l: &Option<Box<ListNode>>) -> i32 {
        if let Some(node) = l {
            return 1 + Self::size(&node.next);
        } else {
            return 0;
        }
    }
}
