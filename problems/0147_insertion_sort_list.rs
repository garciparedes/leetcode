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
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {   
        if head.is_none() {
            return None;
        }
        let mut i = 0;
        loop {
            i += 1;
            let mut current =  &mut head;
            for i in 0..i - 1 {
                current = &mut current.as_mut().unwrap().next;
            }
            
            let mut current_node = current.as_mut().unwrap();
            if let Some(mut next_node) = current_node.next.take() {
                current_node.next = next_node.next.take();

                if next_node.val < head.as_ref().unwrap().val {   
                    next_node.next = head;
                    head = Some(next_node);
                    continue;
                }
                
                let mut prev = &mut head;
                let mut j = 0;
                while let Some(prev_node) = prev {
                    j += 1;
                    if let Some(node) = &prev_node.next {
                        if j == i || !(node.val < next_node.val) {
                            next_node.next = prev_node.next.take();
                            prev_node.next = Some(next_node);
                            break;
                        }
                        prev = &mut prev_node.next;   
                        continue;
                    }

                    prev_node.next = Some(next_node);
                    break;
                }
                continue;
            } 
            break;
        }
        return head;

    }
}
